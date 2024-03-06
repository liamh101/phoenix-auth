use totp_rs::{Algorithm, Secret, TOTP};

const TOTP_STEP: u64 = 30;
const OTP_DIGITS: usize = 6;
const SKEW: u8 = 1;

pub fn is_valid_secret(secret: String) -> bool {
    match TOTP::new(
        Algorithm::SHA1,
        OTP_DIGITS,
        SKEW,
        TOTP_STEP,
        Secret::Raw(secret.as_bytes().to_vec()).to_bytes().unwrap(),
    ) {
        Ok(_) => true,
        Err(..) => false,
    }
}

pub fn get_current_token(secret: String) -> String {
    let totp = TOTP::new(
        Algorithm::SHA512,
        OTP_DIGITS,
        SKEW,
        TOTP_STEP,
        Secret::Raw(secret.as_bytes().to_vec()).to_bytes().unwrap(),
    ).unwrap();

    totp.generate_current().unwrap()
}
