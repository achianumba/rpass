//! Secrets store.
use eyre::{bail, Result, WrapErr};
use gpgme::{Context, Protocol};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{create_dir_all, write};
use std::path::PathBuf;
use toml::to_string;

/// Secrets store.
#[derive(Serialize, Deserialize, Debug)]
pub struct Store {
    pub path: PathBuf,
    pub file: PathBuf,
    pub index: StoreIndex,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoreIndex {
    pub key: Option<String>,
    pub map: HashMap<String, String>,
}

impl Store {
    pub fn init(key: Option<String>, path: PathBuf) -> Result<Self> {
        if !path.exists() {
            create_dir_all(&path).wrap_err(format!(
                "Failed to create password store folder '{}'",
                &path.display()
            ))?;
        } else {
            bail!(
                "Aborting password store initialization. '{}' already exists.",
                &path.display()
            );
        };

        Ok(Self {
            index: StoreIndex {
                key,
                map: HashMap::new(),
            },
            path: path.to_owned(),
            file: path.to_owned().join("store.toml"),
            id: "rpass::store::index".to_string(),
        })
    }

    pub fn save_index(&self) -> Result<()> {
        self.save(&self.file, &self.id, &self.index.map)?;
        Ok(())
    }

    pub fn save(&self, file: &PathBuf, id: &String, entry: &HashMap<String, String>) -> Result<()> {
        let mut cipher: Vec<u8> = Vec::new();

        if self.index.key.is_some() && &self.file == file {
            cipher.append(&mut vec![107, 101, 121, 32, 61, 32, 39]); // key = '
            cipher.append(
                &mut self
                    .index
                    .key
                    .to_owned()
                    .unwrap()
                    .to_string()
                    .as_bytes()
                    .to_vec(),
            );
            cipher.append(&mut vec![39, 10, 109, 97, 112, 32, 61, 32, 39, 39, 39, 10]); // '\nmap = '''\n
            cipher.append(&mut self.encrypt(id, entry)?);
            cipher.append(&mut vec![39, 39, 39]); // '''
        } else {
            cipher = self.encrypt(id, entry)?;
        }

        write(file, cipher).wrap_err(format!("Failed to save data to '{}'", id))?;

        Ok(())
    }

    pub fn encrypt(&self, id: &String, entry: &HashMap<String, String>) -> Result<Vec<u8>> {
        let mut ctx = Context::from_protocol(Protocol::OpenPgp)
            .wrap_err(format!("Failed to create encryption context for '{}.", id))?;

        // Using match to hide potential secret info from output
        let mut _plaintext = match to_string(entry) {
            std::result::Result::Ok(t) => t,
            Err(_) => {
                bail!(format!("Failed to store serialized entry for '{}'", id));
            }
        };

        // Where id == store's index file
        if id == &self.id {
            ctx.set_armor(true);
        } else {
            ctx.set_armor(false);
        }

        let mut cipher: Vec<u8> = Vec::new();

        // Doesn't require a random symmetric key for the session
        // since an array of Key isn't passed to ctx.encrypt
        if let Some(key) = &self.index.key {
            let public_key = ctx
                .get_key(key)
                .wrap_err(format!("Failed to retrieve '{}' from keystore.", key))?;

            ctx.encrypt(Some(&public_key), _plaintext, &mut cipher)
                .wrap_err(format!(
                    "Failed to encrypt entry for {} using key with ID {}",
                    id, key
                ))?;
        } else {
            ctx.encrypt(&vec![], _plaintext, &mut cipher)
                .wrap_err(format!(
                    "Failed to encrypt entry for {} using a symmetric key phrase.",
                    id
                ))?;
        };

        Ok(cipher)
    }
}
