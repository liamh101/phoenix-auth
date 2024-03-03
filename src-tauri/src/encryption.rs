use std::str;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};

const KEY: &str = "CHANGEME";

pub fn encrypt(original: &str) -> String {
    let mc = new_magic_crypt!(KEY, 256);
    
    mc.encrypt_str_to_base64(original)
}

pub fn decrypt(encrypted: &str) -> String {
    let mc = new_magic_crypt!(KEY, 256);

    mc.decrypt_base64_to_string(encrypted).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::encryption::{decrypt, encrypt};

    #[test]
    fn can_encrypt_and_decrypt() {
        let original = "hello world";
        let encrypted = encrypt(original);
        let decrypted = decrypt(&encrypted);

        assert_ne!(encrypted, "hello world");
        assert_eq!(decrypted, "hello world");
    }
}