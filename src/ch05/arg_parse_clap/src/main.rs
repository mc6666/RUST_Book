use clap::Parser;

// filepath 預設值
const DEFAULT_PATH:&str = "*.*";

// 設定參數結構
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 比對字串
    #[arg(short, long)]
    pattern: String,
    
    /// file to load
    #[arg(short, long, default_value_t = DEFAULT_PATH.to_string())]
    filepath: String,
}

fn main() {
    let args = Args::parse();

    println!("{}", args.pattern);
    let path: std::path::PathBuf = std::path::PathBuf::from(args.filepath);
    println!("{:?}", path);
}
