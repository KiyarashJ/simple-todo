# 📋 Todo List App

![Rust](https://img.shields.io/badge/Rust-1.81-orange?logo=rust&style=flat-square)
![Slint](https://img.shields.io/badge/Slint-1.8-blue?logo=slint&style=flat-square)
![Actix-Web](https://img.shields.io/badge/Actix--Web-4.9-green?logo=actix&style=flat-square)
![License](https://img.shields.io/badge/License-MIT-yellow?style=flat-square)

A modern, fast, and responsive Todo List application built with **Rust**, **Slint** for the frontend, and **Actix-web** for the backend. This app allows you to manage your tasks with ease, featuring a clean UI and a robust REST API.

---

## 🚀 Features

- **Create Todos**: Add new tasks with a title, description, and completion status.
- **View Todos**: Display all tasks in a sleek, user-friendly interface.
- **Delete Todos**: Remove tasks with a single click.
- **Asynchronous Backend**: Powered by Actix-web for high-performance task management.
- **Cross-Platform UI**: Built with Slint for a native-like experience.

---

## 🛠️ Tech Stack

- **Frontend**: [Slint](https://slint.dev/) - A declarative UI toolkit for Rust.
- **Backend**: [Actix-web](https://actix.rs/) - A powerful and fast web framework for Rust.
- **Async Runtime**: [Tokio](https://tokio.rs/) - For handling asynchronous operations.
- **HTTP Client**: [Reqwest](https://crates.io/crates/reqwest) - For making HTTP requests to the backend.

---

## 📦 Installation

Follow these steps to set up the project locally:

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/your-username/todo-list-app.git
   cd todo-list-app
   ```

# Install Dependencies🐊:
## Ensure you have Rust installed. Then run:bash
```bash
cargo build
```

# Run the Application🐴:
## Start the backend and frontend:bash
```bash
cargo run
```
### The backend will run on http://localhost:3000, and the Slint UI will launch automatically.

# UsageAdd
### a Todo:Enter a title, description, and check the completion status in the UI🪼.
### Click the "Add" button to send the task to the backend✨.

### View Todos:All tasks are displayed in a list, fetched from the backend on startup🎉.
### Delete a Todo:Click the "Delete" button next to a task to remove it from the list and backend.

# Troubleshooting:
### "Decoding response body" error:Ensure the backend is running on http://localhost:3000.
### Verify the DELETE endpoint returns a valid response (e.g., a 200 OK status with a body or 204 No Content).

UI not updating:Check that the todos model in Slint is correctly updated after DELETE operations.
Ensure the ModelRc is properly set after modifying the todo list.

# Contributing🤝:
Contributions are welcome! Follow these steps:Fork the repository.
Create a new branch: git checkout -b feature/your-feature.
Make your changes and commit: git commit -m "Add your feature".
Push to your branch: git push origin feature/your-feature.
Open a Pull Request.

# License
This project is licensed under the MIT License (LICENSE). AcknowledgmentsSlint for the amazing UI toolkit.
Actix-web for the robust backend framework.
Tokio for async runtime support.



