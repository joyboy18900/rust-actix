# todo-rust-actix

This is a little Rust project I made to learn about the language and the Actix Web framework.

* Add new tasks
* See all your tasks
* Update tasks
* Delete tasks

**It's just for practice, so the code might not be perfect!**  I'm still learning Rust.

## Getting Started

1. **Prerequisites:**
    * Make sure you have Rust and Cargo installed. You can download and install them from the official Rust website: [https://www.rust-lang.org/](https://www.rust-lang.org/)

2. **Clone the Repository:**
   ~~~bash
   git clone https://github.com/joyboy18900/rust-actix.git cd rust-actix

3. **Build and Run:**
   ~~~bash
   cargo run

This will compile the code and start the server on `http://localhost:8080`.

## The API has these endpoints:

| Method | Endpoint       | What it Does                |
|--------|----------------|-----------------------------|
| GET    | `/items`      | Shows you all your tasks   |
| POST   | `/items`      | Lets you add a new task     |
| PUT    | `/items/{id}` | Updates a task by its ID  |
| DELETE | `/items/{id}` | Deletes a task by its ID  |

## Trying it Out

You can test it with `curl` :

* **Add a task:**
   ~~~bash
   curl -X POST -H "Content-Type: application/json" -d '{"id": 1, "name": "Buy groceries"}' http://localhost:8080/items

* **See your tasks:**
   ~~~bash
   curl http://localhost:8080/items

 * **Update a task:**
   ~~~bash
   curl -X PUT -H "Content-Type: application/json" -d '{"id": 1, "name": "Buy milk and eggs"}' http://localhost:8080/items/1

* **Delete a task:**
  ~~~bash
  curl -X DELETE http://localhost:8080/items/1
