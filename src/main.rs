use structopt::StructOpt;
mod opt;
use opt::Opt;
use std::collections::HashMap;
use std::fs::File;

mod fetcher;
use fetcher::load_creators_from_blocks;

// num workers
const NUM_WORKERS: usize = 10;
const FILE_NAME: &str = "creators.json";

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    let (sender, receiver) = crossbeam_channel::unbounded();
    let (block_sender, block_receiver) = crossbeam_channel::unbounded();

    // load blocks
    for block_num in opt.from_block..opt.to_block {
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

    let mut creator_map: HashMap<String, usize> = HashMap::new();
    loop {
        let creator = receiver.recv().expect("Channel failed");
        let count = creator_map.get(&creator).unwrap_or(&0);
        creator_map.insert(creator, count + 1);

        let file = File::create(FILE_NAME).expect("Could not create file");
        serde_json::to_writer_pretty(file, &creator_map).expect("Could not write to file");
    }
}
