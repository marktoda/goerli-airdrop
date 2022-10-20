pub use distributor::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod distributor {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "Distributor was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static DISTRIBUTOR_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"addresses\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountPerAddress\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"distribute\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static DISTRIBUTOR_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b5061024b806100206000396000f3fe60806040526004361061001e5760003560e01c80631826c11914610023575b600080fd5b61003661003136600461010d565b610038565b005b60005b82518110156100a957828181518110610056576100566101d8565b60200260200101516001600160a01b03166108fc839081150290604051600060405180830381858888f19350505050158015610096573d6000803e3d6000fd5b50806100a1816101ee565b91505061003b565b5060405133904780156108fc02916000818181858888f193505050501580156100d6573d6000803e3d6000fd5b505050565b634e487b7160e01b600052604160045260246000fd5b80356001600160a01b038116811461010857600080fd5b919050565b6000806040838503121561012057600080fd5b823567ffffffffffffffff8082111561013857600080fd5b818501915085601f83011261014c57600080fd5b8135602082821115610160576101606100db565b8160051b604051601f19603f83011681018181108682111715610185576101856100db565b6040529283528183019350848101820192898411156101a357600080fd5b948201945b838610156101c8576101b9866100f1565b855294820194938201936101a8565b9997909101359750505050505050565b634e487b7160e01b600052603260045260246000fd5b60006001820161020e57634e487b7160e01b600052601160045260246000fd5b506001019056fea26469706673582212205d42bd55839f2efaadf4e3d82ef647a1736828c88f64c913eeb6c5d0037e6c9364736f6c63430008110033" . parse () . expect ("invalid bytecode")
        });
    pub struct Distributor<M>(ethers::contract::Contract<M>);
    impl<M> Clone for Distributor<M> {
        fn clone(&self) -> Self {
            Distributor(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Distributor<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for Distributor<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Distributor))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> Distributor<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), DISTRIBUTOR_ABI.clone(), client).into()
        }
        #[doc = r" Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it."]
        #[doc = r" Returns a new instance of a deployer that returns an instance of this contract after sending the transaction"]
        #[doc = r""]
        #[doc = r" Notes:"]
        #[doc = r" 1. If there are no constructor arguments, you should pass `()` as the argument."]
        #[doc = r" 1. The default poll duration is 7 seconds."]
        #[doc = r" 1. The default number of confirmations is 1 block."]
        #[doc = r""]
        #[doc = r""]
        #[doc = r" # Example"]
        #[doc = r""]
        #[doc = r" Generate contract bindings with `abigen!` and deploy a new contract instance."]
        #[doc = r""]
        #[doc = r" *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact."]
        #[doc = r""]
        #[doc = r" ```ignore"]
        #[doc = r" # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {"]
        #[doc = r#"     abigen!(Greeter,"../greeter.json");"#]
        #[doc = r""]
        #[doc = r#"    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();"#]
        #[doc = r"    let msg = greeter_contract.greet().call().await.unwrap();"]
        #[doc = r" # }"]
        #[doc = r" ```"]
        pub fn deploy<T: ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::std::result::Result<
            ethers::contract::builders::ContractDeployer<M, Self>,
            ethers::contract::ContractError<M>,
        > {
            let factory = ethers::contract::ContractFactory::new(
                DISTRIBUTOR_ABI.clone(),
                DISTRIBUTOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `distribute` (0x1826c119) function"]
        pub fn distribute(
            &self,
            addresses: ::std::vec::Vec<ethers::core::types::Address>,
            amount_per_address: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([24, 38, 193, 25], (addresses, amount_per_address))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Distributor<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `distribute` function with signature `distribute(address[],uint256)` and selector `[24, 38, 193, 25]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "distribute", abi = "distribute(address[],uint256)")]
    pub struct DistributeCall {
        pub addresses: ::std::vec::Vec<ethers::core::types::Address>,
        pub amount_per_address: ethers::core::types::U256,
    }
}
