use std::collections::HashMap;
use serde_json::{Value};
use std::fmt::Write;




pub async fn get_price(_price:&str) -> String {
    let response = reqwest::Client::new()
        .get("https://min-api.cryptocompare.com/data/price")
        .query(&[("fsym", _price), ("tsyms", "USD")])
        .header("accept", "application/json").send()
        .await.ok().expect("REASON");// Uses ? operator for error propagation

    let mut report = String::new();
    let _prices: String = match response.json::<HashMap<String,f64>>().await{
            

            Ok(num) => {

                
                
                write!(&mut report, "The price of {} is ${}", _price,num["USD"]).unwrap();
                // format!("The price of {} is ${}",_price,num["USD"])
                return report
            
            },
            Err(_) => {
                // format!("The price of {_price} is not supported yet")

                write!(&mut report, "The price of {} is not supported yet", _price).unwrap();

                return report
            }
        };

}





pub async fn get_marketcap(coin:&str) -> String {
    let token = format!("https://min-api.cryptocompare.com/data/pricemultifull?fsyms={}&tsyms=USD",coin);  
    
    
    let response = reqwest::get(&token).await.expect("REASON"); 
    let market_cap : Value =  response.json().await.ok().expect("REASON");
    let mut report = String::new();
    if market_cap.get("DISPLAY").is_some() {
        let json_response = format!("{}",&market_cap.get("DISPLAY")
        .expect("REASON").get(&coin)
        .expect("REASON").get("USD").expect("REASON")
        .get("MKTCAP").unwrap()
        .as_str().unwrap());

        write!(&mut report, "The market capitalization of {} is {}",coin,json_response).unwrap();

        return report;
    }else{
        write!(&mut report,"Error fetching data!").unwrap();
        return report;
    }

}