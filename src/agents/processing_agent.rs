// use genai::chat::printer::print_chat_stream;
use genai::chat::{ChatMessage, ChatRequest};
use genai::Client;
use std::sync::{Arc, OnceLock};

// static PROCESS_SYSTEM_CONFIGURATION: OnceCell<String> = OnceCell::const_new();
const CLIENT: OnceLock<Arc<Client>> = OnceLock::new();

static PROCESS_SYSTEM_CONFIGURATION: &str = include_str!("../../knowledgebase/avalanche_gpt_config.txt");


#[inline(always)]
pub fn get_client() -> Arc<Client> {
    CLIENT.get_or_init(||{
        Arc::new(Client::default())
    }).clone()
}


pub async fn process(_text:&str) -> Option<std::string::String> {

    let client = get_client();
    let chat_req: ChatRequest = ChatRequest::new(vec![
        ChatMessage::system(PROCESS_SYSTEM_CONFIGURATION),
        ChatMessage::user(_text)
    ]);

    let model: &str = "gemini-2.5-flash";

    let chat_res = client.exec_chat(model, chat_req, None).await;

    chat_res.expect("REASON").content_text_into_string()
    
}