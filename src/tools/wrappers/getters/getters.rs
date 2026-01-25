use crate::services::getter::{market::{get_marketcap, get_price},
    token::{get_token_balance_mainnet, get_token_details_mainnet},
    nft::{get_nft_balance_mainnet, get_nft_details_mainnet, get_nft_total_supply_mainnet},
    wallet::{get_native_balance}
};
use futures::future::join_all;




// ============================= PRICES ===================================

pub async fn get_price_wrapper(args:&[&str])-> String{
    // Spawn all futures at once
    let futures = args.iter().map(|&token| get_price(token));

    // Run them concurrently and collect results
    let results: Vec<String> = join_all(futures).await;
    results.join(", ")

}



pub async fn get_marketcap_wrapper(args: &[&str])-> String{
    // Spawn all futures at once
    let futures = args.iter().map(|&token| get_marketcap(token));

    // Run them concurrently and collect results
    let results: Vec<String> = join_all(futures).await;
    results.join(", ")
    
}



// ==================== TOKENS ===================================



pub async fn get_token_details_mainnet_wrapper(args: &[&str]) -> String{
    //Spawn all futures at once
    let futures = args.iter().map(|&token| get_token_details_mainnet(token));

    // Run them concurrently and collect results
    let results: Vec<String> = join_all(futures).await;
    results.join(", ")
    
}


pub async fn get_token_balance_mainnet_wrapper(args:&[&str]) -> String{

    let token = args[0];
    let futures = args.iter().skip(1)
        .map(|&wallet| get_token_balance_mainnet(token,wallet));
    let results: Vec<String> = join_all(futures).await;

    results.join(", ")
    
}

pub async fn get_native_balance_wrapper(args:&[&str])-> String{

    //Spawn all futures at once
    let futures = args.iter().map(|&token| get_native_balance(token));

    // Run them concurrently and collect results
    let results: Vec<String> = join_all(futures).await;
    results.join(", ")

}



// ==================== NFT ==========================================

pub async fn get_nft_balance_mainnet_wrapper(args:&[&str]) -> String{
    //Spawn all futures at once
    let token = args[0];
    let futures = args.iter().skip(1)
        .map(|&wallet| get_nft_balance_mainnet(token,wallet));

    let results: Vec<String> = join_all(futures).await;

    results.join(", ")
}



pub async fn get_nft_details_mainnet_wrapper(args:&[&str])-> String{
    //Spawn all futures at once
    let futures = args.iter().map(|&token| get_nft_details_mainnet(token));

    // Run them concurrently and collect results
    let results: Vec<String> = join_all(futures).await;
    results.join(", ")

}


pub async fn get_nft_total_supply_mainnet_wrapper(args:&[&str])->String{
    //Spawn all futures at once
    let futures = args.iter().map(|&token| get_nft_total_supply_mainnet(token));

    // Run them concurrently and collect results
    let results: Vec<String> = join_all(futures).await;
    results.join(", ")

}