extern crate rand;
extern crate reqwest;

use rand::Rng;
use rouille::Response;
use std::env;

type Unwraper<T> = Result<T, Box<dyn std::error::Error>>;

fn main() {
    let mut args = env::args();
    let exe = args.next();
    let port = args.next();
    if port.is_none() {
        println!("{}", format!("usage: {} port", exe.unwrap()));
        return;
    }
    let port = port.unwrap().trim().parse::<f64>();
    if port.is_err() {
        println!("Bad port number");
        return;
    }

    let address = format!("127.0.0.1:{}", port.unwrap());
    rouille::start_server(address, move |_| {
        Response::text(joke_or_insult().unwrap_or("".to_string()))
    });
}

fn joke_or_insult() -> Unwraper<String> {
    let mut rng = rand::thread_rng();
    let rdm = rng.gen_range(0, 2);
    let res = if rdm == 0 {
        let client = reqwest::Client::new();
        client
            .get("https://icanhazdadjoke.com/")
            .header("Accept", "text/plain")
            .send()?
            .text()?
    } else {
        reqwest::get("https://insult.mattbas.org/api/insult")?.text()?
    }
    .to_string();

    Ok(res)
}
