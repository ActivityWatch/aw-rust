/*
 * Much of the code taken from https://github.com/rustless/rustless-example/
 */

extern crate rustless;
extern crate iron;
extern crate hyper;
extern crate rustc_serialize as serialize;
extern crate valico;
/*extern crate router;*/

extern crate time;
extern crate rusqlite;

//use hyper::status::StatusCode;
/*use router::Router;*/

use rustless::Nesting;
use rustless::batteries::swagger;
use rustless::batteries::schemes;

use valico::json_schema;

//use rusqlite::Connection;

mod api;

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

    println!("Server running on {}:{}", host, port);
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

    schemes::enable_schemes(&mut app, scope).unwrap();
}

fn setup_db(mut app: &mut rustless::Application) {
    //run_db(&mut app);
}
