use alloy::providers::{ProviderBuilder, DynProvider, Provider};
use tokio::sync::{OnceCell};
use once_cell::sync::Lazy;


// ---------- STATICS ----------
static ALCHEMY_API_KEY: Lazy<String> = Lazy::new(||{
    std::env::var("ALCHEMY_API_KEY").expect("ALCHEMY_API_KEY must be set")
});

// ---------- Getting API Keys ----------
fn get_alchemy_api_key() -> &'static str {
    &ALCHEMY_API_KEY
}



static _AVAX_TESTNET: OnceCell<DynProvider> = OnceCell::const_new();
static _AVAX_MAINNET: OnceCell<DynProvider> = OnceCell::const_new();


// ---------- Providers ----------
pub async fn init_avax_mainnet() -> &'static DynProvider {
    _AVAX_MAINNET.get_or_init(||async{
        ProviderBuilder::new()
        .connect(format!(
            "https://avax-mainnet.g.alchemy.com/v2/{}",
            get_alchemy_api_key()
        ).as_str())
        .await
        .expect("RPC init failed")
        .erased()
    }).await
}




pub async fn init_avax_testnet()-> &'static DynProvider {
    _AVAX_TESTNET.get_or_init(||async{
        ProviderBuilder::new()
        .connect(format!(
            "https://avax-fuji.g.alchemy.com/v2/{}",
            get_alchemy_api_key()
        ).as_str())
        .await
        .expect("RPC init failed")
        .erased()
    }).await

    
}



