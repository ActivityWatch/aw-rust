use std::fmt;
use std::str::FromStr;

use serde_json;
use serde_json::{Error, Value};



#[derive(Serialize, Deserialize)]
struct Bucket {
    id: u8,
    id_str: String,
    event_type: String
}


#[derive(Serialize, Deserialize)]
struct Event {
    bucket_id: u8,
    timestamp: u8,
    duration: f32,
    data: Value
}

struct Database {
    bucket_count: u8
}

impl Database {
    fn new() -> Database {
        Database { bucket_count: 0 }
    }

    fn create_bucket(&mut self, id_str: &str, event_type: &str) -> Bucket {
        let bucket_id = self.bucket_count;
        self.bucket_count += 1;
        Bucket {
            id: bucket_id,
            id_str: String::from(id_str),
            event_type: String::from(event_type)
        }
    }
}

impl Bucket {
    fn create_event_from_json(&self, json: &str) -> Result<Event, Error> {
        let mut e: Event = serde_json::from_str(json)?;
        e.bucket_id = self.id;
        Ok(e)
    }
}

impl fmt::Display for Event {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "Bucket id '{}' and data '{}'", self.bucket_id, self.data)
    }
}

pub fn example() -> Result<(), Error> {
    let mut db: Database = Database::new();

    let b: Bucket = db.create_bucket("bucket_name", "activewindow");

    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"{
                    "bucket_id": 0,
                    "timestamp": 0,
                    "duration": 0.1,
                    "data": {
                        "appname": "Google Chrome",
                        "title": "idk man"
                    }
                  }"#;

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let e: Event = b.create_event_from_json(data)?;
    println!("{}", e);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::example;

    #[test]
    fn event() {
        example();
    }
}
