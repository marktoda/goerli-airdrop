// structopt for command line arguments

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "goerli-airdrop")]
pub struct Opt {
    /// The rpc url to fetch data from
    #[structopt(short, long, env = "RPC_URL")]
    pub rpc_url: String,

    /// The block to start searching from
    #[structopt(short, long, default_value = "0")]
    pub from_block: u64,

    /// The block to search until
    /// default date is the first merged block
    #[structopt(short, long, default_value = "7382818")]
    pub to_block: u64,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "distributor")]
pub struct DistributorOpt {
    /// The rpc url to fetch data from
    #[structopt(short, long, env = "RPC_URL")]
    pub rpc_url: String,

    /// The private key to distribute from
    #[structopt(short, long)]
    pub private_key: String,
}
