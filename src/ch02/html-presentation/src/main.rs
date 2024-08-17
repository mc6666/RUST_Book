use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};
use std::path::PathBuf;

async fn index(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    let mut full_path: PathBuf = PathBuf::from("static/");
    full_path.push(path);
    Ok(NamedFile::open(full_path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| App::new().route("/{filename:.*}", web::get().to(index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

// use actix_files as fs;
// use actix_web::{App, HttpServer};

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
    // HttpServer::new(|| App::new().service(fs::Files::new("/static", ".").show_files_listing()))
        // .bind(("127.0.0.1", 8080))?
        // .run()
        // .await
// }