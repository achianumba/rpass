//! Secrets store.
use miette::{Result, bail, miette};
use std::collections::HashMap;
use std::fs::{read_dir, read_to_string, write};
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use toml::{from_str, to_string};
use uuid::Uuid;

use crate::{blue, purple, red};

/// Secrets store.
#[derive(Debug)]
pub struct Store {
    /// The store's folder
    pub path: PathBuf,

    /// The store's `store.toml` file
    pub file: PathBuf,

    /// The store's paths map and key
    pub index: StoreIndex,
}

#[derive(Debug)]
pub struct StoreIndex {
    /// The store's GPG public key ID
    pub key: Option<String>,

    /// The store's paths-to-uuid4 mapping.
    pub paths: HashMap<String, String>,

    /// Just a string used to distinguish entry names from store.toml when saving files etc.
    pub name: String,
}

impl Store {
    /// Save a store's `store.toml` file.
    pub fn save_index(&self) -> Result<()> {
        let mut index_bytes: Vec<u8> = Vec::new();

        if let Some(key) = &self.index.key {
            index_bytes.append(&mut vec![107, 101, 121, 32, 61, 32, 39]); // key = '
            index_bytes.append(&mut key.to_string().as_bytes().to_vec());
        }

        index_bytes.append(&mut vec![
            39, 10, 112, 97, 116, 104, 115, 32, 61, 32, 39, 39, 39, 10,
        ]); // '\npaths = '''\n

        let mut paths = self
            .encrypt(
                self.file.display().to_string(),
                &self.index.name,
                &self.index.paths,
            )?
            .unwrap();

        index_bytes.append(&mut paths);
        index_bytes.append(&mut vec![39, 39, 39]); // '''

        write(&self.file, index_bytes).map_err(|e| {
            miette!(
                "{}. {}",
                red!("Failed to save store index in {}", self.file.display()),
                e.to_string()
            )
        })?;
        Ok(())
    }

    /// Encrypt a secret.
    pub fn encrypt(
        &self,
        outfile: String,
        name: &String,
        entry: &HashMap<String, String>,
    ) -> Result<Option<Vec<u8>>> {
        // Using match to hide potential secret info from output
        let mut _plaintext = match to_string(entry) {
            std::result::Result::Ok(t) => t,
            Err(_) => {
                bail!(red!("Failed to serialize entry for '{}'", name));
            }
        };

        let mut args: Vec<&str> = vec!["-q", "--batch", "--yes"];

        if name == &self.index.name {
            args.push("-a");
        } else {
            args.push("-o");
            args.push(outfile.as_str());
        }

        if let Some(key) = &self.index.key {
            args.push("-r");
            args.push(key.as_str());
            args.push("-e");
        } else {
            args.push("--symmetric");
        };

        let mut child_process = Command::new("gpg")
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| {
                miette!(
                    "{}. {}",
                    red!("Failed to spawn a child process for 'git' shell command"),
                    e.to_string()
                )
            })?;

        if let Some(mut stdin) = child_process.stdin.take() {
            stdin.write_all(_plaintext.as_bytes()).map_err(|e| {
                miette!(
                    "{}. {}",
                    red!("Failed encrypt secrets {}", name),
                    e.to_string()
                )
            })?;
        }

        let output = child_process.wait_with_output().map_err(|e| {
            miette!(
                "{}. {}",
                red!("Failed encrypt entry {}", name),
                e.to_string()
            )
        })?;

        if output.status.success() {
            if outfile != self.file.display().to_string() {
                println!("Saved {}", blue!("{}", name));
            }
        } else {
            bail!(
                "{}\n{}",
                red!("Failed encrypt {}", name),
                String::from_utf8(output.stderr).map_err(|e| miette!("{}", e.to_string()))?
            );
        }

        if outfile == self.file.display().to_string() {
            return Ok(Some(output.stdout));
        }

        Ok(None)
    }

    /// Load the index of an existing store
    pub fn load(path_string: &String) -> Result<Self> {
        let mut store = Self {
            index: StoreIndex {
                key: None,
                paths: HashMap::new(),
                name: "rpass::store::index".to_string(),
            },
            path: PathBuf::from(path_string),
            file: PathBuf::from(path_string).join("store.toml"),
        };

        let file = PathBuf::from(path_string)
            .join("store.toml")
            .display()
            .to_string();

        let paths = store.decrypt(&file, &"rpass::store::index".to_string())?;

        store.index.paths = paths;

        Ok(store)
    }

    pub fn decrypt(&mut self, file: &String, name: &String) -> Result<HashMap<String, String>> {
        let mut args = vec!["-d", "-q", "--batch", "--yes"];

        if name != &self.index.name {
            args.push(file);
        }

        let mut child_process = Command::new("gpg")
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| {
                miette!(
                    "{}. {}",
                    red!("Failed to run 'gpg' command."),
                    e.to_string()
                )
            })?;

        if let Some(mut stdin) = child_process.stdin.take() {
            if name == &self.index.name {
                let (mut index_cipher, key) = Store::read_index(file)?;

                self.index.key = key;

                stdin.write_all(&mut index_cipher).map_err(|e| {
                    miette!(
                        "{}. {}",
                        red!("Failed to pipe store index to gpg"),
                        e.to_string()
                    )
                })?;
            }
        }

        let output = child_process.wait_with_output().map_err(|e| {
            let msg = if name == &self.index.name {
                red!("Failed to decrypt store index ad {}", &self.file.display())
            } else {
                red!("Failed to decrypt entry for {}", name)
            };

            miette!("{}. {}", msg, e.to_string())
        })?;

        let mut _map: HashMap<String, String> = HashMap::new();

        if output.status.success() {
            let plaintext = String::from_utf8(output.stdout).map_err(|e| {
                miette!(
                    "{}. {}",
                    red!(
                        "Failed to serialize decrypted secret {}",
                        self.file.display()
                    ),
                    e.to_string()
                )
            })?;

            _map = match from_str(&plaintext) {
                std::result::Result::Ok(m) => m,
                Err(_) => {
                    bail!(red!("Failed to deserialize store index at '{}'", name));
                }
            };
        } else {
            bail!(
                "{}\n{}",
                red!("Failed to decrypt to decrypt entry for {}", name),
                String::from_utf8(output.stderr).map_err(|e| miette!("{}", e.to_string()))?
            );
        }

        Ok(_map)
    }

    fn read_index(path_string: &String) -> Result<(Vec<u8>, Option<String>)> {
        let file_contents = read_to_string(path_string).map_err(|e| {
            miette!(
                "{}. {}",
                red!("Failed to read store index at '{path_string}'."),
                e.to_string()
            )
        })?;

        let mut _paths_cipher: Vec<u8> = Vec::new();
        let mut key: Option<String> = None;

        if file_contents.starts_with("-") {
            _paths_cipher = file_contents.as_bytes().to_vec();
        } else {
            let saved_index: HashMap<String, String> = from_str(&file_contents).map_err(|e| {
                miette!(
                    "{}. {}",
                    red!(
                        "Failed to deserialize saved store index in '{}'",
                        path_string
                    ),
                    e.to_string()
                )
            })?;

            match saved_index.get("key") {
                Some(k) => {
                    key = Some(k.to_owned());
                }
                None => {
                    bail!(red!(
                        "'key' field missing from the store index at '{}'",
                        path_string
                    ));
                }
            }

            match saved_index.get("paths") {
                Some(p) => {
                    _paths_cipher = p.as_bytes().to_vec();
                }
                None => {
                    bail!(red!(
                        "'paths' field is missing from the store index at '{}'",
                        path_string
                    ));
                }
            }
        };

        Ok((_paths_cipher, key))
    }

    /// Read input from standard input
    pub fn read_user_input(&mut self, prompt: String, echo: &bool) -> Result<String> {
        let prompt = purple!("{}: ", prompt);

        if *echo {
            return self.read_and_echo_user_input(prompt);
        }

        Ok(self.read_secret_user_input(prompt).map_err(|e| {
            miette!(
                "{}. {}",
                red!("Failed to read secret user input"),
                e.to_string()
            )
        })?)
    }

    /// Read input from standard input and echo each keypress as it's entered.
    pub fn read_and_echo_user_input(&mut self, prompt: String) -> Result<String> {
        let mut input = String::new();

        print!("{}", prompt);
        // Makes sure the above prompt is shown first.
        io::stdout()
            .flush()
            .map_err(|e| miette!("{}. {}", red!("Failed to flush stdout"), e.to_string()))?;

        io::stdin()
            .read_line(&mut input)
            .map_err(|e| miette!("{}. {}", red!("Failed to read user input"), e.to_string()))?;

        Ok(input.trim().to_string())
    }

    /// Read user input without echoing keypresses.
    fn read_secret_user_input(&self, prompt: String) -> Result<String, std::io::Error> {
        rpassword::prompt_password(prompt)
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
                    bail!(red!("The store does not contain an entry named '{}'", name));
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
            .map_err(|e| {
                miette!(
                    "{}. {}",
                    red!(
                        "Failed to read actual actual path at '{}'",
                        directory.display()
                    ),
                    e.to_string()
                )
            })?
            .map(|entry| entry.unwrap().path())
            .filter(|entry| !entry.ends_with("store.toml") && !entry.ends_with(".git"))
            .collect();

        let mut index = entries.len();

        for entry in &mut entries {
            index -= 1;

            let id = if entry.is_file() {
                entry.file_stem().unwrap().display().to_string()
            } else {
                entry.file_name().unwrap().display().to_string()
            };

            let name = &paths.get(&id).unwrap();

            if index == 0 {
                if entry.is_dir() {
                    println!("{}└── {}", prefix, blue!("{}", name));

                    self.print_tree(
                        &mut directory.join(&entry),
                        &paths,
                        &format!("{}    ", prefix),
                    )?;
                } else {
                    println!("{}└── {}", prefix, name);
                }
            }

            if index != 0 {
                if entry.is_dir() {
                    println!("{}├── {}", prefix, blue!("{}", name));

                    self.print_tree(
                        &mut directory.join(&entry),
                        &paths,
                        &format!("{}│   ", prefix),
                    )?;
                } else {
                    println!("{}├── {}", prefix, name);
                }
            }
        }

        Ok(())
    }

    pub fn is_repo(&self) -> bool {
        self.path.join(".git").is_dir()
    }
}
