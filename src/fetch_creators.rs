use structopt::StructOpt;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::io::Read;

mod opt;
use crate::opt::Opt;
mod fetcher;
use crate::fetcher::load_creators_from_blocks;

// num workers
const NUM_WORKERS: usize = 10;
const FILE_NAME: &str = "creators.json";
const LAST_BLOCK_FILE_NAME: &str = "block_num.txt";

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    let (sender, receiver) = crossbeam_channel::unbounded();
    let (block_sender, block_receiver) = crossbeam_channel::unbounded();

    // load blocks
    for block_num in get_start_block(&opt)..opt.to_block {
        block_sender.send(block_num).expect("Channel failed");
    }

    // spawn workers
    for _ in 0..NUM_WORKERS {
        let block_receiver = block_receiver.clone();
        let sender = sender.clone();
        let rpc_url = opt.rpc_url.clone();
        tokio::spawn(async move {
            load_creators_from_blocks(&rpc_url, block_receiver, sender).await;
        });
    }

    let mut creator_map = load_creator_map();
    println!("Loaded {} creators", creator_map.keys().len());
    let mut last_block_num = get_start_block(&opt);
    loop {
        let (creator, block_num) = receiver.recv().expect("Channel failed");
        let count = creator_map.get(&creator).unwrap_or(&0);
        creator_map.insert(creator, count + 1);

        if block_num > last_block_num + 10000 {
            std::fs::write("block_num.txt", block_num.to_string()).expect("Could not write to file");

            let file = File::create(FILE_NAME).expect("Could not create file");
            serde_json::to_writer_pretty(file, &creator_map).expect("Could not write to file");

            last_block_num = block_num;
        }
    }
}

fn get_start_block(opt: &Opt) -> u64 {
    if Path::new(LAST_BLOCK_FILE_NAME).exists() {
        let mut file = File::open(LAST_BLOCK_FILE_NAME).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        println!("Loaded last block from file: {}", data);
        data.parse::<u64>().unwrap()
    } else {
        opt.from_block
    }
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
