use std::{fs, string};
use std::path::Path;
use easy_tree::Tree;
// let mut file_tree: Tree<String> = Tree::new();

pub fn init() -> std::io::Result<()> {
    let path = Path::new("./media/");
    if !path.exists() {
        fs::create_dir(&path)?;
        println!("Initialized media directory");
    }
    Ok(())
}

