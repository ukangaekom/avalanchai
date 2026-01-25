use alloy::{providers::Provider,
primitives::Address,sol};
use regex::Regex;
use once_cell::sync::Lazy;


sol! {
    #[sol(rpc)]
    interface IERC165 {
        function supportsInterface(bytes4 interfaceId) external view returns (bool);
    }
}


// REGEX
static RE: Lazy<Regex> = Lazy::new(||Regex::new(r"\[([^\]]+)]").unwrap());


pub fn destructor_task(response_text: &str) -> Vec<String>{

    let tool_calls: Vec<String> = RE.find_iter(response_text)
        .map(|caps| {
            caps.as_str()
            .chars().filter(|&c| c != '\n' && c != ' ')
            .collect()})
        .collect();

    return tool_calls;

}



// FUNCTION IMPLEMENTATIONS


pub fn extract_tool_params(input: &str) -> Option<(String, Vec<String>)> {
    // Find the part inside the brackets
    let start = input.find('[')?;
    let end = input.find(']')?;

    // Extract the inside and split by comma
    let content = &input[start + 1..end];
    let mut parts = content.split(',').map(|s| s.trim().trim_matches('"')).filter(|s| !s.is_empty());

    let first = parts.next()?.to_string();
    let rest = parts.map(|s| s.to_string()).collect();

    Some((first, rest))
}


// NFT VERIFYING FUNCTIONS

pub async fn is_erc721_nft_contract(provider: &impl Provider, addr: Address) -> bool {
    let erc165 = IERC165::new(addr, provider);

    if let Ok(result) = erc165.supportsInterface([0x80, 0xac, 0x58, 0xcd].into()).call().await {
        if result { return true; }
    }
    false
}


pub async fn is_erc1155_nft_contract(provider: &impl Provider, addr: Address) -> bool {
    let erc165 = IERC165::new(addr, provider);
    if let Ok(result) = erc165.supportsInterface([0xd9, 0xb6, 0x7a, 0x26].into()).call().await {
        if result { return true; }
    }
    false
}
