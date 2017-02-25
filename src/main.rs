/*
 * Much of the code taken from https://github.com/rustless/rustless-example/
 */

extern crate rustless;
extern crate iron;
extern crate hyper;
extern crate valico;

extern crate time;
extern crate rusqlite;
extern crate serde_json;

use rustless::Nesting;
use rustless::batteries::swagger;
use rustless::batteries::schemes;

use valico::json_schema;

use std::io::Read;

//use rusqlite::Connection;

mod api;
mod db;

fn main() {
    let mut app = rustless::Application::new(self::api::root());

    setup_swagger(&mut app);
    setup_jsonschema(&mut app);
    setup_db(&mut app);

    // See `iron` docs for clarification.
    let chain = iron::Chain::new(app);

    /*let host: net::IpAddr = args.flag_ip.parse().unwrap();
     *let port: u16 = args.flag_port.parse().unwrap();*/
    let host = "0.0.0.0";
    let port: u16 = 8585;

    println!("Server running on http://{}:{}/", host, port);
    println!("API docs at http://{}:{}/api-docs", host, port);
    iron::Iron::new(chain).http((host, port)).unwrap();
}

fn setup_swagger(mut app: &mut rustless::Application) {
    swagger::enable(&mut app, swagger::Spec {
        info: swagger::Info {
            title: "ActivityWatch".to_string(),
            description: Some("No description yet.".to_string()),
            contact: Some(swagger::Contact {
                name: "Erik Bjäreholt".to_string(),
                url: Some("http://erik.bjareho.lt".to_string()),
                ..std::default::Default::default()
            }),
            license: Some(swagger::License {
                name: "MIT".to_string(),
                url: "http://opensource.org/licenses/MIT".to_string()
            }),
            ..std::default::Default::default()
        },
        ..std::default::Default::default()
    });

    app.root_api.mount(swagger::create_api("api-docs"));
}

fn setup_jsonschema(mut app: &mut rustless::Application) {
    let scope = json_schema::Scope::new();

    // Can't get to work due to event_schema_json needing to be of type rustless::Value which, as
    // far as I can see, is the same as serde_json::Value but for some reason wont work.
    // See issue I made here: https://github.com/rustless/valico/issues/25
    /*
    let event_schema_json: serde_json::Value = {
        let file = std::fs::File::open("schemas/event.json").unwrap();
        let mut event_schema_string = String::new();
        file.read_to_string(&mut event_schema_string);
        serde_json::from_str(event_schema_string.as_str()).unwrap()
    };

    let event_schema = scope.compile_and_return(event_schema_json.clone(), true).ok().unwrap();
    println!("Is valid: {}", event_schema.validate(&event_schema_json).is_valid());
    */

    schemes::enable_schemes(&mut app, scope).unwrap();
}

fn setup_db(mut app: &mut rustless::Application) {
    let conn = db::create_connection();

    app.ext.insert::<self::db::AppDb>(conn);
}
