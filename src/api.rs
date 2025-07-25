use std::{fs::{self, read_to_string, write}, time::Duration};

use actix_web::{delete, get, post, web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

use crate::send_req::BackendRes;


#[derive(Deserialize, Serialize, Clone)]
pub struct FormData {
    todo_input : String,
    todo_description: String,
    todo_ischecked: bool
}

#[derive(Serialize, Deserialize)]
pub struct WriteRes {
    msg: String
}

#[derive(Serialize, Deserialize)]
pub struct GetRes {
    pub todo_input: String,
    pub todo_description: String,
    pub todo_ischecked: bool
}



pub async fn run_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(write_todo)
        .service(get_todos)
        .service(delete_todo)
        
    })
    .bind(("127.0.0.1", 3000))?
    .keep_alive(Duration::from_secs(10))
    .run()
    .await
}



#[post("/write-todo")]
async fn write_todo(data: web::Json<FormData>) -> HttpResponse {
    let mut todos: Vec<FormData> = match read_to_string("todos.json") {
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new()
    };
    todos.push(data.into_inner());
    let stringifying = serde_json::to_string_pretty(&todos).expect("failed to stringify !");
    write("todos.json", stringifying).expect("an error occured while writing file");
    HttpResponse::Ok().json(WriteRes {msg: String::from("the write todo on file was successful !")})
}

#[get("/get-todos")]
async fn get_todos() -> HttpResponse {

    if let Ok(content) = fs::read_to_string("todos.json") {
        let datas: Vec<GetRes> = serde_json::from_str(&content).unwrap_or_else(|_| Vec::new());
        HttpResponse::Ok().json(datas)
     } else {
        HttpResponse::Ok().json(WriteRes {
            msg: String::from("there is no Todo !")
        })
    }
}

#[delete("/delete-todo")]
async fn delete_todo(data: web::Json<FormData>) -> HttpResponse {
    let todos: Vec<FormData> = match read_to_string("todos.json") {
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(),
    };

let todos_len = todos.len();
    let filtered: Vec<_> = todos
        .into_iter()
        .filter(|todo| {
            !(todo.todo_input == data.todo_input
                && todo.todo_description == data.todo_description
                && todo.todo_ischecked == data.todo_ischecked)
        })
        .collect();

    let stringifying = serde_json::to_string_pretty(&filtered).unwrap();
    fs::write("todos.json", stringifying).unwrap();

    let response = if filtered.len() < todos_len {
        BackendRes {
            msg: "data was deleted successfully !".to_string(),
        }
    } else {
        BackendRes {
            msg: "could not be able to delete successfully !".to_string(),
        }
    };

    HttpResponse::Ok().json(response)
}