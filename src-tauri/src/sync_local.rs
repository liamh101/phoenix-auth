use crate::database::{Account, SyncAccount, SyncLog, SyncLogType};
use crate::state::ServiceAccess;
use crate::sync_api::{
    get_record, get_single_record, remove_record, update_record, Record, SyncManifest,
};
use crate::{database, encryption, sync_api};
use std::cmp::PartialEq;
use tauri::AppHandle;

#[derive(PartialEq, Eq, Debug)]
enum SyncStatus {
    UpToDate,
    LocalOutOfDate,
    RemoteOutOfDate,
    RemoteMissing,
}

pub async fn sync_all_accounts(app_handle: AppHandle, sync_account: SyncAccount) {
    let decrypted_sync_account = encryption::decrypt_sync_account(&encryption::get_key_directory(&app_handle), sync_account);
    let authenticated_account = match sync_api::authenticate_account(decrypted_sync_account.clone()).await {
        Ok(account) => account,
        Err(err) => {
            handle_error_log(&app_handle, err.formatted_message());
            return;
        }
    };

    let soft_deleted_accounts = app_handle.db(database::get_soft_deleted_accounts).unwrap();

    for account in soft_deleted_accounts {
        match remove_local_account(&app_handle, &account, &authenticated_account).await {
            Ok(_) => continue,
            Err(err) => {
                handle_error_log(&app_handle, err);
                continue;
            }
        }
    }

    let accounts_without_external = app_handle.db(database::get_accounts_without_external_id).unwrap();

    for account in accounts_without_external {
        if account.external_id.is_none() {
            match create_new_local_account(&app_handle, &account, &authenticated_account).await {
                Ok(_) => continue,
                Err(err) => {
                    handle_error_log(&app_handle, err);
                    return;
                }
            };
        }
    }

    let manifest_result = sync_api::get_manifest(&authenticated_account).await;

    let manifest = match manifest_result {
        Ok(manifest) => manifest,
        Err(err) => {
            handle_error_log(&app_handle, err.formatted_message());
            return;
        },
    };

    let mut manifest_ids = Vec::new();

    for manifest_item in manifest {
        let potential_account = app_handle
            .db(|db| database::get_account_by_external_id(&manifest_item.id, db))
            .unwrap();

        //Log manifest id to check what items need removing
        manifest_ids.push(manifest_item.id);

        if potential_account.is_none() {
            //Get external and create
            match copy_account_from_remote(&app_handle, &manifest_item, &authenticated_account)
                .await
            {
                Ok(_) => continue,
                Err(err) => {
                    handle_error_log(&app_handle, err);
                    continue;
                }
            };
        }

        let account = potential_account.unwrap();
        let sync_status = get_sync_status(&account, &manifest_item);

        if sync_status == SyncStatus::LocalOutOfDate {
            match update_existing_account(
                &app_handle,
                &account,
                &manifest_item,
                &authenticated_account,
            )
            .await
            {
                Ok(_) => continue,
                Err(err) => {
                    handle_error_log(&app_handle, err);
                    continue;
                }
            }
        }

        if sync_status == SyncStatus::RemoteOutOfDate {
            match update_existing_remote_account(&app_handle, &account, &authenticated_account)
                .await
            {
                Ok(_) => continue,
                Err(err) => {
                    handle_error_log(&app_handle, err);
                    continue;
                }
            }
        }
    }

    //Remove accounts not in manifest list
    app_handle
        .db(|db| database::delete_accounts_without_external_ids(manifest_ids, db))
        .unwrap();
}

fn get_sync_status(account: &Account, sync_manifest: &SyncManifest) -> SyncStatus {
    if account.external_last_updated.is_none() {
        return SyncStatus::LocalOutOfDate;
    }

    let external_last_updated = account.external_last_updated.unwrap();

    if external_last_updated < sync_manifest.updated_at {
        return SyncStatus::LocalOutOfDate;
    }

    if external_last_updated > sync_manifest.updated_at {
        return SyncStatus::RemoteOutOfDate;
    }

    SyncStatus::UpToDate
}

async fn remove_local_account(
    app_handle: &AppHandle,
    account: &Account,
    authenticated_account: &SyncAccount,
) -> Result<bool, String> {
    if account.external_id.is_some() {
        match remove_record(&account.external_id.unwrap(), &authenticated_account).await {
            Err(err) => return Err(err.formatted_message()),
            _ => {}
        }
    }

    Ok(app_handle
        .db(|db| database::delete_account(account, db))
        .unwrap())
}

async fn create_new_local_account(
    app_handle: &AppHandle,
    account: &Account,
    authenticated_account: &SyncAccount,
) -> Result<Record, String> {
    let full_account_details = app_handle
        .db(|db| database::get_account_details_by_id(account.id as u32, db))
        .unwrap();
    let decrypted_account = encryption::decrypt_account(&encryption::get_key_directory(app_handle), &full_account_details);

    let record = match get_record(&decrypted_account, authenticated_account).await {
        Ok(record) => record,
        Err(err) => return Err(err.formatted_message()),
    };
    app_handle
        .db(|db| database::set_remote_account(db, &decrypted_account, &record))
        .unwrap();

    Ok(record)
}

async fn copy_account_from_remote(
    app_handle: &AppHandle,
    manifest_item: &SyncManifest,
    sync_account: &SyncAccount,
) -> Result<Account, String> {
    let new_account_record = match get_single_record(&manifest_item.id, sync_account).await {
        Ok(record) => record,
        Err(response_error) => return Err(response_error.formatted_message()),
    };
    let mut new_account_algo = "".to_string();

    if new_account_record.algorithm.is_some() {
        new_account_algo = new_account_record
            .algorithm
            .clone()
            .unwrap()
            .algorithm_to_string();
    }

    let new_account = app_handle
        .db(|db| {
            database::create_new_account(
                &new_account_record.name,
                &encryption::encrypt(&encryption::get_key_directory(app_handle), &new_account_record.secret).unwrap(),
                &new_account_record.otp_digits,
                &new_account_record.totp_step,
                &new_account_record.colour,
                &new_account_algo,
                db,
            )
        })
        .unwrap();
    app_handle
        .db(|db| database::set_remote_account(db, &new_account, &new_account_record.to_record()))
        .unwrap();

    Ok(new_account)
}

async fn update_existing_account(
    app_handle: &AppHandle,
    account: &Account,
    manifest_item: &SyncManifest,
    sync_account: &SyncAccount,
) -> Result<Account, String> {
    let existing_record = match get_single_record(&manifest_item.id, sync_account).await {
        Ok(record) => record,
        Err(response_error) => return Err(response_error.formatted_message()),
    };
    let mut new_account_algo = "".to_string();

    if existing_record.algorithm.is_some() {
        new_account_algo = existing_record
            .algorithm
            .clone()
            .unwrap()
            .algorithm_to_string();
    }

    let updated_account = app_handle
        .db(|db| {
            database::update_existing_account(
                &account.id,
                &existing_record.name,
                &encryption::encrypt(&encryption::get_key_directory(app_handle), &existing_record.secret).unwrap(),
                existing_record.otp_digits,
                existing_record.totp_step,
                "5c636a", // While Server has not been updated
                &new_account_algo,
                db,
            )
        })
        .unwrap();

    app_handle
        .db(|db| database::set_remote_account(db, account, &existing_record.to_record()))
        .unwrap();

    Ok(updated_account)
}

async fn update_existing_remote_account(
    app_handle: &AppHandle,
    account: &Account,
    sync_account: &SyncAccount,
) -> Result<Record, String> {
    let decrypted_record = encryption::decrypt_account(&encryption::get_key_directory(app_handle), account);
    let updated_record_details = match update_record(&decrypted_record, sync_account).await {
        Ok(record) => record,
        Err(response_error) => return Err(response_error.formatted_message()),
    };

    app_handle
        .db(|db| database::set_remote_account(db, account, &updated_record_details))
        .unwrap();

    Ok(updated_record_details)
}

fn handle_error_log(app_handle: &AppHandle, log: String) -> SyncLog {
    app_handle
        .db(|db| database::create_sync_log(db, log, SyncLogType::ERROR))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::database::Account;
    use crate::sync_api::SyncManifest;
    use crate::sync_local::{get_sync_status, SyncStatus};

    #[test]
    fn test_external_date_missing() {
        let account = Account {
            id: 0,
            name: "".to_string(),
            secret: "".to_string(),
            colour: "".to_string(),
            totp_step: 0,
            otp_digits: 0,
            algorithm: None,
            external_id: Option::from(1234),
            external_last_updated: None,
            external_hash: Option::from("HELLOWORLD".to_string()),
            deleted_at: None,
        };

        let manifest = SyncManifest {
            id: 1234,
            updated_at: 1725483734,
        };

        assert_eq!(
            SyncStatus::LocalOutOfDate,
            get_sync_status(&account, &manifest)
        );
    }

    #[test]
    fn test_local_older() {
        let account = Account {
            id: 0,
            name: "".to_string(),
            secret: "".to_string(),
            colour: "".to_string(),
            totp_step: 0,
            otp_digits: 0,
            algorithm: None,
            external_id: Option::from(1234),
            external_last_updated: Option::from(1725483730),
            external_hash: Option::from("HELLOWORLD".to_string()),
            deleted_at: None,
        };

        let manifest = SyncManifest {
            id: 1234,
            updated_at: 1725483734,
        };

        assert_eq!(
            SyncStatus::LocalOutOfDate,
            get_sync_status(&account, &manifest)
        );
    }

    #[test]
    fn test_remote_older() {
        let account = Account {
            id: 0,
            name: "".to_string(),
            secret: "".to_string(),
            colour: "".to_string(),
            totp_step: 0,
            otp_digits: 0,
            algorithm: None,
            external_id: Option::from(1234),
            external_last_updated: Option::from(1725483734),
            external_hash: Option::from("HELLOWORLD".to_string()),
            deleted_at: None,
        };

        let manifest = SyncManifest {
            id: 1234,
            updated_at: 1725483730,
        };

        assert_eq!(
            SyncStatus::RemoteOutOfDate,
            get_sync_status(&account, &manifest)
        );
    }

    #[test]
    fn test_matching_accounts() {
        let account = Account {
            id: 0,
            name: "".to_string(),
            secret: "".to_string(),
            colour: "".to_string(),
            totp_step: 0,
            otp_digits: 0,
            algorithm: None,
            external_id: Option::from(1234),
            external_last_updated: Option::from(1725483734),
            external_hash: Option::from("HELLOWORLD".to_string()),
            deleted_at: None,
        };

        let manifest = SyncManifest {
            id: 1234,
            updated_at: 1725483734,
        };

        assert_eq!(SyncStatus::UpToDate, get_sync_status(&account, &manifest));
    }
}
