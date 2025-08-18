use std::collections::HashMap;
use std::path::PathBuf;

use clap::Args;
use eyre::Result;

use crate::store::Store;

/// List secrets saved in a path or list the fields saved in an entry.
#[derive(Debug, Args)]
#[command(alias = "ls")]
pub struct List {
    /// The name/path of the secret
    #[arg()]
    pub name: Option<String>,
}

impl List {
    pub fn run(&self, path_string: &String) -> Result<()> {
        let store = Store::load(path_string)?;
        let mut name = String::new();
        let mut _root = PathBuf::new();

        if let Some(n) = &self.name {
            name = n.to_owned();
            _root = store.get_path(n)?;
        } else {
            _root = store.path.to_owned();
        }

        if _root.is_file() {
            let fields = store.decrypt(&_root, &name)?;
            println!("'{}' contains the following fields\n", &name);

            for (field, _) in fields {
                println!("- {}", field);
            }

            return Ok(());
        }

        let mut paths: HashMap<String, String> = HashMap::new();

        for (k, v) in store.index.paths.clone() {
            paths.insert(v, k);
        }

        if _root == store.path {
            println!("rPass Store")
        } else {
            println!(
                "{}",
                paths
                    .get(&_root.file_name().unwrap().display().to_string())
                    .unwrap()
            );
        }

        store.print_tree(&mut _root, &paths, &"".to_string())?;

        Ok(())
    }
}
