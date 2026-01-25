use alloy::{
    primitives::{Address},
    providers::Provider,
    sol,  
};
use std::str::FromStr;
use crate::connection::provider::{init_avax_mainnet, init_avax_testnet};




pub async fn get_native_balance(wallet: &str) -> String{

    let provider = init_avax_mainnet().await;

    let wallet_addr = Address::from_str(wallet).expect("REASON");
    let decimal_balance:f32 = provider.get_balance(wallet_addr).await.unwrap().into();
    let power:f32 = 10.0_f32.powi(18).into();
    let display_balance:f32 = decimal_balance/power;




    return format!("The address{} has a Avax native token balance of {}",wallet,display_balance);

}