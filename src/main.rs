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
use valico::json_dsl;
/*use router::Router;*/

use rustless::batteries::swagger;
use rustless::Nesting;

//use rusqlite::Connection;

mod api;

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
}

fn main() {
/*    let mut router = Router::new();
 *    router.get("/", hello_world, "index");
 *    router.get("/tests/sql", test_sql, "index");
 *
 *    Iron::new(router).http("localhost:3000").unwrap();
 *    println!("On 3000");*/

    /*hello_world();
     *test_sql();*/
    println!("Creating app");
    let mut app = rustless::Application::new(self::api::root());

    setup_swagger(&mut app);
    println!("asd");

    app.root_api.mount(swagger::create_api("api-docs"));
    /*schemes::enable_schemes(&mut app, json_schema::Scope::new()).unwrap();*/

    /*run_db(&mut app);*/

    // See `iron` docs for clarification.
    let chain = iron::Chain::new(app);

    /*let host: net::IpAddr = args.flag_ip.parse().unwrap();
     *let port: u16 = args.flag_port.parse().unwrap();*/
    let host = "0.0.0.0";
    let port: u16 = 8585;

    iron::Iron::new(chain).http((host, port)).unwrap();

    println!("On {}", port);
/*
 *    iron::Iron::new(app).http("0.0.0.0:4000").unwrap();
 *    println!("On 4000");
 *
 *    println!("Rustless server started!");*/
}
