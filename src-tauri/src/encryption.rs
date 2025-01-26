use std::path::PathBuf;
use dotenv_codegen::dotenv;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use std::{fs, str};
use base64::Engine;
use base64::engine::general_purpose;
use chacha20poly1305::{aead::{Aead, AeadCore, KeyInit, OsRng}, ChaCha20Poly1305, Key, Nonce};
use chacha20poly1305::aead::generic_array::GenericArray;
use crate::database::{Account, SyncAccount};
use tauri::{AppHandle, Manager};


const KEY: &str = dotenv!("ENCRYPTION_KEY");
const KEY_FILE_NAME: &str = "private.key";

pub fn encrypt(key_location_path: PathBuf, original: &str) -> Result<String, String> {
    let key = get_key(key_location_path);

    let cipher = ChaCha20Poly1305::new(&key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let mut enc_data = cipher.encrypt(&nonce, original.as_ref()).unwrap();

    enc_data.splice(0..0, nonce.to_vec());

    Ok(general_purpose::STANDARD.encode(enc_data))
}

pub fn decrypt(key_location_path: PathBuf, encrypted: &str) -> Result<String, String> {
    let key = get_key(key_location_path);

    let cipher = ChaCha20Poly1305::new(&key);
    let mut encrypt_bytes = general_purpose::STANDARD.decode(encrypted).unwrap();
    let nonce = GenericArray::clone_from_slice(&encrypt_bytes[0..12]) as Nonce;
    encrypt_bytes.splice(0..12, []);

    let plaintext = cipher.decrypt(&nonce, encrypt_bytes.as_ref()).unwrap();

    Ok(String::from_utf8(plaintext).unwrap())
}

pub fn decrypt_account(key_location: PathBuf, account: &Account) -> Account
{
    let secret = decrypt(key_location, &account.secret).unwrap();

    return Account {
        id: account.id.clone(),
        name: account.name.clone(),
        secret,
        totp_step: account.totp_step.clone(),
        otp_digits: account.otp_digits.clone(),
        algorithm: account.algorithm.clone(),
        external_id: account.external_id.clone(),
        external_last_updated: account.external_last_updated.clone(),
        external_hash: account.external_hash.clone(),
        deleted_at: account.deleted_at.clone(),
    }
}

pub fn decrypt_sync_account(key_location: PathBuf, account: SyncAccount) -> SyncAccount {
    let password = decrypt(key_location, &account.password).unwrap();

    return SyncAccount {
        id: account.id.clone(),
        username: account.username.clone(),
        password,
        url: account.url.clone(),
        token: account.token.clone(),
    }
}

pub fn get_key_directory(handle: &AppHandle) -> PathBuf {
    handle.path().app_data_dir().expect("The App data directory should exist")
}

pub fn legacy_encrypt(original: &str) -> String {
    let mc = new_magic_crypt!(KEY, 256);

    mc.encrypt_str_to_base64(original)
}

pub fn legacy_decrypt(encrypted: &str) -> Result<String, ()> {
    let mc = new_magic_crypt!(KEY, 256);
    let result = mc.decrypt_base64_to_string(encrypted);

    if result.is_err() {
        return Err(());
    }

    return Ok(result.unwrap())
}


fn get_key(base_path: PathBuf) -> Key {
    fs::create_dir_all(&base_path).expect("The app data directory should be created.");
    let key_path = base_path.join(KEY_FILE_NAME);
    let file_exists = fs::exists(&key_path).expect("Could not read key directory");

    match file_exists {
        true => GenericArray::clone_from_slice(&fs::read(&key_path).unwrap()[0..]) as Key,
        false => create_key(&key_path)
    }
}

fn create_key(key_path: &PathBuf) -> Key {
    let key = ChaCha20Poly1305::generate_key(&mut OsRng);

    fs::write(key_path, key).expect("Could not create Key file");

    key
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::database::{Account, AccountAlgorithm, SyncAccount};
    use crate::encryption::{decrypt, decrypt_account, decrypt_sync_account, encrypt, legacy_decrypt, legacy_encrypt};

    #[test]
    fn can_encrypt_and_decrypt_existing_key() {
        let path = PathBuf::from("./bin");
        let original = "hello world";
        let encrypted = encrypt(path.clone(), original).unwrap();
        let decrypted = decrypt(path.clone(), &encrypted).unwrap();

        assert_ne!(encrypted, "hello world");
        assert_eq!(decrypted, "hello world");

        let predefined_encrypted = "IJAJctNE9bichzwx5YtpKuU62ncethJ0p9HLymqueV1sdQEzfFb5";
        let predefined_decrypted = decrypt(path, &encrypted).unwrap();

        assert_eq!(predefined_decrypted, "hello world");
    }

    #[test]
    fn can_encrypt_and_decrypt_missing_key() {
        let path = PathBuf::from("./bin/blank");
        let original = "hello world";
        let encrypted = encrypt(path.clone(), original).unwrap();
        let decrypted = decrypt(path, &encrypted).unwrap();

        assert_ne!(encrypted, "hello world");
        assert_eq!(decrypted, "hello world");
    }

    #[test]
    fn can_decrypt_account() {
        let path = PathBuf::from("./bin");
        let secret = encrypt(path.clone(), "hello world").unwrap();

        let account = Account {
            id: 1,
            name: "This is a test".to_string(),
            secret,
            totp_step: 30,
            otp_digits: 8,
            algorithm: Option::from(AccountAlgorithm::SHA512),
            external_id: Option::from(2),
            external_last_updated: Option::from(2003),
            external_hash: Option::from("HelloWorld".to_string()),
            deleted_at: Option::from(23),
        };

        let decrypted_account = decrypt_account(path.clone(), &account);

        assert_eq!(decrypted_account.id, 1);
        assert_eq!(decrypted_account.name, "This is a test".to_string());
        assert_eq!(decrypted_account.secret, "hello world".to_string());
        assert_eq!(decrypted_account.totp_step, 30);
        assert_eq!(decrypted_account.otp_digits, 8);
        assert_eq!(decrypted_account.algorithm, Option::from(AccountAlgorithm::SHA512));
        assert_eq!(decrypted_account.external_id, Option::from(2));
        assert_eq!(decrypted_account.external_last_updated, Option::from(2003));
        assert_eq!(decrypted_account.external_hash, Option::from("HelloWorld".to_string()));
        assert_eq!(decrypted_account.deleted_at, Option::from(23));
    }

    #[test]
    fn can_decrypt_sync_account() {
        let path = PathBuf::from("./bin");
        let password = encrypt(path.clone(), "hello world").unwrap();
        
        let sync_account = SyncAccount {
            id: 1,
            username: "username".to_string(),
            password,
            url: "https://test.com".to_string(),
            token: Option::from("token".to_string()),
        };

        let decrypted_account = decrypt_sync_account(path.clone(), sync_account);

        assert_eq!(decrypted_account.id, 1);
        assert_eq!(decrypted_account.username, "username".to_string());
        assert_eq!(decrypted_account.password, "hello world".to_string());
        assert_eq!(decrypted_account.url, "https://test.com".to_string());
        assert_eq!(decrypted_account.token, Option::from("token".to_string()));
    }

    #[test]
    fn can_legacy_encrypt_and_decrypt() {
        let original = "hello world";
        let encrypted = legacy_encrypt(original);
        let decrypted = legacy_decrypt(&encrypted);

        assert_ne!(encrypted, "hello world");
        assert_eq!(decrypted.unwrap(), "hello world");
    }
}
