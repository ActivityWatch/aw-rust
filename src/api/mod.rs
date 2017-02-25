extern crate rustless;
extern crate iron;
extern crate rustc_serialize as serialize;
extern crate valico;
/*extern crate router;*/

extern crate time;
extern crate rusqlite;

use rustless::{
    Api, Nesting, Versioning
};
use valico::json_dsl;
/*use router::Router;*/

use time::Timespec;
use rusqlite::Connection;

pub fn root() -> rustless::Api {
    Api::build(|api| {
        // TODO: What does Versioning::AcceptHeader mean here?
        /*api.version("v1", Versioning::AcceptHeader("aw"));
         *api.prefix("api");*/

        api.mount(buckets());

        api.mount(Api::build(|tests_api| {
            tests_api.prefix("tests");

            tests_api.get("hello", |endpoint| {
                endpoint.handle(|client, params| {
                    println!("Running hello_world test");
                    client.text(hello_world())
                })
            });

            tests_api.get("sql", |endpoint| {
                endpoint.handle(|client, params| {
                    println!("Running SQL test");
                    test_sql();
                    client.text(String::from("ran SQL test"))
                })
            });
        }));
    })
}

fn buckets() -> rustless::Api {
    Api::build(|buckets_api| {
        buckets_api.prefix("buckets");

        buckets_api.mount(Api::build(|events_api| {
            events_api.namespace(":id", |event_ns| {
                event_ns.params(|params| {
                    params.req_typed("id", json_dsl::u64());
                });

                event_ns.get("events", |endpoint| {
                    endpoint.handle(|client, params| {
                        println!("{:?}", params);
                        client.text(String::from("events will be listed here later"))
                    })
                });

                event_ns.get("heartbeat", |endpoint| {
                    endpoint.handle(|client, params| {
                        println!("{:?}", params);
                        client.text(String::from("events will be listed here later"))
                    })
                });
            });
        }));
    })
}

fn hello_world() -> String {
    String::from("Hello World!")
    //Ok(Response::with((status::Ok, "Hello World!")))
}

#[derive(Debug)]
struct Event {
    id: i32,
    timestamp: Timespec,
    bucket: String,
    data: Option<Vec<u8>>
}

fn test_sql() {
    let conn = Connection::open_in_memory().unwrap();

    conn.execute("CREATE TABLE event (
                  id              INTEGER PRIMARY KEY,
                  timestamp       TEXT NOT NULL,
                  bucket          TEXT NOT NULL,
                  data            BLOB
                  )", &[]).unwrap();
    let me = Event {
        id: 0,
        timestamp: time::get_time(),
        bucket: "test-bucket".to_string(),
        data: None
    };
    conn.execute("INSERT INTO event (timestamp, bucket, data)
                  VALUES (?1, ?2, ?3)",
                 &[&me.timestamp, &me.bucket, &me.data]).unwrap();

    let mut stmt = conn.prepare("SELECT id, timestamp, bucket, data FROM event").unwrap();
    let event_iter = stmt.query_map(&[], |row| {
        Event {
            id: row.get(0),
            timestamp: row.get(1),
            bucket: row.get(2),
            data: row.get(3)
        }
    }).unwrap();

    for event in event_iter {
        println!("Found event {:?}", event.unwrap());
    }

    //Ok(Response::with((status::Ok, "SQL Test was ran")))
}
