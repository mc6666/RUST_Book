use std::{fmt::Debug, fs::{self, File}, io::{BufReader, BufWriter, Read, Write}};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Serialize)]
pub struct DirWalkerEntry {
    pub file_name: String,
    pub path: String,
    pub depth: u32,
    pub is_dir: bool,
    pub is_file: bool,
    pub size: u64
}

pub struct DirWalker {
    pub items: Vec<DirWalkerEntry>,
    pub depth: u32
}

impl DirWalker {
    pub fn new() -> DirWalker {
        DirWalker {
            items: Vec::new(),
            depth: 0
        }
    }

    pub fn run(&mut self, path: &str) -> &mut Self {
        self.walk(path, 0);
        self
    }

    pub fn walk(&mut self, path: &str, depth: u32) {
        if self.depth > 0 && depth > self.depth {
            return;
        }
        for entry in fs::read_dir(path).unwrap() {
            let item = entry.unwrap();
            if item.file_name().to_str().unwrap().starts_with(".") {
                continue;
            }
            let path = item.path();
            if !fs::metadata(&path).is_ok() {
                continue;
            }
            if path.is_dir() {
                self.items.push(DirWalkerEntry {
                    file_name: item.file_name().to_str().unwrap().to_string(),
                    path: path.to_str().unwrap().to_string(),
                    depth: depth,
                    is_dir: true,
                    is_file: false,
                    size: 0
                });
                self.walk(path.to_str().unwrap(), depth + 1);
            }
            else {
                self.items.push(DirWalkerEntry {
                    file_name: item.file_name().to_str().unwrap().to_string(),
                    path: path.to_str().unwrap().to_string(),
                    depth: depth,
                    is_dir: false,
                    is_file: true,
                    size: fs::metadata(&path).unwrap().len()
                });
            }
        }
    }

    pub fn depth(&mut self, depth: u32) -> &mut Self {
        self.depth = depth;
        self
    }

    pub fn ext(&mut self, extensions: Vec<&str>) -> &mut Self {
        self.items = self.items.clone().into_iter().filter(|item| {
            for ext in &extensions {
                if item.file_name.ends_with(ext) {
                    return true;
                }
            }
            false
        }).collect();
        self
    }

    pub fn get_items(&self) -> Vec<DirWalkerEntry> {
        (*self.items).to_vec()
    }
}
