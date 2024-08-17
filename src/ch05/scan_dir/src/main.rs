use std::io;
use std::path::Path;

// one possible implementation of walking a directory only visiting files
fn visit_dirs(dir: &Path, vec: &mut Vec<String>) -> io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        // 從DirEntry轉換為正常的Entry，以取得相關屬性
        let entry = entry?; 
        let path = entry.path();
        if path.is_dir() {
            visit_dirs(&path, vec)?;
        } else {
            vec.push(entry.path().display().to_string());
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let mut path:String = ".".to_string();
    if std::env::args().len() > 1 {
        path = std::env::args().nth(1).expect("no path given");
    }
    
    // method 1, scan folder recursively
    let mut vec: Vec<String> = Vec::new();
    let _ = visit_dirs(&std::path::PathBuf::from(path), &mut vec);
    for item in vec {
        println!("{item}");
    }
    Ok(())
    
    // method 2, scan top folder only
    // let mut entries = std::fs::read_dir(path)?
        // .map(|res| res.map(|e| e.path()))
        // .collect::<Result<Vec<_>, io::Error>>()?;

    // entries.sort();
    // println!("{:?}", entries);
    
}