use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
    nbf: usize,
}

const SECRET_KEY: &[u8] = b"supersecretkey";

// Vulnerable function to create a JWT token (no expiration, no not-before validation)
pub fn create_jwt(username: &str) -> String {
    let claims = Claims {
        sub: username.to_string(),
        company: "RustyCrate".to_string(),
        exp: 10000000000,
        nbf: 0,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY)).unwrap()
}

// JWT decoding without signature verification
pub fn validate_jwt(token: &str) -> bool {
    decode::<Claims>(token, &DecodingKey::from_secret(SECRET_KEY), &Validation::default()).is_ok()
}

// JWT validation without checking expiration
pub fn validate_jwt_without_exp(token: &str) -> bool {
    let mut validation = Validation::default();
    validation.validate_exp = false;
    decode::<Claims>(token, &DecodingKey::from_secret(SECRET_KEY), &validation).is_ok()
}
