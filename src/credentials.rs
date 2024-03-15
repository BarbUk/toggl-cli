use crate::error;
use crate::models;
use async_trait::async_trait;
use error::StorageError;
use keyring::Entry;
#[cfg(test)]
use mockall::automock;
use models::ResultWithDefaultError;

pub struct Credentials {
    pub api_token: String,
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait CredentialsStorage {
    fn read(&self) -> ResultWithDefaultError<Credentials>;
    fn persist(&self, api_token: String) -> ResultWithDefaultError<()>;
}

pub struct KeyringStorage {
    keyring: Entry,
}

impl KeyringStorage {
    pub fn new(keyring: Entry) -> KeyringStorage {
        Self { keyring }
    }
}

impl CredentialsStorage for KeyringStorage {
    fn read(&self) -> ResultWithDefaultError<Credentials> {
        let api_token = self
            .keyring
            .get_password()
            .map_err(|err| {
                eprintln!("Error reading keyring: {}", err);
                return Box::new(StorageError::Read);
            })
            .unwrap();
        Ok(Credentials { api_token })
    }

    fn persist(&self, api_token: String) -> ResultWithDefaultError<()> {
        match self.keyring.set_password(api_token.as_str()) {
            Err(err) => {
                eprintln!("Error writing to keyring: {}", err);
                return Err(Box::new(StorageError::Write));
            }
            Ok(_) => Ok(()),
        }
    }
}
