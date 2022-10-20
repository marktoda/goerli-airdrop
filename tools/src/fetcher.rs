use crossbeam::channel::{Receiver, Sender};
use ethers::prelude::*;
use ethers_providers::{Http, Provider};

pub async fn load_creators_from_blocks(
    rpc_url: &str,
    block_provider: Receiver<u64>,
    sender: Sender<(String, u64)>,
) -> () {
    let provider = Provider::<Http>::try_from(rpc_url).expect("Could not create provider");
    for block_num in block_provider.iter() {
        if block_num % 100 == 0 {
            println!("Fetching creators for block {}", block_num);
        }
        let block_result = provider.get_block_with_txs(block_num).await;
        if let Ok(Some(block)) = block_result {
            block
                .transactions
                .iter()
                .filter(|transaction| transaction.to.is_none())
                .map(|transaction| transaction.recover_from())
                .filter_map(|result| result.ok())
                .map(|result| format!("{:x}", result))
                .for_each(|c| sender.send((c, block_num)).expect("Channel failed"));
        } else {
            println!("Error fetching block: {:?}", block_result);
        }
    }
}
