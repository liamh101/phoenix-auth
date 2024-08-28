use tauri::AppHandle;
use crate::database::{Account, SyncAccount};
use crate::{database, sync_api};
use crate::state::ServiceAccess;
use crate::sync_api::{get_record, get_single_record, Record, SyncManifest};

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

        if account.external_last_updated.unwrap() < manifest_item.updatedAt {
            match update_existing_account(&app_handle, &account, &manifest_item, &sync_account).await {
                Ok(_) => continue,
                Err(_) => continue,
            }
        }
    }

    //Remove accounts not in manifest list
    app_handle.db(|db| database::delete_accounts_without_external_ids(manifest_ids, db)).unwrap();
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