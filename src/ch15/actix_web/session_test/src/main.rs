use actix_session::{Session, SessionMiddleware, storage::CookieSessionStore};
use actix_web::{web, App, Error, HttpResponse, HttpServer, cookie::Key};

async fn index(session: Session) -> Result<HttpResponse, Error> {
    // 讀取 session 變數 counter
    if let Some(count) = session.get::<i32>("counter")? {
        // counter 每次加1
        session.insert("counter", count + 1)?;
    } else {
        session.insert("counter", 1)?; // counter不存在，設定counter=1
    }

    // 回傳 counter
    Ok(HttpResponse::Ok().body(format!(
        "Count is {:?}!",
        session.get::<i32>("counter")?.unwrap()
    )))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                // 使用 session middleware 建立 cookie 
                SessionMiddleware::builder(
                    CookieSessionStore::default()
                    , Key::from(&[0; 64]))
                    .cookie_secure(false)
                    .build()
            )
            .service(web::resource("/").to(index))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}