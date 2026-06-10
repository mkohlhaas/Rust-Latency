use actix_web::{App, Error, HttpServer, error::ErrorInternalServerError, web};
use anyhow::Result;
use clap::Parser;
use rusqlite::Connection;
use std::env;
use std::sync::{Arc, Mutex};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  #[arg(short, long)]
  node_id: u16,
}

type Conn = web::Data<Arc<Mutex<Connection>>>;

#[tokio::main]
async fn main() -> Result<()> {
  let args = Args::parse();
  let port = 8080 + args.node_id;
  let listen_addr = format!("127.0.0.1:{}", port);
  let database_url = env::var("DATABASE_URL")?;
  let conn = Arc::new(Mutex::new(Connection::open(database_url)?));
  let app = move || {
    App::new()
      .app_data(web::Data::new(conn.clone()))
      .service(web::resource("/hello").route(web::get().to(say_hello)))
  };

  Ok(HttpServer::new(app).bind(listen_addr)?.run().await?)
}

async fn say_hello(conn: Conn) -> Result<String, Error> {
  let conn = conn.lock().unwrap();
  let result = conn
    .query_row("SELECT 'hello world'", [], |row| row.get(0))
    .map_err(ErrorInternalServerError)?;

  Ok(result)
}
