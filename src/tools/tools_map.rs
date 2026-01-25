use crate::tools::wrappers::getters::getters::*;
use std::collections::HashMap;
use std::pin::Pin;
use std::future::Future;
use once_cell::sync::Lazy;
use std::sync::Arc;


type AsyncFn = Arc<
    dyn for<'a> Fn(&'a [&'a str]) -> Pin<Box<dyn Future<Output = String> + Send + 'a>>
        + Send
        + Sync,
>;


pub static TOOLS: Lazy<HashMap<&'static str, AsyncFn>> = Lazy::new(||{
    let mut m: HashMap<&'static str, AsyncFn> = HashMap::new();
    m.insert("get_price", Arc::new(|args| Box::pin(get_price_wrapper(args))));
    m.insert("get_marketcap", Arc::new(|args| Box::pin(get_marketcap_wrapper(args))));
    m.insert("get_native_balance", Arc::new(|args| Box::pin(get_native_balance_wrapper(args))));
    m.insert("get_token_details_mainnet", Arc::new(|args| Box::pin(get_token_details_mainnet_wrapper(args))));
    m.insert("get_token_balance_mainnet", Arc::new(|args| Box::pin(get_token_balance_mainnet_wrapper(args))));
    m.insert("get_nft_balance_mainnet", Arc::new(|args| Box::pin(get_nft_balance_mainnet_wrapper(args))));
    m.insert("get_nft_details_mainnet", Arc::new(|args| Box::pin(get_nft_details_mainnet_wrapper(args))));
    m.insert("get_nft_total_supply_mainnet", Arc::new(|args| Box::pin(get_nft_total_supply_mainnet_wrapper(args))));
    m
});
