use serde::{Deserialize, Serialize};
use bcrypt::{DEFAULT_COST};
use bcrypt::BcryptError;

#[derive(Clone, Deserialize, Serialize)]
pub struct Register {
    pub username: String,
    pub password: String,
    pub age: u32,
    pub email: String
}

impl Register {
    pub fn hash(&self) -> Result<String, BcryptError> {
        let hashed = bcrypt::hash(self.password.as_str(), DEFAULT_COST)?;
        Ok(hashed)
    }
    pub fn verify_hash(&self, hash: &str) -> Result<bool, BcryptError> {
        let is_verfied_hash = bcrypt::verify(self.password.as_str(), hash)?;
        Ok(is_verfied_hash)
    }
}

#[derive(Serialize)]
pub struct RegisterReponse {
    pub username: String,
    pub age: u32,
    pub email: String
}

#[derive(Clone, Deserialize, Serialize)]
pub struct SignIn {
    pub email: String,
    pub password: String
}


#[derive(Serialize)]
pub struct SignInReponse {
    pub username: String,
    pub age: u32,
    pub email: String
}