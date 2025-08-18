//! Secrets store.
use eyre::{Result, WrapErr, bail};
use gpgme::{Context, Protocol};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::{File, create_dir_all, read_dir, read_to_string, write};
use std::io::{self, Write};
use std::path::PathBuf;
use toml::{from_str, to_string};
use uuid::Uuid;

/// Secrets store.
#[derive(Serialize, Deserialize, Debug)]
pub struct Store {
    /// The store's folder
    pub path: PathBuf,

    /// The store's `store.toml` file
    pub file: PathBuf,

    /// The store's paths map and key
    pub index: StoreIndex,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StoreIndex {
    /// The store's GPG public key ID
    pub key: Option<String>,

    /// The store's paths-to-uuid4 mapping.
    pub paths: HashMap<String, String>,

    /// Just a string used to distinguish entry names from store.toml when saving files etc.
    pub name: String,
}

impl Store {
    /// Create a secrets store.
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
                paths: HashMap::new(),
                name: "rpass::store::index".to_string(),
            },
            path: path.to_owned(),
            file: path.to_owned().join("store.toml"),
        })
    }

    /// Save a store's `store.toml` file.
    pub fn save_index(&self) -> Result<()> {
        self.save(self.file.to_owned(), &self.index.name, &self.index.paths)?;
        Ok(())
    }

    /// Save a secret to a file.
    pub fn save(
        &self,
        file: PathBuf,
        name: &String,
        entry: &HashMap<String, String>,
    ) -> Result<()> {
        let mut cipher: Vec<u8> = Vec::new();

        if self.index.key.is_some() && self.file == file {
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
            cipher.append(&mut vec![
                39, 10, 112, 97, 116, 104, 115, 32, 61, 32, 39, 39, 39, 10,
            ]); // '\npaths = '''\n
            cipher.append(&mut self.encrypt(name, entry)?);
            cipher.append(&mut vec![39, 39, 39]); // '''
        } else {
            cipher = self.encrypt(name, entry)?;
        }

        if &self.index.name != name {
            create_dir_all(file.parent().unwrap()).wrap_err(format!(
                "Failed to create entry directory for '{}' at '{}'",
                name,
                file.display()
            ))?;
        }

        write(file, cipher).wrap_err(format!("Failed to save data to '{}'", name))?;

        Ok(())
    }

    /// Encrypt a secret.
    pub fn encrypt(&self, name: &String, entry: &HashMap<String, String>) -> Result<Vec<u8>> {
        let mut ctx = Context::from_protocol(Protocol::OpenPgp).wrap_err(format!(
            "Failed to create encryption context for '{}.",
            name
        ))?;

        // Using match to hide potential secret info from output
        let mut _plaintext = match to_string(entry) {
            std::result::Result::Ok(t) => t,
            Err(_) => {
                bail!(format!("Failed to serialize entry for '{}'", name));
            }
        };

        // Where id == store's index file
        if name == &self.index.name {
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
                    name, key
                ))?;
        } else {
            ctx.encrypt(&vec![], _plaintext, &mut cipher)
                .wrap_err(format!(
                    "Failed to encrypt entry for {} using a symmetric key phrase.",
                    name
                ))?;
        };

        Ok(cipher)
    }

    /// Log information to stdout
    pub fn log<I: Display>(&self, info: I) {
        println!("\n\x1b[1;32mrpass\x1b[0m {}\n", info);
    }

    /// Load the index of an existing store
    pub fn load(path_string: &String) -> Result<Self> {
        let path = PathBuf::from(path_string);
        let file = path.join("store.toml");
        let file_contents = read_to_string(&file).wrap_err(format!(
            "Failed to read store index at '{}'.",
            file.display()
        ))?;

        let mut key: Option<String> = None;
        let mut _cipher: Vec<u8> = Vec::new();

        if file_contents.starts_with("-") {
            _cipher = file_contents.as_bytes().to_vec();
        } else {
            let saved_index: HashMap<String, String> =
                from_str(&file_contents).wrap_err(format!(
                    "Failed to deserialize saved store index in '{}'",
                    file.display()
                ))?;

            if !saved_index.contains_key("key") {
                bail!(
                    "'key' field missing from the store index at '{}'",
                    &file.display()
                );
            }

            key = Some(saved_index["key"].to_string());

            if !saved_index.contains_key("paths") {
                bail!(
                    "'paths' field is missing from the store index at '{}'",
                    &file.display()
                )
            }

            _cipher = file_contents.as_bytes().to_vec();
        };

        let mut ctx = Context::from_protocol(Protocol::OpenPgp)
            .wrap_err("Failed to create decryption context for store.")?;
        let mut plaintext_bytes: Vec<u8> = Vec::new();

        ctx.decrypt(&mut _cipher, &mut plaintext_bytes)
            .wrap_err("Failed to decrypt store index cipher")?;

        let plaintext = String::from_utf8(plaintext_bytes)
            .wrap_err("Failed to convert store index bytes to string.")?;

        let paths: HashMap<String, String> = match from_str(&plaintext) {
            std::result::Result::Ok(m) => m,
            Err(_) => {
                bail!(format!(
                    "Failed to deserialize store index at '{}'",
                    &file.display()
                ));
            }
        };

        let name = "rpass::index::file".to_string();

        Ok(Self {
            path,
            file,
            index: StoreIndex { key, paths, name },
        })
    }

    /// Read input from standard input
    pub fn read_user_input(&mut self, prompt: String, echo: &bool) -> Result<String> {
        if *echo {
            return self.read_and_echo_user_input(prompt);
        }

        Ok(self.read_secret_user_input(prompt)?)
    }

    /// Read input from standard input and echo each keypress as it's entered.
    fn read_and_echo_user_input(&mut self, prompt: String) -> Result<String> {
        let mut input = String::new();

        print!("\x1b[1;35m{prompt}:\x1b[0m ");
        io::stdout().flush()?; // Make sure the above prompt is shown first.

        io::stdin().read_line(&mut input)?;

        Ok(input.trim().to_string())
    }

    /// Read user input without echoing keypresses.
    fn read_secret_user_input(&self, prompt: String) -> Result<String, std::io::Error> {
        rpassword::prompt_password(format!("\x1b[1;35m{prompt}:\x1b[0m "))
    }

    /// Builds and returns an entry's ID from the entry's path
    pub fn set_entry_path(&mut self, name: &String) -> Result<PathBuf> {
        let entry_paths = name.split('/').collect::<Vec<&str>>();
        let mut file = self.path.to_owned();
        let mut store_paths = self.index.paths.to_owned();

        for pathname in &entry_paths {
            let id = self
                .index
                .paths
                .entry(pathname.to_string())
                .or_insert(Uuid::new_v4().to_string());

            file.push(&id);
            store_paths.insert(pathname.to_string(), id.to_owned());
        }

        self.index.paths = store_paths;

        Ok(file)
    }

    /// Construct an entry's actual path from the virtual name
    pub fn get_path(&self, name: &String) -> Result<PathBuf> {
        let paths = name.split('/').collect::<Vec<&str>>();
        let mut path = self.path.to_owned();

        for pathname in paths {
            match self.index.paths.get(pathname) {
                None => {
                    bail!("The store does not contain an entry named '{}'", name);
                }
                Some(p) => {
                    path.push(p);
                }
            };
        }

        if !path.is_dir() {
            path.set_extension("gpg");
        }

        Ok(path)
    }

    pub fn print_tree(
        &self,
        directory: &mut PathBuf,
        paths: &HashMap<String, String>,
        prefix: &String,
    ) -> Result<()> {
        let mut entries: Vec<PathBuf> = read_dir(&directory)
            .wrap_err(format!(
                "Failed to read actual actual path at '{}'",
                directory.display()
            ))?
            .map(|entry| entry.unwrap().path())
            .filter(|entry| !entry.ends_with("store.toml"))
            .collect();

        let mut index = paths.len();

        for entry in &mut entries {
            index -= 1;

            let mut _name = if entry.is_file() {
                entry.file_stem().unwrap().display().to_string()
            } else {
                entry.file_name().unwrap().display().to_string()
            };

            if index == 0 {
                println!("{}└── {}", prefix, paths.get(&_name).unwrap());

                if entry.is_dir() {
                    self.print_tree(
                        &mut directory.join(&_name),
                        paths,
                        &format!("{}    ", prefix),
                    )?;
                }
            } else {
                println!("{}├── {}", prefix, paths.get(&_name).unwrap());

                if entry.is_dir() {
                    self.print_tree(
                        &mut directory.join(&_name),
                        paths,
                        &format!("{}│   ", prefix),
                    )?;
                }
            }
        }

        Ok(())
    }

    pub fn decrypt(&self, file: &PathBuf, name: &String) -> Result<HashMap<String, String>> {
        let mut cipher = File::open(&file).wrap_err(format!(
            "Failed to read secret for '{}' (actual: '{}'",
            &name,
            &file.display()
        ))?;

        let mut plaintext_bytes: Vec<u8> = Vec::new();

        let mut ctx = Context::from_protocol(Protocol::OpenPgp).wrap_err(format!(
            "Failed to create encryption context for '{}.",
            name
        ))?;

        ctx.decrypt(&mut cipher, &mut plaintext_bytes)
            .wrap_err(format!("Failed to decrypt entry for '{}'", name))?;

        let plaintext = String::from_utf8(plaintext_bytes).wrap_err(format!(
            "Failed to convert cipher to text content for '{}'",
            name
        ))?;

        let saved_secret: HashMap<String, String> =
            from_str(&plaintext).wrap_err(format!("Failed to deserialize entry in '{}'", name))?;

        Ok(saved_secret)
    }
}
