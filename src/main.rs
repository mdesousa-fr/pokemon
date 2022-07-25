use std::time::Instant;
use reqwest::Client;
use rayon::prelude::*;
use clap::{Arg, Command};
use serde::Deserialize;
use rand::Rng;

fn main() {
    let command = Command::new("Pokemon catcher")
        .author("Mathieu DE SOUSA <mathieu.desousa@proton.me>")
        .version("0.1.0")
        .about("Catch a pokemon")
        .arg(Arg::with_name("number")
            .short('n')
            .long("number")
            .takes_value(true)
            .value_parser(clap::value_parser!(u16).range(1..800))
            .help("Pokemon number"))
        .get_matches();
    
    let number: u16 = *command.get_one("number").unwrap_or(&1);

    println!("--> Real multi thread");
    real_multi_threads(number);
}

fn real_multi_threads(x: u16) {
    let start = Instant::now();

    let mut data: Vec<u16> = Vec::new();

    println!("Number: {}", x);
    for i in 1..x+1 { data.push(i); }

    data.par_iter_mut().for_each(|_| {
        // let p = pokemon(&v);
        let p = Pokemon::get();
        println!("Got {}!", p.name);
    });

    let elapsed = start.elapsed();
    println!("Elapsed time: {:.2?}", elapsed);
}

#[derive(Deserialize)]
struct Pokemon {
    name: String,
}

impl Pokemon {
    #[tokio::main]
    async fn get() -> Pokemon {
        let client = Client::new();

        let rand_id: u16 = rand::thread_rng().gen_range(1..=800);

        let resp = client
            .get(format!("https://pokeapi.co/api/v2/pokemon/{}", rand_id))
            .send()
            .await
            .expect("failed to get response")
            .json::<Pokemon>()
            .await
            .expect("failed to get payload");
        
        return resp
    }
}
