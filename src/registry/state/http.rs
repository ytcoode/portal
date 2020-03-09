use super::Global;
use rocket::State;
use std::sync::Arc;
use std::sync::Mutex;

pub fn serve(global: Arc<Mutex<Global>>) {
    rocket::ignite()
        .manage(global)
        .mount("/", routes![query])
        .launch();
}

#[get("/<id>")]
fn query(id: String, global: State<Arc<Mutex<Global>>>) -> &'static str {
    println!("id: {}", id);
    global
        .lock()
        .unwrap()
        .tunnels
        .insert("id".to_string(), vec![]);

    "Hello, world!"
}
