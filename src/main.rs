extern crate iron;
extern crate router;
extern crate params;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;

use router::Router;

use std::process;
use std::process::Command;

use params::{Params, Value};

use std::env;

fn main() {
    let mut router = Router::new();

    router.get("/", get_from, "root");
    router.get("/:cmd_test", cmd_test, "cmd_test");
    router.get("/quit", quit, "quit");

    println!("Serving on http://127.0.0.1:31764 ...");

    Command::new("cmd")
        .args(&["/C", "EXPLORER http://127.0.0.1:31764"]) // EXPLORER opens default browser on Windows.
        .spawn()
        .expect("failed to open default browser with localhost:31764");

    // and then
    Iron::new(router).http("127.0.0.1:31764").unwrap();
}

fn get_from(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    let mime = "Text/Html; Charset=Utf8".parse::<Mime>().unwrap();
    response.set_mut(mime);
    response.set_mut(r#"
        <title>localhoster</title>
        <h1>localhoster</h1>
        <p><a href="/:cmd_test?cmd=test.bat">cmd_test</a></p>
        <p><a href="/quit">quit</a></p>
    "#);

    Ok(response)
}

fn cmd_test(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    let mime = "Text/Html; Charset=Utf8".parse::<Mime>().unwrap();
    response.set_mut(mime);

    let param_map = request.get_ref::<Params>().unwrap();

    // https://doc.rust-lang.org/1.27.0/std/env/fn.current_dir.html
    // https://docs.rs/params/0.5.1/params/struct.Map.html
    let mut path = env::current_dir().unwrap();
    match param_map.find(&["cmd"]) {
        Some(&Value::String(ref name)) => path.push(name),
        _ => path.push("")
    };

    let output = 
        Command::new("cmd")
            .args(&["/C", path.to_str().unwrap()])
            .output()
            .expect("failed to execute process.");

    response.set_mut(format!(r#"
        <title>cmd_test</title>
        <h1>cmd_test</h1>
        <p><a href="/">home</a></p>
        <p><pre>{:#?}</pre></p>
        <p><pre>{:#?}</pre></p>
        <p><a href="/:cmd_test?cmd=test.bat">cmd_test</a></p>
        <p><a href="/quit">quit</a></p>
    "#, path, output));

    Ok(response)
}

fn quit(_request: &mut Request) -> IronResult<Response> {
    process::exit(0)
}


