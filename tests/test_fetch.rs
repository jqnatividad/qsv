use crate::workdir::Workdir;

#[test]
fn fetch_simple() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL"],
            svec!["https://api.zippopotam.us/us/99999"],
            svec!["  http://api.zippopotam.us/us/90210"],
            svec!["https://api.zippopotam.us/us/94105"],
            svec!["http://api.zippopotam.us/us/92802"],
            svec!["https://query.wikidata.org/sparql?query=SELECT%20?dob%20WHERE%20{wd:Q42%20wdt:P569%20?dob.}&format=json"],
        ],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL").arg("data.csv").arg("--store-error");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![r#"HTTP 404 - Not Found"#],
        svec![
            r#"{"post code":"90210","country":"United States","country abbreviation":"US","places":[{"place name":"Beverly Hills","longitude":"-118.4065","state":"California","state abbreviation":"CA","latitude":"34.0901"}]}"#
        ],
        svec![
            r#"{"post code":"94105","country":"United States","country abbreviation":"US","places":[{"place name":"San Francisco","longitude":"-122.3892","state":"California","state abbreviation":"CA","latitude":"37.7864"}]}"#
        ],
        svec![
            r#"{"post code":"92802","country":"United States","country abbreviation":"US","places":[{"place name":"Anaheim","longitude":"-117.9228","state":"California","state abbreviation":"CA","latitude":"33.8085"}]}"#
        ],
        svec![
            r#"{"head":{"vars":["dob"]},"results":{"bindings":[{"dob":{"datatype":"http://www.w3.org/2001/XMLSchema#dateTime","type":"literal","value":"1952-03-11T00:00:00Z"}}]}}"#
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn fetch_simple_url_template() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![
            svec!["zip code"],
            svec!["99999"],
            svec!["  90210   "],
            svec!["94105  "],
            svec!["92802"],
        ],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("1")
        .arg("data.csv")
        .arg("--store-error")
        .arg("--url-template")
        .arg("https://api.zippopotam.us/us/{zip_code}");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![r#"HTTP 404 - Not Found"#],
        svec![
            r#"{"post code":"90210","country":"United States","country abbreviation":"US","places":[{"place name":"Beverly Hills","longitude":"-118.4065","state":"California","state abbreviation":"CA","latitude":"34.0901"}]}"#
        ],
        svec![
            r#"{"post code":"94105","country":"United States","country abbreviation":"US","places":[{"place name":"San Francisco","longitude":"-122.3892","state":"California","state abbreviation":"CA","latitude":"37.7864"}]}"#
        ],
        svec![
            r#"{"post code":"92802","country":"United States","country abbreviation":"US","places":[{"place name":"Anaheim","longitude":"-117.9228","state":"California","state abbreviation":"CA","latitude":"33.8085"}]}"#
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn fetch_simple_redis() {
    // if there is no local redis server, skip fetch_simple_redis test
    let redis_client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
    if let Err(_) = redis_client.get_connection() {
        return;
    }

    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL"],
            svec!["https://api.zippopotam.us/us/99999"],
            svec!["  http://api.zippopotam.us/us/90210"],
            svec!["https://api.zippopotam.us/us/94105"],
            svec!["http://api.zippopotam.us/us/92802"],
            svec!["https://query.wikidata.org/sparql?query=SELECT%20?dob%20WHERE%20{wd:Q42%20wdt:P569%20?dob.}&format=json"],
        ],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("data.csv")
        .arg("--store-error")
        .arg("--redis");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![r#"HTTP 404 - Not Found"#],
        svec![
            r#"{"post code":"90210","country":"United States","country abbreviation":"US","places":[{"place name":"Beverly Hills","longitude":"-118.4065","state":"California","state abbreviation":"CA","latitude":"34.0901"}]}"#
        ],
        svec![
            r#"{"post code":"94105","country":"United States","country abbreviation":"US","places":[{"place name":"San Francisco","longitude":"-122.3892","state":"California","state abbreviation":"CA","latitude":"37.7864"}]}"#
        ],
        svec![
            r#"{"post code":"92802","country":"United States","country abbreviation":"US","places":[{"place name":"Anaheim","longitude":"-117.9228","state":"California","state abbreviation":"CA","latitude":"33.8085"}]}"#
        ],
        svec![
            r#"{"head":{"vars":["dob"]},"results":{"bindings":[{"dob":{"datatype":"http://www.w3.org/2001/XMLSchema#dateTime","type":"literal","value":"1952-03-11T00:00:00Z"}}]}}"#
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn fetch_jql_single() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL"],
            svec!["http://api.zippopotam.us/us/90210"],
            svec!["http://api.zippopotam.us/us/94105"],
            svec!["https://api.zippopotam.us/us/92802"],
        ],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("--new-column")
        .arg("City")
        .arg("--jql")
        .arg(r#"."places"[0]."place name""#)
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["URL", "City"],
        svec!["http://api.zippopotam.us/us/90210", "Beverly Hills"],
        svec!["http://api.zippopotam.us/us/94105", "San Francisco"],
        svec!["https://api.zippopotam.us/us/92802", "Anaheim"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn fetch_jql_single_file() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL"],
            svec!["http://api.zippopotam.us/us/90210"],
            svec!["http://api.zippopotam.us/us/94105"],
            svec!["https://api.zippopotam.us/us/92802"],
        ],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("--new-column")
        .arg("City")
        .arg("--jqlfile")
        .arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/resources/test/fetch_jql_single.jql"
        ))
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["URL", "City"],
        svec!["http://api.zippopotam.us/us/90210", "Beverly Hills"],
        svec!["http://api.zippopotam.us/us/94105", "San Francisco"],
        svec!["https://api.zippopotam.us/us/92802", "Anaheim"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn fetch_jql_jqlfile_error() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![svec!["URL"], svec!["http://api.zippopotam.us/us/90210"]],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("--jqlfile")
        .arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/resources/test/fetch_jql_single.jql"
        ))
        .arg("--jql")
        .arg(r#"."places"[0]."place name""#)
        .arg("data.csv");

    let got: String = wrk.output_stderr(&mut cmd);
    assert_eq!(
        &*got,
        r#"Invalid arguments.

Usage:
    qsv fetch [options] [--jql <selector> | --jqlfile <file> ] [--http-header <k:v>...] [<column>] [<input>]
"#
    )
}

#[test]
fn fetch_jql_multiple() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL"],
            svec!["http://api.zippopotam.us/us/90210"],
            svec!["http://api.zippopotam.us/us/94105"],
            svec!["https://api.zippopotam.us/us/92802"],
        ],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("--new-column")
        .arg("CityState")
        .arg("--jql")
        .arg(r#""places"[0]."place name","places"[0]."state abbreviation""#)
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["URL", "CityState"],
        svec!["http://api.zippopotam.us/us/90210", "Beverly Hills, CA"],
        svec!["http://api.zippopotam.us/us/94105", "San Francisco, CA"],
        svec!["https://api.zippopotam.us/us/92802", "Anaheim, CA"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn fetch_jql_multiple_file() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL"],
            svec!["http://api.zippopotam.us/us/90210"],
            svec!["http://api.zippopotam.us/us/94105"],
            svec!["https://api.zippopotam.us/us/92802"],
        ],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("--new-column")
        .arg("CityState")
        .arg("--jqlfile")
        .arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/resources/test/fetch_jql_multiple.jql"
        ))
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["URL", "CityState"],
        svec!["http://api.zippopotam.us/us/90210", "Beverly Hills, CA"],
        svec!["http://api.zippopotam.us/us/94105", "San Francisco, CA"],
        svec!["https://api.zippopotam.us/us/92802", "Anaheim, CA"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn fetch_custom_header() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![svec!["URL"], svec!["http://httpbin.org/get"]],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("--http-header")
        .arg(" X-Api-Key :  DEMO_KEY")
        .arg("--http-header")
        .arg("X-Api-Secret :ABC123XYZ")
        .arg("--jql")
        .arg(r#""headers"."X-Api-Key","headers"."X-Api-Secret""#)
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["DEMO_KEY, ABC123XYZ"]];
    assert_eq!(got, expected);
}

use std::{sync::mpsc, thread};

use actix_web::{
    dev::ServerHandle, middleware, rt, web, App, HttpRequest, HttpServer, Responder, Result,
};

use serde::Serialize;
#[derive(Serialize)]
struct MyObj {
    fullname: String,
}

async fn index() -> impl Responder {
    "Hello world!"
}

/// handler with path parameters like `/user/{name}/`
/// returns Smurf fullname in JSON format
async fn get_fullname(req: HttpRequest, name: web::Path<String>) -> Result<impl Responder> {
    println!("{:?}", req);

    let obj = MyObj {
        fullname: format!("{} Smurf", name),
    };

    Ok(web::Json(obj))
}

// convenience macros for changing test ip/port to use
macro_rules! test_server {
    () => {
        "127.0.0.1:8081"
    };
}

macro_rules! test_url {
    ($api_parm:expr) => {
        concat!("http://", test_server!(), "/", $api_parm)
    };
}

/// start an Actix Webserver with Rate Limiting via Governor
async fn run_webserver(tx: mpsc::Sender<ServerHandle>) -> std::io::Result<()> {
    use actix_governor::{Governor, GovernorConfigBuilder};

    // Allow bursts with up to five requests per IP address
    // and replenishes one element every 250 ms (4 qps)
    let governor_conf = GovernorConfigBuilder::default()
        .per_millisecond(250)
        .burst_size(7)
        .finish()
        .unwrap();

    // server is server controller type, `dev::ServerHandle`
    let server = HttpServer::new(move || {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(Governor::new(&governor_conf))
            .service(web::resource("/user/{name}").route(web::get().to(get_fullname)))
            .service(web::resource("/").to(index))
    })
    .bind(test_server!())?
    .run();

    // send server controller to main thread
    let _ = tx.send(server.handle());

    // run future
    server.await
}

#[test]
fn fetch_ratelimit() {
    // start webserver with rate limiting
    let (tx, rx) = mpsc::channel();

    println!("START Webserver ");
    thread::spawn(move || {
        let server_future = run_webserver(tx);
        rt::System::new().block_on(server_future)
    });

    let server_handle = rx.recv().unwrap();

    // proceed with usual unit test
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL"],
            svec![test_url!("user/Smurfette")],
            svec![test_url!("user/Papa")],
            svec![test_url!("user/Clumsy")],
            svec![test_url!("user/Brainy")],
            svec![test_url!("user/Grouchy")],
            svec![test_url!("user/Hefty")],
            svec![test_url!("user/Greedy")],
            svec![test_url!("user/Jokey")],
            svec![test_url!("user/Chef")],
            svec![test_url!("user/Vanity")],
            svec![test_url!("user/Handy")],
            svec![test_url!("user/Scaredy")],
            svec![test_url!("user/Tracker")],
            svec![test_url!("user/Sloppy")],
            svec![test_url!("user/Harmony")],
            svec![test_url!("user/Painter")],
            svec![test_url!("user/Poet")],
            svec![test_url!("user/Farmer")],
            svec![test_url!("user/Natural")],
            svec![test_url!("user/Snappy")],
            svec![test_url!(
                "user/The quick brown fox jumped over the lazy dog by the zigzag quarry site"
            )],
        ],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("--new-column")
        .arg("Fullname")
        .arg("--jql")
        .arg(r#"."fullname""#)
        .arg("--rate-limit")
        .arg("4")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["URL", "Fullname"],
        svec![test_url!("user/Smurfette"), "Smurfette Smurf"],
        svec![test_url!("user/Papa"), "Papa Smurf"],
        svec![test_url!("user/Clumsy"), "Clumsy Smurf"],
        svec![test_url!("user/Brainy"), "Brainy Smurf"],
        svec![test_url!("user/Grouchy"), "Grouchy Smurf"],
        svec![test_url!("user/Hefty"), "Hefty Smurf"],
        svec![test_url!("user/Greedy"), "Greedy Smurf"],
        svec![test_url!("user/Jokey"), "Jokey Smurf"],
        svec![test_url!("user/Chef"), "Chef Smurf"],
        svec![test_url!("user/Vanity"), "Vanity Smurf"],
        svec![test_url!("user/Handy"), "Handy Smurf"],
        svec![test_url!("user/Scaredy"), "Scaredy Smurf"],
        svec![test_url!("user/Tracker"), "Tracker Smurf"],
        svec![test_url!("user/Sloppy"), "Sloppy Smurf"],
        svec![test_url!("user/Harmony"), "Harmony Smurf"],
        svec![test_url!("user/Painter"), "Painter Smurf"],
        svec![test_url!("user/Poet"), "Poet Smurf"],
        svec![test_url!("user/Farmer"), "Farmer Smurf"],
        svec![test_url!("user/Natural"), "Natural Smurf"],
        svec![test_url!("user/Snappy"), "Snappy Smurf"],
        svec![
            test_url!(
                "user/The quick brown fox jumped over the lazy dog by the zigzag quarry site"
            ),
            "The quick brown fox jumped over the lazy dog by the zigzag quarry site Smurf"
        ],
    ];
    assert_eq!(got, expected);

    // init stop webserver and wait until server gracefully exit
    println!("STOPPING Webserver");
    rt::System::new().block_on(server_handle.stop(true));
}
