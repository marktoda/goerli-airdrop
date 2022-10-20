use anyhow::{anyhow, Result};
use futures::future::join_all;
use ethers::abi::Abi;
use ethers_contract::Contract;
use ethers::core::k256::ecdsa::SigningKey;
use ethers_middleware::SignerMiddleware;
use ethers::prelude::*;
use ethers::types::{H160, U256, transaction::eip2718::TypedTransaction};
use ethers_providers::{Http, Provider};
use std::collections::HashMap;
use std::str::FromStr;
use structopt::StructOpt;

mod opt;
use crate::opt::DistributorOpt;
mod util;
use crate::util::load_creator_map;

const FILE_NAME: &str = "../data/creators.json";
const CHUNK_SIZE: usize = 100;
// broadcast txs in chunks
const BROADCAST_CHUNK_SIZE: usize = 5;
const DISTRIBUTOR_ADDRESS: &str = "0xe2a453EAc17001f311F642976509E8C502138756";
const ABI_STR: &str = r#"[
    {
      "inputs": [
        {
          "internalType": "address[]",
          "name": "addresses",
          "type": "address[]"
        },
        {
          "internalType": "uint256",
          "name": "amountPerAddress",
          "type": "uint256"
        }
      ],
      "name": "distribute",
      "outputs": [],
      "stateMutability": "payable",
      "type": "function"
    }
  ]"#;

// some false positives for some reason.. filter by tx count
#[tokio::main]
async fn main() {
    let opt = DistributorOpt::from_args();
    let creator_map = load_creator_map(FILE_NAME);

    let low_chunks = chunk_creators(&creator_map, 0, 1);
    let med_chunks = chunk_creators(&creator_map, 2, 9);
    let high_chunks = chunk_creators(&creator_map, 10, std::usize::MAX);

    let wallet = Wallet::from_str(&opt.private_key).expect("Invalid private key").with_chain_id(5u64);
    let provider = Provider::<Http>::try_from(opt.rpc_url).expect("Could not create provider");
    // 1000 eth
    let high_distribution: U256 = U256::from_dec_str("1000000000000000000000").unwrap();
    // 100 eth
    let med_distribution: U256 = U256::from_dec_str("100000000000000000000").unwrap();
    // 10 eth
    let low_distribution: U256 = U256::from_dec_str("10000000000000000000").unwrap();

    println!("Distributing to high addresses");
    distribute_chunks(
        high_chunks,
        high_distribution,
        wallet.clone(),
        provider.clone(),
    )
    .await
    .expect("failed to distribute");

    println!("Distributing to medium addresses");
    distribute_chunks(
        med_chunks,
        med_distribution,
        wallet.clone(),
        provider.clone(),
    )
    .await
    .expect("failed to distribute");

    println!("Distributing to low addresses");
    distribute_chunks(
        low_chunks,
        low_distribution,
        wallet.clone(),
        provider.clone(),
    )
    .await
    .expect("failed to distribute");

}

// chunk the creators up by CHUNK_SIZE
fn chunk_creators(
    creator_map: &HashMap<String, usize>,
    gte: usize,
    lte: usize,
) -> Vec<Vec<String>> {
    creator_map
        .into_iter()
        .filter(|(_, count)| **count >= gte && **count <= lte)
        .map(|(address, _count)| address.clone())
        .collect::<Vec<String>>()
        .chunks(CHUNK_SIZE)
        .into_iter()
        .map(|x| x.to_vec())
        .collect()
}

// distribute to each chunk
async fn distribute_chunks(
    chunks: Vec<Vec<String>>,
    amount: U256,
    wallet: Wallet<SigningKey>,
    provider: Provider<Http>,
) -> Result<()> {
    let abi: Abi = serde_json::from_str(ABI_STR)?;
    let client = SignerMiddleware::new(provider.clone(), wallet.clone());
    let contract = Contract::new(H160::from_str(DISTRIBUTOR_ADDRESS).unwrap(), abi, &client);
    let mut nonce = provider.get_transaction_count(client.address(), None).await?;

    let (base_fee, _) = provider.estimate_eip1559_fees(None).await?;
    let txs: Vec<TypedTransaction> = chunks.iter().map(|chunk| {
        let addresses: Vec<H160> = chunk
            .iter()
            .map(|x| H160::from_str(x).expect("Invalid address"))
            .collect();

        let mut tx = contract
            .method::<_, ()>("distribute", (addresses.clone(), amount)).expect("method does not exist")
            .value(amount * U256::from(addresses.len() as u64))
            .from(client.address())
            .gas_price(base_fee * U256::from_dec_str("2").unwrap())
            .gas(10000000u64).tx;
        tx
            .set_nonce(nonce)
            .set_chain_id(5u64);

        nonce += U256::from_dec_str("1").unwrap();
        tx

    })
    .collect();
    for chunk in txs.chunks(BROADCAST_CHUNK_SIZE) {
        println!("Broadcasting {} distribution txs", chunk.len());
        let sent_txs = join_all(chunk.iter().map(|tx| broadcast(&client, tx))).await;
        for tx in sent_txs {
            println!("Sent tx: {:?}", tx);
        }
    }

    Ok(())
}

async fn broadcast<T, U>(
    signer: &SignerMiddleware<T, U>,
    tx: &TypedTransaction,
) -> Result<TxHash>
    where
    T: Middleware,
    U: Signer,
{
    // Signing manually so we skip `fill_transaction` and its `eth_createAccessList` request.
    let signature = signer
        .sign_transaction(
            &tx,
            *tx.from().expect("Tx should have a `from`."))
        .await
        .map_err(|err| anyhow!(err.to_string()))?;
    println!("Broadcasting tx: {:?}", tx.hash(&signature));

    // Submit the raw transaction
    let tx = signer
        .provider()
        .send_raw_transaction(tx.rlp_signed(&signature))
        .await
        .map_err(|err| anyhow!(err.to_string()))?
        .await
        .map_err(|err| anyhow!(err.to_string()))?
        .ok_or(anyhow!("No transaction hash returned"))?;

    Ok(tx.transaction_hash)
}
