slint::include_modules!();

mod send_req;
mod api;

use std::{rc::Rc};
use send_req::{send_to_back, get_todos};
use api::run_server;
use slint::{Model, ModelRc, VecModel};
use tokio::runtime::Runtime;
use crate::send_req::{delete_todo};

fn main() {
    std::thread::spawn(|| {
        actix_web::rt::System::new().block_on(run_server()).unwrap();
    });

    let ui = MainWindow::new().unwrap();
    let ui_handle = ui.as_weak();
    let rt = Rc::new(Runtime::new().unwrap()); 

    let ui_handle_send = ui_handle.clone();
    let rt_send = rt.clone();
    ui.on_send_data(move |todo_input, todo_description, todo_ischecked| {
        let ui = ui_handle_send.upgrade().unwrap();
        rt_send.block_on(async {
            match send_to_back(&todo_input, &todo_description, todo_ischecked).await {
                Ok(message) => {
                    ui.set_backEnd_res(message.into());
                }
                Err(e) => {
                    ui.set_backEnd_res(format!("Error: {}", e).into());
                }
            }
        });
    });

    let rt_get = rt.clone();
    let todos = rt_get.block_on(get_todos()).unwrap_or_default();
    let converted_todos: Vec<(slint::SharedString, slint::SharedString, bool)> = todos
        .into_iter()
        .map(|todo| (
            slint::SharedString::from(todo.todo_input),
            slint::SharedString::from(todo.todo_description),
            todo.todo_ischecked,
        ))
        .collect();
    let model = Rc::new(VecModel::from(converted_todos));
    ui.set_todos(model.into()); 

    let ui_handle_delete = ui_handle.clone();
    let rt_delete = rt.clone();
    ui.on_delete_item(move |todo_input, todo_description, todo_ischecked| {
        let ui = ui_handle_delete.upgrade().unwrap();
        let list = ui.get_todos();
        rt_delete.block_on(async {
            match delete_todo(&todo_description, &todo_input , todo_ischecked).await {
                Ok(message) => {
                    if let Some(model) = list.as_any().downcast_ref::<VecModel<(slint::SharedString, slint::SharedString, bool)>>() {
                        if let Some(index) = model.iter().position(|todo: (slint::SharedString, slint::SharedString, bool)| {
                            todo.0 == todo_input && todo.1 == todo_description && todo.2 == todo_ischecked
                        }) {
                            model.remove(index);
                            ui.set_todos(ModelRc::new(VecModel::from(model.iter().collect::<Vec<_>>())));
                        } else {
                            ui.set_backEnd_res("Error: Todo not found in list".into());
                        }
                        ui.set_backEnd_res(message.into());
                    } else {
                        ui.set_backEnd_res("Error: Failed to cast model".into());
                    }
                }
                Err(e) => {
                    ui.set_backEnd_res(format!("Error deleting todo: {}", e).into());
                }
            }
        });
    });

    ui.run().unwrap();
}