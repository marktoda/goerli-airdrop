use structopt::StructOpt;
use ethers::prelude::*;
use ethers_providers::{Http, Provider};
use ethers::types::{U256, H160};
use std::collections::HashMap;
use std::str::FromStr;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

mod opt;
use crate::opt::Opt;

// num workers
const FILE_NAME: &str = "creators.json";

// some false positives for some reason.. filter by tx count
#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    let creator_map = load_creator_map();
    let provider = Provider::<Http>::try_from(opt.rpc_url).expect("Could not create provider");
    let u256_zero = U256::from_dec_str("0").unwrap();

    let mut filtered_map: HashMap<String, usize> = HashMap::new();
    for (creator, count) in creator_map {
        // fetch the tx count of the creator address
        let tx_count = provider.get_transaction_count(H160::from_str(&creator).expect("Could not parse address"), None).await.expect("Could not fetch tx count");
        println!("{}: {}", creator, tx_count);
        if tx_count > u256_zero {
            println!("Found one lt0");
            filtered_map.insert(creator, count);
        }
    }

    let file = File::create(FILE_NAME).expect("Could not create file");
    serde_json::to_writer_pretty(file, &filtered_map).expect("Could not write to file");
}

pub fn load_creator_map() -> HashMap<String, usize> {
    let path = Path::new(FILE_NAME);
    if path.exists() {
        // Open the file in read-only mode with buffer.
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        // Read the JSON contents of the file as an instance of `User`.
        serde_json::from_reader(reader).unwrap()
    } else {
        HashMap::new()
    }
}
