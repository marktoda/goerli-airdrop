use ethers::prelude::*;
use ethers::types::{H160, U256};
use ethers_providers::{Http, Provider};
use std::collections::HashMap;
use std::fs::File;
use std::str::FromStr;
use structopt::StructOpt;

mod opt;
use crate::opt::Opt;
mod util;
use crate::util::load_creator_map;

// num workers
const FILE_NAME: &str = "../data/creators.json";

// some false positives for some reason.. filter by tx count
#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    let creator_map = load_creator_map(FILE_NAME);
    let provider = Provider::<Http>::try_from(opt.rpc_url).expect("Could not create provider");
    let one_ether = U256::from_dec_str("1000000000000000000").unwrap();

    let mut filtered_map: HashMap<String, usize> = HashMap::new();
    let mut i = 0;
    for (creator, count) in creator_map.clone() {
        if i % 100 == 0 {
            println!("{} / {}", i, &creator_map.len());
        }
        // fetch the creator balance
        let balance = provider
            .get_balance(
                H160::from_str(&creator).expect("Could not parse address"),
                None,
            )
            .await
            .expect("Could not fetch balance");

        if balance < one_ether {
            filtered_map.insert(creator, count);
        }
        i += 1;
    }

    let file = File::create(FILE_NAME).expect("Could not create file");
    serde_json::to_writer_pretty(file, &filtered_map).expect("Could not write to file");
}
