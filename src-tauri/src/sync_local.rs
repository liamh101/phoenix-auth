use std::cmp::PartialEq;
use tauri::AppHandle;
use crate::database::{Account, SyncAccount};
use crate::{database, sync_api};
use crate::state::ServiceAccess;
use crate::sync_api::{get_record, get_single_record, Record, SyncManifest, update_record};

#[derive(PartialEq, Eq, Debug)]
enum SyncStatus {
    UpToDate,
    LocalOutOfDate,
    RemoteOutOfDate,
    RemoteMissing,
}

pub async fn sync_all_accounts(app_handle: AppHandle, sync_account: SyncAccount) {
    let authenticated_account = sync_api::authenticate_account(sync_account.clone()).await.unwrap();

    let accounts_without_external = app_handle.db(|db| database::get_accounts_without_external_id(db)).unwrap();
    let manifest_result = sync_api::get_manifest(&authenticated_account).await;

    let manifest = match manifest_result {
        Ok(manifest) => manifest,
        Err(_) => return
    };

    for account in accounts_without_external {
        if account.external_id.is_none() {
            match create_new_local_account(&app_handle, &account, &authenticated_account).await {
                Ok(_) => continue,
                Err(_) => continue,
            };
        }
    }

    let mut manifest_ids = Vec::new();

    for manifest_item in manifest {
        let potential_account = app_handle.db(|db| database::get_account_by_external_id(&manifest_item.id, &db)).unwrap();

        //Log manifest id to check what items need removing
        manifest_ids.push(manifest_item.id);

        if potential_account.is_none() {
            //Get external and create
            match copy_account_from_remote(&app_handle, &manifest_item, &sync_account).await {
                Ok(_) => continue,
                Err(_) => continue,
            };
        }

        let account = potential_account.unwrap();
        let sync_status = get_sync_status(&account, &manifest_item);


        if sync_status == SyncStatus::LocalOutOfDate {
            match update_existing_account(&app_handle, &account, &manifest_item, &sync_account).await {
                Ok(_) => continue,
                Err(_) => continue,
            }
        }

        if sync_status == SyncStatus::RemoteOutOfDate {
            match update_existing_remote_account(&app_handle, &account, &sync_account).await {
                Ok(_) => continue,
                Err(_) => continue,
            }
        }
    }

    //Remove accounts not in manifest list
    app_handle.db(|db| database::delete_accounts_without_external_ids(manifest_ids, db)).unwrap();
}

fn get_sync_status(account: &Account, sync_manifest: &SyncManifest) -> SyncStatus {
    if account.external_last_updated.is_none() {
        return SyncStatus::LocalOutOfDate
    }

    let external_last_updated = account.external_last_updated.unwrap();

    if external_last_updated < sync_manifest.updatedAt {
        return SyncStatus::LocalOutOfDate
    }

    if external_last_updated > sync_manifest.updatedAt {
        return SyncStatus::RemoteOutOfDate
    }

    return SyncStatus::UpToDate
}

async fn create_new_local_account(app_handle: &AppHandle, account: &Account, authenticated_account: &SyncAccount) -> Result<Record, String>{
    let full_account_details = app_handle.db(|db| database::get_account_details_by_id(account.id as u32, &db)).unwrap();

    let record = match get_record(&full_account_details, &authenticated_account).await {
        Ok(record) => record,
        Err(err) => return Err(err.formatted_message()),
    };
    app_handle.db(|db| database::set_remote_account(db, &account, &record)).unwrap();

    Ok(record)
}

async fn copy_account_from_remote(app_handle: &AppHandle, manifest_item: &SyncManifest, sync_account: &SyncAccount) -> Result<Account, String> {
    let new_account_record = match get_single_record(&manifest_item.id, &sync_account).await {
        Ok(record) => record,
        Err(response_error) => return Err(response_error.formatted_message())
    };
    let mut new_account_algo = "".to_string();

    if new_account_record.algorithm.is_some() {
        new_account_algo = new_account_record.algorithm.clone().unwrap().algorithm_to_string();
    }

    let new_account = app_handle.db(|db| database::create_new_account(&new_account_record.name, &new_account_record.secret, &new_account_record.otpDigits, &new_account_record.totpStep, &new_account_algo, &db)).unwrap();
    app_handle.db(|db| database::set_remote_account(db, &new_account, &new_account_record.to_record())).unwrap();

    return Ok(new_account)
}

async fn update_existing_account(app_handle: &AppHandle, account: &Account, manifest_item: &SyncManifest, sync_account: &SyncAccount) -> Result<Account, String> {
    let existing_record = match get_single_record(&manifest_item.id, &sync_account).await {
        Ok(record) => record,
        Err(response_error) => return Err(response_error.formatted_message())
    };
    let mut new_account_algo = "".to_string();

    if existing_record.algorithm.is_some() {
        new_account_algo = existing_record.algorithm.clone().unwrap().algorithm_to_string();
    }

    let updated_account = app_handle.db(|db| database::update_existing_account(&account.id, &existing_record.name, &existing_record.secret, existing_record.otpDigits, existing_record.totpStep, &new_account_algo, &db)).unwrap();
    app_handle.db(|db| database::set_remote_account(db, &account, &existing_record.to_record())).unwrap();

    Ok(updated_account)
}

async fn update_existing_remote_account(app_handle: &AppHandle, account: &Account, sync_account: &SyncAccount) -> Result<Record, String> {
    let updated_record_details = match update_record(&account, sync_account).await {
        Ok(record) => record,
        Err(response_error) => return Err(response_error.formatted_message())
    };
    app_handle.db(|db| database::set_remote_account(db, &account, &updated_record_details)).unwrap();

    Ok(updated_record_details)
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
            totp_step: 0,
            otp_digits: 0,
            algorithm: None,
            external_id: Option::from(1234),
            external_last_updated: None,
            external_hash: Option::from("HELLOWORLD".to_string()),
        };

        let manifest = SyncManifest {
            id: 1234,
            updatedAt: 1725483734,
        };

        assert_eq!(SyncStatus::LocalOutOfDate, get_sync_status(&account, &manifest));
    }

    #[test]
    fn test_local_older() {
        let account = Account {
            id: 0,
            name: "".to_string(),
            secret: "".to_string(),
            totp_step: 0,
            otp_digits: 0,
            algorithm: None,
            external_id: Option::from(1234),
            external_last_updated: Option::from(1725483730),
            external_hash: Option::from("HELLOWORLD".to_string()),
        };

        let manifest = SyncManifest {
            id: 1234,
            updatedAt: 1725483734,
        };

        assert_eq!(SyncStatus::LocalOutOfDate, get_sync_status(&account, &manifest));
    }

    #[test]
    fn test_remote_older() {
        let account = Account {
            id: 0,
            name: "".to_string(),
            secret: "".to_string(),
            totp_step: 0,
            otp_digits: 0,
            algorithm: None,
            external_id: Option::from(1234),
            external_last_updated: Option::from(1725483734),
            external_hash: Option::from("HELLOWORLD".to_string()),
        };

        let manifest = SyncManifest {
            id: 1234,
            updatedAt: 1725483730,
        };

        assert_eq!(SyncStatus::RemoteOutOfDate, get_sync_status(&account, &manifest));
    }

    #[test]
    fn test_matching_accounts() {
        let account = Account {
            id: 0,
            name: "".to_string(),
            secret: "".to_string(),
            totp_step: 0,
            otp_digits: 0,
            algorithm: None,
            external_id: Option::from(1234),
            external_last_updated: Option::from(1725483734),
            external_hash: Option::from("HELLOWORLD".to_string()),
        };

        let manifest = SyncManifest {
            id: 1234,
            updatedAt: 1725483734,
        };

        assert_eq!(SyncStatus::UpToDate, get_sync_status(&account, &manifest));
    }
}