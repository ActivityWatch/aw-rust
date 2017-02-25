use iron::typemap;

use rustless::{self, Extensible};

use rusqlite::Connection;

// A unit struct we need to store our pool into TypeMap
// See `typemap` docs for clarification.
pub struct AppDb;
impl typemap::Key for AppDb {
    type Value = Connection;
}

// Let's create a shorthand function to simplify a DB access within our
// endpoints.
pub trait DatabaseExt: rustless::Extensible {
    fn db(&self) -> &Connection;
}

impl DatabaseExt for rustless::Application {
    fn db(&self) -> &Connection {
        //create_connection()
        self.ext().get::<AppDb>().unwrap()
    }
}

pub fn create_connection() -> Connection {
    let conn = Connection::open_in_memory().unwrap();

    conn.execute("CREATE TABLE event (
                      id        INTEGER PRIMARY KEY,
                      timestamp TEXT NOT NULL,
                      bucket    TEXT NOT NULL,
                      data      BLOB
                  )", &[]).unwrap();
    conn
}
