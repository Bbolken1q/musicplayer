use std::fmt::Display;
use std::fs;
use std::path::Path;
use ego_tree::*;
// let mut file_tree: Tree<String> = Tree::new();

#[derive(Clone)]
// #[derive()]
pub struct DirectoryEntry {
    path: String,
    is_dir: bool,
}

impl Display for DirectoryEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = write!(f, "({}{})", self.path, if self.is_dir { " | Directory" } else { "" });
        Ok(())
    }
}

pub fn init() -> std::io::Result<()> {
    let path = Path::new("./media/");
    if !path.exists() {
        fs::create_dir(&path)?;
        println!("Initialized media directory");
    } else {
        println!("Found media directory");
    }
    Ok(())
}

pub fn get_file_tree(str: &str) -> Tree<DirectoryEntry> {
    let mut fs_tree = Tree::new(DirectoryEntry { path: str.to_string(), is_dir: true });
    let mut root = fs_tree.root_mut();

    let paths = fs::read_dir(str).unwrap();
    for path in paths {
        let path = path.unwrap();
        if path.path().is_dir() {
            let foreign_tree = get_file_tree(&path.path().to_string_lossy().to_string());
            append_subtree(&mut root, foreign_tree.root());
        } else {
            root.append(DirectoryEntry { path: path.path().to_string_lossy().to_string(), is_dir: false});
        }
    }

    return fs_tree;
}

fn append_subtree<T: Clone>(parent: &mut NodeMut<T>, subtree_root: NodeRef<T>) {
    let mut new_node = parent.append(subtree_root.value().clone());
    
    for child in subtree_root.children() {
        append_subtree(&mut new_node, child);
    }
}

pub fn print_tree_fancy<T: std::fmt::Display>(tree: &Tree<T>) {
    let root = tree.root();
    println!("{}", root.value());
    
    let children: Vec<_> = root.children().collect();
    for (i, child) in children.iter().enumerate() {
        let is_last = i == children.len() - 1;
        print_node_fancy(*child, "", is_last);
    }
}

fn print_node_fancy<T: std::fmt::Display>(
    node: ego_tree::NodeRef<T>, 
    prefix: &str, 
    is_last: bool
) {
    let value_str = node.value().to_string();
    let display_name = Path::new(&value_str).to_string_lossy().to_string();
    // unwrap().path().to_string_lossy().to_string();
    
    println!("{}{} {}", 
        prefix, 
        if is_last { "└──" } else { "├──" }, 
        display_name
    );
    
    let children: Vec<_> = node.children().collect();
    for (i, child) in children.iter().enumerate() {
        let is_last_child = i == children.len() - 1;
        let new_prefix = format!(
            "{}{}", 
            prefix, 
            if is_last { "    " } else { "│   " }
        );
        print_node_fancy(*child, &new_prefix, is_last_child);
    }
}

pub fn list_directory(tree: &NodeRef<DirectoryEntry>) {
    let mut i = 0;
    for element in tree.children() {
        println!("{}. {}",i,  element.value());
        i += 1;
    }
}

pub fn list_directories(tree: &NodeRef<DirectoryEntry>) {
    let mut i = 0;
    for element in tree.children() {
        if(element.value().is_dir == true) {
            println!("{}. {}",i,  element.value());
        }
        i += 1;
    }
}

pub fn get_directory<'a>(tree: NodeRef<DirectoryEntry>, num: i32) -> NodeRef<DirectoryEntry> {
    let mut i = 0;
    for element in tree.children() {
        if(element.value().is_dir == true && i == num) {
            return element;
        }
        i += 1;
    }

    return tree;
}

pub fn change_working_directory(num: i32, working_directory: &mut NodeRef<DirectoryEntry>) {
    if(num == -1) {
        *working_directory = working_directory.parent().unwrap_or(*working_directory);
        return
    }
    *working_directory = get_directory(*working_directory, num);
}