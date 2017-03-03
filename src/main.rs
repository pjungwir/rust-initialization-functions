extern crate postgres;
#[macro_use]
extern crate chan;

use std::io::prelude::*;
use std::io;
use std::io::{Read, Write};
use std::fs::File;
use std::path::Path;
use postgres::{Connection, TlsMode};
use postgres::tls::openssl::OpenSsl;
use postgres::types::ToSql;
use postgres::transaction::Transaction;
use postgres::stmt::Statement;
use std::env;

fn main() {
  let mut log = open_log();
  let conn = make_db_connection();
  let db = make_app_db(&conn);
  
  do_the_work(&mut log, &db);
}

fn do_the_work<W: Write>(mut log: W, db: &MyAppDb) {
  println!("working hard or hardly working?");
}

pub struct MyAppDb<'a> {
  pub insert_user: Statement<'a>,
  pub insert_order: Statement<'a>,
  // ... etc ...
}

fn make_db_connection() -> Connection {
  let negotiator = OpenSsl::new().unwrap();   // Can't go out of scope---and we don't even use it all the time!
  let url = env::var("MYAPP_DATABASE").unwrap_or("postgres://myapp_test:secret@localhost:5432/myapp_test".to_owned());
  let tls = if url.contains("@localhost") { TlsMode::None }
            else { TlsMode::Require(&negotiator) };
  Connection::connect(url, tls).expect("Can't connect to Postgres")
}

fn make_app_db(conn: &Connection) -> MyAppDb {
  let insert_user = conn.prepare("INSERT INTO users VALUES ($1, $2)").unwrap();
  let insert_order = conn.prepare("INSERT INTO orders VALUES ($1, $2)").unwrap();

  MyAppDb {
    insert_user: insert_user,
    insert_order: insert_order,
  }
}

// This actually compiles,
// but I would love to have it with no Box.
fn open_log() -> Box<Write> {
  let mut job_id = None;
  let job_id_env = env::var("MYAPP_JOB_ID");
  if let Ok(val) = job_id_env {
    job_id = Some(val.clone());
    let home = env::var("HOME").expect("HOME must be set");
    let path = format!("{}/log/myapp-{}.log", home, val);
    let path = Path::new(&path);
    match File::create(&path) {
      Ok(mut f) => Box::new(f) as Box<Write>,
      Err(e) => {
        if format!("{}", e) == "No such file or directory (os error 2)" {
          Box::new(io::stdout()) as Box<Write> // oh well
        } else {
          panic!("Can't open log file: {}", e);
        }
      },
    }
  } else {
    Box::new(io::stdout()) as Box<Write>
  }
}



