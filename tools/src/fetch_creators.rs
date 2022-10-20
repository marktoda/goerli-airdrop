use std::fs::File;
use std::io::Read;
use std::path::Path;
use structopt::StructOpt;

mod opt;
use crate::opt::Opt;
mod fetcher;
use crate::fetcher::load_creators_from_blocks;
mod util;
use crate::util::load_creator_map;

// num workers
const NUM_WORKERS: usize = 10;
const FILE_NAME: &str = "../data/creators.json";
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

    let mut creator_map = load_creator_map(FILE_NAME);
    println!("Loaded {} creators", creator_map.keys().len());
    let mut last_block_num = get_start_block(&opt);
    loop {
        let (creator, block_num) = receiver.recv().expect("Channel failed");
        let count = creator_map.get(&creator).unwrap_or(&0);
        creator_map.insert(creator, count + 1);

        if block_num > last_block_num + 10000 {
            std::fs::write("block_num.txt", block_num.to_string())
                .expect("Could not write to file");

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
