use reqwest;
use serde::{Deserialize, Serialize};

use crate::api::GetRes;    


#[derive(Serialize)]
pub struct FormData {
    todo_input: String,
    todo_description: String,
    todo_ischecked: bool
}

#[derive(Deserialize, Serialize)]
pub struct BackendRes {
    pub msg: String
}



pub async fn send_to_back(
    todo_input: &str, 
    todo_description: &str, 
    todo_ischecked: bool
) -> Result<String, reqwest::Error> {
    let data = FormData {
        todo_input: todo_input.to_string(),
        todo_description: todo_description.to_string(),
        todo_ischecked
    };
    let client = reqwest::Client::new();
   let response = client
    .post("http://127.0.0.1:3000/write-todo")
    .json(&data)
    .send()
    .await?
    .json::<BackendRes>()
    .await?;
    Ok(response.msg)
}


pub async fn get_todos() -> Result<Vec<GetRes>, reqwest::Error> {
    let client = reqwest::Client::new();
    let get = client.get("http://127.0.0.1:3000/get-todos")
    .send()
    .await?
    .json::<Vec<GetRes>>()
    .await?;
    Ok(get)
}

pub async fn delete_todo(
    todo_input: &str,
    todo_description: &str,
    todo_ischecked: bool
) -> Result<String, reqwest::Error> {
    let data = FormData {
        todo_input: todo_input.to_string(),
        todo_description: todo_description.to_string(),
        todo_ischecked
    };

    let client = reqwest::Client::new();
    let delete_a_todo = client
    .delete("http://127.0.0.1:3000/delete-todo")
    .json(&data)
    .send()
    .await?
    .text()
    .await?;
    Ok(delete_a_todo)
}