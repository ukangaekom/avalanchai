use alloy::{
    primitives::{Address},
    providers::Provider,
    sol
};
use std::str::FromStr;
use crate::connection::provider::{init_avax_mainnet, init_avax_testnet};
use crate::tools::utils::{is_erc721_nft_contract, is_erc1155_nft_contract};




// NFT Standard Contract Interface
sol!(

    #[sol(rpc)]
    contract IERC721{

        function name() returns (string);
        function symbol() external view returns (string memory);
        function tokenURI(uint256 tokenId) external view returns (string memory);
        function totalSupply() external view returns (uint256);
        /// @notice Returns the number of NFTs owned by `owner`.
        function balanceOf(address owner) external view returns (uint256 balance);
        /// @notice Returns the owner of `tokenId`.
        function ownerOf(uint256 tokenId) external view returns (address owner);
        /// @notice Safely transfers `tokenId` from `from` to `to`, checking `to` can handle ERC721.
        function safeTransferFrom(address from, address to, uint256 tokenId) external;
        /// @notice Safe transfer with additional `data`.
        function safeTransferFrom(address from, address to, uint256 tokenId, bytes calldata data) external;
        /// @notice Transfers `tokenId` from `from` to `to` (no safety check).
        function transferFrom(address from, address to, uint256 tokenId) external;
        /// @notice Approves `to` to transfer `tokenId`.
        function approve(address to, uint256 tokenId) external;
        /// @notice Returns the approved address for `tokenId`.
        function getApproved(uint256 tokenId) external view returns (address operator);
        /// @notice Approve or remove `operator` as an operator for the caller.
        function setApprovalForAll(address operator, bool _approved) external;
        /// @notice Returns if `operator` is allowed to manage all of `owner`'s assets.
        function isApprovedForAll(address owner, address operator) external view returns (bool);
    
    }
);



pub async fn get_nft_total_supply_testnet(nft_address:&str)-> String {

    let provider = init_avax_testnet().await; 
    let token_addr = Address::from_str(nft_address).expect("REASON");

    let wallet = provider.get_code_at(token_addr).await;

    if !wallet.expect("REASONS").is_empty(){
        let erc721 = IERC721::new(token_addr,provider.clone());
        let name = erc721.name().call().await;
        let symbol = erc721.symbol().call().await;
        let totalsupply = erc721.totalSupply().call().await;

        format!("NFT collection name {:#?}, bearing the symbol: 
        {:#?} has a total supply of {:#?} on Avax Testnet",name,symbol,totalsupply)

    }else{

        format!("The address {}, is a Wallet address",nft_address)

    }

   

}



pub async fn get_nft_total_supply_mainnet(nft_address:&str) ->String  {
    
    let provider = init_avax_mainnet().await; 
    let token_addr = Address::from_str(nft_address).expect("REASON");

    let wallet = provider.get_code_at(token_addr).await;

    if !wallet.expect("REASONS").is_empty(){

        if is_erc721_nft_contract(&provider, token_addr).await {
            let erc721 = IERC721::new(token_addr,provider.clone());
            let name = erc721.name().call().await;
            let symbol = erc721.symbol().call().await;
            let totalsupply = erc721.totalSupply().call().await;
            format!("The ERC721 NFT collection name {:#?}, bearing the symbol:
         {:#?} has a total supply of {:#?}",name,symbol,totalsupply)

        } else if is_erc1155_nft_contract(&provider, token_addr).await {
            let erc721 = IERC721::new(token_addr,provider.clone());
            let name = erc721.name().call().await;
            let symbol = erc721.symbol().call().await;
            let totalsupply = erc721.totalSupply().call().await;
            format!("The ERC1155 NFT collection name {:#?}, bearing the symbol:
         {:#?} has a total supply of {:#?}",name,symbol,totalsupply)

        }else{
            format!("This contract address isn't a standard NFT")
        }
        
    }else{
        format!("The address {}, is a Wallet address",nft_address)

    }
    

}



pub async fn get_nft_details_testnet(nft_address:&str)-> String  {

    let provider = init_avax_testnet().await;  

    let token_addr = Address::from_str(nft_address).expect("REASON");


    let wallet = provider.get_code_at(token_addr).await;

    if !wallet.expect("REASONS").is_empty(){

        if is_erc721_nft_contract(&provider, token_addr).await{

            let erc721 = IERC721::new(token_addr,provider.clone());
            let name = erc721.name().call().await;
            let symbol = erc721.symbol().call().await;
            let totalsupply = erc721.totalSupply().call().await;

            format!("The ERC721 NFT name is {:#?} with name 
            {:#?} have a total supply of {:#?} on Avax Testnet",symbol, name,totalsupply )

        }else if is_erc1155_nft_contract(&provider, token_addr).await{

             let erc721 = IERC721::new(token_addr,provider.clone());
            let name = erc721.name().call().await;
            let symbol = erc721.symbol().call().await;
            let totalsupply = erc721.totalSupply().call().await;

            format!("The ERC1155 NFT name is {:#?} with name 
            {:#?} have a total supply of {:#?} on Avax Testnet",symbol, name,totalsupply)

        }else{
            format!("The contract address {} isn't a standard NFT", nft_address)
        }

        


    }else{

        format!("The address {}, is a Wallet address",nft_address)


    }
    

}



pub async fn get_nft_details_mainnet(nft_address:&str)-> String  {

    let provider = init_avax_mainnet().await;  

    let token_addr = Address::from_str(nft_address).expect("REASON");

    let wallet = provider.get_code_at(token_addr).await;

    if !wallet.expect("REASONS").is_empty(){

        if is_erc721_nft_contract(&provider, token_addr).await{

            let erc721 = IERC721::new(token_addr,provider.clone());
            let name = erc721.name().call().await;
            let symbol = erc721.symbol().call().await;
            let totalsupply = erc721.totalSupply().call().await;

            format!("The ERC721 NFT name is {:#?} with name 
            {:#?} have a total supply of {:#?} on Avax Testnet",symbol, name,totalsupply )

        }else if is_erc1155_nft_contract(&provider, token_addr).await{

            let erc721 = IERC721::new(token_addr,provider.clone());
            let name = erc721.name().call().await;
            let symbol = erc721.symbol().call().await;
            let totalsupply = erc721.totalSupply().call().await;

            format!("The ERC1155 NFT name is {:#?} with name 
            {:#?} have a total supply of {:#?} on Avax Testnet",symbol, name,totalsupply )

        }else{
            format!("The address isn't a standard NFT contract")
        }





    }else{

        format!("The address {}, is a Wallet address",nft_address)


    }
    

}




pub async fn get_nft_balance_testnet(nft_address:&str, wallet_address: &str)-> String  {

    let provider = init_avax_testnet().await; 

    let token_addr = Address::from_str(nft_address).expect("REASON");
    let wallet_addr = Address::from_str(wallet_address).expect("REASON");

    let wallet = provider.get_code_at(token_addr).await;

    if !wallet.expect("REASONS").is_empty(){

        let verify_wallet = provider.get_code_at(wallet_addr).await;

        if verify_wallet.expect("REASONS").is_empty(){

             if is_erc721_nft_contract(&provider, token_addr).await{

                let erc721 = IERC721::new(token_addr,provider.clone());

                let name = erc721.name().call().await;
                let symbol = erc721.symbol().call().await;
                let balance = erc721.balanceOf(wallet_addr).call().await;

                format!("The wallet {:#?} has an ERC721 NFT balance of {:#?}
                in {:#?}{:#?} for Avax Testnet",wallet_addr,balance,name,symbol)

             }else if is_erc1155_nft_contract(&provider, token_addr).await{

                let erc721 = IERC721::new(token_addr,provider.clone());

                let name = erc721.name().call().await;
                let symbol = erc721.symbol().call().await;
                let balance = erc721.balanceOf(wallet_addr).call().await;

                format!("The wallet {:#?} has an ERC1155 NFT balance of {:#?}
                in {:#?}{:#?} for Avax Testnet",wallet_addr,balance,name,symbol)


             }else{
                format!("The address {} isn't a standard NFT",nft_address)
             }
            

        }else{
            format!("The address {}, is not a wallet address",nft_address)

        }     
    }else{
        format!("The address {}, is a Wallet address",nft_address)

    }

}



pub async fn get_nft_balance_mainnet(nft_address:&str, wallet_address: &str) -> String {

    let provider = init_avax_mainnet().await; 

    let token_addr = Address::from_str(nft_address).expect("REASON");
    let wallet_addr = Address::from_str(wallet_address).expect("REASON");

    let wallet = provider.get_code_at(token_addr).await;

    if !wallet.expect("REASONS").is_empty(){
        
        let verify_wallet = provider.get_code_at(wallet_addr).await;

        if verify_wallet.expect("REASONS").is_empty(){
            if is_erc721_nft_contract(&provider, token_addr).await{

                let erc721 = IERC721::new(token_addr,provider.clone());

                let name = erc721.name().call().await;
                let symbol = erc721.symbol().call().await;
                let balance = erc721.balanceOf(wallet_addr).call().await;

                format!("The wallet {:#?} has an ERC721 NFT balance of {:#?}
                in {:#?}{:#?} for Avax Mainnet",wallet_addr,balance,name,symbol)

             }else if is_erc1155_nft_contract(&provider, token_addr).await{

                let erc721 = IERC721::new(token_addr,provider.clone());

                let name = erc721.name().call().await;
                let symbol = erc721.symbol().call().await;
                let balance = erc721.balanceOf(wallet_addr).call().await;

                format!("The wallet {:#?} has an ERC1155 NFT balance of {:#?}
                in {:#?}{:#?} for Avax Mainnet",wallet_addr,balance,name,symbol)


             }else{
                format!("The address {} isn't a standard NFT on Avax",nft_address)
             }
            
        }else{

            format!("The address {}, is not a wallet address",nft_address)

        }
    }else{

         format!("The address {}, is a Wallet address",nft_address)
    }

}