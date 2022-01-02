use crate::workdir::Workdir;

#[test]
fn fetch_simple() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL"],
            svec!["http://api.zippopotam.us/us/90210"],
            svec!["https://api.zippopotam.us/us/94105"],
            svec!["http://api.zippopotam.us/us/92802"],
            svec!["https://query.wikidata.org/sparql?query=SELECT%20?dob%20WHERE%20{wd:Q42%20wdt:P569%20?dob.}&format=json"],
        ],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            r#"{"post code": "90210", "country": "United States", "country abbreviation": "US", "places": [{"place name": "Beverly Hills", "longitude": "-118.4065", "state": "California", "state abbreviation": "CA", "latitude": "34.0901"}]}"#
        ],
        svec![
            r#"{"post code": "94105", "country": "United States", "country abbreviation": "US", "places": [{"place name": "San Francisco", "longitude": "-122.3892", "state": "California", "state abbreviation": "CA", "latitude": "37.7864"}]}"#
        ],
        svec![
            r#"{"post code": "92802", "country": "United States", "country abbreviation": "US", "places": [{"place name": "Anaheim", "longitude": "-117.9228", "state": "California", "state abbreviation": "CA", "latitude": "33.8085"}]}"#
        ],
        svec![
            r#"{
  "head" : {
    "vars" : [ "dob" ]
  },
  "results" : {
    "bindings" : [ {
      "dob" : {
        "datatype" : "http://www.w3.org/2001/XMLSchema#dateTime",
        "type" : "literal",
        "value" : "1952-03-11T00:00:00Z"
      }
    } ]
  }
}"#
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
        svec!["http://api.zippopotam.us/us/90210", "\"Beverly Hills, CA\""],
        svec!["http://api.zippopotam.us/us/94105", "\"San Francisco, CA\""],
        svec!["https://api.zippopotam.us/us/92802", "\"Anaheim, CA\""],
    ];
    assert_eq!(got, expected);
}

use std::{sync::mpsc, thread};

use actix_web::{
    dev::Server, middleware, rt, web, App, HttpRequest, HttpServer, Responder, Result,
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
/// returns Smurf fullname
async fn get_fullname(
    req: HttpRequest,
    web::Path((name,)): web::Path<(String,)>,
) -> Result<impl Responder> {
    println!("{:?}", req);

    let obj = MyObj {
        fullname: format!("{} Smurf", name),
    };

    Ok(web::Json(obj))
}

/// start an Actix Webserver with Rate Limiting via Governor
fn run_webserver(tx: mpsc::Sender<Server>) -> std::io::Result<()> {
    let mut sys = rt::System::new("test");

    use actix_governor::{Governor, GovernorConfig, GovernorConfigBuilder};

    // Allow bursts with up to five requests per IP address
    // and replenishes one element every 500 ms (2 qps)
    let governor_conf: GovernorConfig = GovernorConfigBuilder::default()
        .per_millisecond(500)
        .burst_size(5)
        .finish()
        .unwrap();

    // srv is server controller type, `dev::Server`
    let srv = HttpServer::new(move || {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .wrap(Governor::new(&governor_conf))
            .service(web::resource("/user/{name}").route(web::get().to(get_fullname)))
            .service(web::resource("/").to(index))
    })
    .bind("127.0.0.1:8080")?
    .run();

    // send server controller to main thread
    let _ = tx.send(srv.clone());

    // run future
    sys.block_on(srv)
}

#[test]
fn fetch_ratelimit() {
    // start webserver with rate limiting
    let (tx, rx) = mpsc::channel();

    println!("START Webserver ");
    thread::spawn(move || {
        let _ = run_webserver(tx);
    });

    let srv = rx.recv().unwrap();

    // proceed with usual unit test
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL"],
            svec!["http://localhost:8080/user/Smurfette"],
            svec!["http://localhost:8080/user/Papa"],
            svec!["http://localhost:8080/user/Clumsy"],
            svec!["http://localhost:8080/user/Brainy"],
            svec!["http://localhost:8080/user/Grouchy"],
            svec!["http://localhost:8080/user/Hefty"],
            svec!["http://localhost:8080/user/Greedy"],
            svec!["http://localhost:8080/user/Jokey"],
            svec!["http://localhost:8080/user/Chef"],
            svec!["http://localhost:8080/user/Vanity"],
            svec!["http://localhost:8080/user/Handy"],
            svec!["http://localhost:8080/user/Scaredy"],
            svec!["http://localhost:8080/user/Tracker"],
            svec!["http://localhost:8080/user/Sloppy"],
            svec!["http://localhost:8080/user/Harmony"],
            svec!["http://localhost:8080/user/Painter"],
            svec!["http://localhost:8080/user/Poet"],
            svec!["http://localhost:8080/user/Farmer"],
            svec!["http://localhost:8080/user/Natural"],
            svec!["http://localhost:8080/user/Snappy"],
        ],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("--new-column")
        .arg("Fullname")
        .arg("--jql")
        .arg(r#"."fullname""#)
        .arg("--rate-limit")
        .arg("2")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["URL", "Fullname"],
        svec!["http://localhost:8080/user/Smurfette", "Smurfette Smurf"],
        svec!["http://localhost:8080/user/Papa", "Papa Smurf"],
        svec!["http://localhost:8080/user/Clumsy", "Clumsy Smurf"],
        svec!["http://localhost:8080/user/Brainy", "Brainy Smurf"],
        svec!["http://localhost:8080/user/Grouchy", "Grouchy Smurf"],
        svec!["http://localhost:8080/user/Hefty", "Hefty Smurf"],
        svec!["http://localhost:8080/user/Greedy", "Greedy Smurf"],
        svec!["http://localhost:8080/user/Jokey", "Jokey Smurf"],
        svec!["http://localhost:8080/user/Chef", "Chef Smurf"],
        svec!["http://localhost:8080/user/Vanity", "Vanity Smurf"],
        svec!["http://localhost:8080/user/Handy", "Handy Smurf"],
        svec!["http://localhost:8080/user/Scaredy", "Scaredy Smurf"],
        svec!["http://localhost:8080/user/Tracker", "Tracker Smurf"],
        svec!["http://localhost:8080/user/Sloppy", "Sloppy Smurf"],
        svec!["http://localhost:8080/user/Harmony", "Harmony Smurf"],
        svec!["http://localhost:8080/user/Painter", "Painter Smurf"],
        svec!["http://localhost:8080/user/Poet", "Poet Smurf"],
        svec!["http://localhost:8080/user/Farmer", "Farmer Smurf"],
        svec!["http://localhost:8080/user/Natural", "Natural Smurf"],
        svec!["http://localhost:8080/user/Snappy", "Snappy Smurf"],
    ];
    assert_eq!(got, expected);

    // init stop webserver and wait until server gracefully exit
    println!("STOPPING Webserver");
    rt::System::new("").block_on(srv.stop(true));
}
