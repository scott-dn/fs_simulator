use std::{collections::BTreeMap, fmt::Display};

pub struct DirManager {
    df_dir: String,
    root_dir: Node,
}

impl DirManager {
    pub fn new() -> Self {
        Self {
            df_dir: "root".into(),
            root_dir: Node::new(),
        }
    }

    pub fn handle_up(&mut self) -> Result<(), &str> {
        if self.df_dir.eq("root") {
            return Err("Cannot move up from root directory");
        }
        self.df_dir = Self::get_parent_path(&self.df_dir);
        Ok(())
    }

    pub fn handle_dir(&self) -> (&String, Vec<&String>) {
        let node = self.get_node(&self.df_dir).unwrap();
        (&self.df_dir, node.child.keys().collect())
    }

    pub fn handle_cd(&mut self, dir: String) -> Result<(), &str> {
        let node = self.get_node(&self.df_dir).unwrap();
        if !node.child.contains_key(&dir) {
            return Err("Subdirectory does not exist");
        }
        self.df_dir = format!("{}\\{dir}", self.df_dir);
        Ok(())
    }

    pub fn handle_mkdir(&mut self, dir: String) -> Result<(), &str> {
        let node = self.get_node_mut(&self.df_dir.clone()).unwrap();
        if node.child.contains_key(&dir) {
            return Err("Subdirectory already exists");
        }
        node.child.insert(dir, Node::new());
        Ok(())
    }

    pub fn handle_mv(&mut self, source: String, destination: String) -> Result<(), &str> {
        let df_node = self.get_node(&self.df_dir).unwrap();
        if !df_node.child.contains_key(&source) {
            return Err("Subdirectory does not exist");
        }

        let (des_raw, des_abs) = self.to_abs_path(&destination);
        if !Self::get_parent_path(&des_abs).starts_with("root") {
            return Err("Illegal relative path");
        }

        if let Some(des_node) = self.get_node(&des_abs) {
            if des_node.child.get(&source).is_some() {
                return Err("Subdirectory already exists");
            }
        }

        let df_node = self.get_node_mut(&self.df_dir.clone()).unwrap();
        let src_node = df_node.child.remove(&source).unwrap();
        if let Some(des_node) = self.get_node_mut(&des_abs) {
            des_node.child.insert(source, src_node);
        } else {
            let parent_des_abs = &Self::get_parent_path(&des_abs);
            let parent_des_node = self.get_node_mut(parent_des_abs).unwrap();
            parent_des_node.child.insert(des_raw, src_node);
        }
        Ok(())
    }

    pub fn handle_tree(&mut self) -> (&str, &Node) {
        (&self.df_dir, self.get_node(&self.df_dir).unwrap())
    }

    fn get_parent_path(path: &str) -> String {
        let split = path.split('\\');
        let count = split.clone().count();
        split.take(count - 1).collect::<Vec<_>>().join("\\")
    }

    fn to_abs_path(&self, path: &str) -> (String, String) {
        if let Some(path_strip) = path.strip_prefix(".\\") {
            let raw_path = path_strip.to_string();
            let abs_path = format!("{}\\{}", self.df_dir, &raw_path);
            (raw_path, abs_path)
        } else if let Some(path_strip) = path.strip_prefix("..\\") {
            let raw_path = path_strip.to_string();
            let abs_path = format!("{}\\{}", Self::get_parent_path(&self.df_dir), &raw_path);
            (raw_path, abs_path)
        } else {
            (path.to_string(), format!("{}\\{path}", self.df_dir))
        }
    }

    fn get_node_mut(&mut self, path: &str) -> Option<&mut Node> {
        let mut node = Some(&mut self.root_dir);
        for path in path.split('\\').skip(1) {
            if let Some(found) = node {
                node = found.child.get_mut(path);
            } else {
                break;
            }
        }
        node
    }

    fn get_node(&self, path: &str) -> Option<&Node> {
        let mut node = Some(&self.root_dir);
        for path in path.split('\\').skip(1) {
            if let Some(found) = node {
                node = found.child.get(path);
            } else {
                break;
            }
        }
        node
    }
}

pub struct Node {
    child: BTreeMap<String, Node>,
}

impl Node {
    fn new() -> Self {
        Node { child: BTreeMap::new() }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        display_tree_util(0, self, f)
    }
}

fn display_tree_util(depth: usize, node: &Node, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let len = node.child.len();
    for (idx, (dir, node)) in node.child.iter().enumerate() {
        for i in 0..(depth * 4) {
            if i == 0 {
                write!(f, "│")?;
            } else {
                write!(f, " ")?;
            }
        }
        if idx == len - 1 {
            writeln!(f, "└── {}", dir)?;
        } else {
            writeln!(f, "├── {}", dir)?;
        }
        display_tree_util(depth + 1, node, f)?;
    }
    Ok(())
}
