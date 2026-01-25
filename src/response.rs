use crate::agents::{processing_agent::process, report_agent::report_result};
use crate::tools::tools_map::TOOLS;
use crate::tools::utils::{extract_tool_params, destructor_task};
use futures::stream::{FuturesUnordered, StreamExt};
use axum::{Json,debug_handler};
use tokio::task;

// use tokio::sync::Mutex; 
#[derive(Debug, serde::Serialize)]
pub struct Respond{
    output:String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Prompt{
    message:String,
    name:String,
    media:String
}



#[debug_handler]
pub async fn request(Json(query):Json<Prompt>) -> Json<Respond>{

    let process = process(&query.message).await.unwrap();

    let agent_tasks: Vec<String> = destructor_task(&process).into_iter().collect();

    if agent_tasks.is_empty(){
        return Json::from(
        Respond{output:process})

    }


    let mut future_task = FuturesUnordered::new();


    for task in agent_tasks.into_iter(){

        //let sem = semaphore.clone();

        future_task.push(task::spawn(async move{

            
            //if let Ok(_permit) = sem.try_acquire_owned(){       

            if let Some((tool, parameters)) = extract_tool_params(&task.to_string()){

                // println!("{:#?}",extract_tool_params(&process));
                // Convert &Vec<String> -> Vec<&str>
                let params: Vec<&str> = parameters.iter().map(|s| s.as_str()).collect();

                if let Some(func) = TOOLS.get(&tool.as_str()){
                    return func(&params).await;
                }else{
                    "Tools not found".to_string();
                };

            

            };

            return "Invalid Task".to_string()

        }));
        
        

    };


    let mut results: Vec<String> = Vec::with_capacity(future_task.len());

    while let Some(res) = future_task.next().await{
        results.push(res.unwrap());
    }

    let reply: &str = &results.join("\n");


    match report_result(reply).await{
        Some(result) => {
            return Json::from(
        Respond{output:result})
    },
        None => {
            return Json::from(
        Respond{output:"Error reporting task please try again".to_string(),})
    
        }
    } 
}