use alloy::{
    primitives::{Address},
    providers::Provider,
    sol,  
};
use std::str::FromStr;
use crate::connection::provider::{init_avax_mainnet, init_avax_testnet};








//  Aave Pool

// Contracts
sol!(
    #[sol(rpc)]
    contract  aave_pool{

        function getWETHAddress() external view returns (address) ;
        function transfer(address to, uint256 amount) external returns (bool);
        function allowance(address owner, address spender) external view returns (uint256);
        function approve(address spender, uint256 amount) external returns (bool);
        function transferFrom(address from, address to, uint256 amount) external returns (bool);
        function name() returns (string);
        function symbol() external view returns (string memory);
        function totalSupply() external view returns (uint256);
        function decimals() external view returns (uint8);

    }
);


// Aave Wrapped Token Gateway
// Contracts 0x2825cE5921538d17cc15Ae00a8B24fF759C6CDaE
sol!(
    #[sol(rpc)]
    contract  AAVE_WRAPPED_TOKEN_GATEWAY{

        function getWETHAddress() external view returns (address);

        function WETH() external view returns (address);

        function POOL() external view returns (address);

        function owner() external view returns (address);
        

    }
);


// functions

pub async fn aave_wrapped_token_get_wrappedethaddress_token(gateway_address: &str) -> String {

    let provider = init_avax_mainnet().await;
    let gateway = Address::from_str(gateway_address).expect("REASON");
    let aave_gatway = AAVE_WRAPPED_TOKEN_GATEWAY::new(gateway,provider.clone());
    let wrapped_address_token = aave_gatway.getWETHAddress().call().await;

    match wrapped_address_token{
        Ok(wrapped_address_token) =>{

              return format!("The the wrapped eth token address of the aave token gateway is {:#?}",&wrapped_address_token);

        }
        Err(_) =>{

            return format!("I think there is a problem trying to know the Wrapped Eth address of aave token gateway")

        }

    }



}


pub async fn aave_wrapped_token_get_weth_address(gateway_address: &str) -> String{
    let provider = init_avax_mainnet().await; 
    let gateway = Address::from_str(gateway_address).expect("REASON");
    let aave_gatway = AAVE_WRAPPED_TOKEN_GATEWAY::new(gateway,provider.clone());
    let weth_address = aave_gatway.WETH().call().await;
    match weth_address{
        Ok(weth_address) =>{

             return format!("The Wrapped Eth address of the aave token gateway is {:#?}",weth_address);

        }
        Err(_) =>{

            return format!("I think there is a problem trying to know the WETH of aave token gateway");
            
        }

    }

    

}

pub async fn aave_wrapped_token_get_pool_address(gateway_address: &str) -> String{
    let provider = init_avax_mainnet().await; 
    let gateway = Address::from_str(gateway_address).expect("REASON");
    let aave_gatway = AAVE_WRAPPED_TOKEN_GATEWAY::new(gateway, provider.clone());
    let pool = aave_gatway.POOL().call().await;

    match pool{
        Ok(pool) =>{

            return format!("The pool address of the aave token gateway is {:#?}",pool);

        }
        Err(_) =>{
            return format!("I think there is a problem trying to know the pool address of aave gateway");
            
        }

    }

   
}

pub async fn aave_wrapped_token_get_owner(gateway_address: &str) -> String{
    let provider = init_avax_mainnet().await; 
    let gateway = Address::from_str(gateway_address).expect("REASON");
    let aave_gatway = AAVE_WRAPPED_TOKEN_GATEWAY::new(gateway,provider.clone());
    let owner = aave_gatway.owner().call().await;

    match owner{
        Ok(owner) =>{
            return format!("The owner of the aave token gateway is {:#?}",&owner);

        }
        Err(_) =>{
             return format!("I think there is a problem trying to know the owner of aave token gateway");
            
        }

    }



}





// Aave Oracle



// Aave Pool Configuration



// Aave Wallet Balance Provider


// Aave Treasury Collector



// Aave Collateral Switch