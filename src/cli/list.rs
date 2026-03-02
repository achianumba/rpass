use std::collections::HashMap;
use std::path::PathBuf;

use clap::Args;

use crate::error::RpassError;
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
    pub fn run(&self, path_string: &String) -> Result<(), RpassError> {
        let mut store = Store::load(path_string)?;
        let mut name = String::new();
        let mut _root = PathBuf::new();

        if let Some(n) = &self.name {
            name = n.to_owned();
            _root = store.get_path(n)?;
        } else {
            _root = store.path.to_owned();
        }

        if _root.is_file() {
            let fields = store.decrypt(&format!("{}", _root.display()), &name)?;
            println!("\n{} contains the following fields\n", format!("{}", &name));

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
            let root_name = paths
                .get(&format!("{}", _root.file_name().unwrap().to_str().unwrap()))
                .unwrap();

            println!("{}", format!("{}", root_name));
        }

        store.print_tree(&mut _root, &paths, &"".to_string())?;

        Ok(())
    }
}
