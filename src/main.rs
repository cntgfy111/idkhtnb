#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

use std::fs::DirBuilder;

use idkhtnb::{
    load_tasks,
    models::{Task, Theme},
    run_tests, StatusCode,
};
use rocket::response::NamedFile;
use rocket::Data;
use rocket_contrib::databases::diesel;
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use std::error::Error;
use uuid::Uuid;

#[database("postgres_db")]
struct DbConn(diesel::PgConnection);

#[get("/")]
fn index() -> NamedFile {
    NamedFile::open("server_data/public/index.html")
        .expect("Can`t open 'server_data/public/index.html'")
}

#[get("/tasks")]
fn get_tasks(conn: DbConn) -> Result<Json<(Vec<Theme>, Vec<Task>)>, diesel::result::Error> {
    let res = load_tasks(&conn)?;
    Ok(Json(res))
}

#[post("/tasks/<id>", format = "text/x-lua", data = "<input>")]
fn post_solution(id: i32, input: Data) -> Result<Json<StatusCode>, Box<dyn Error>> {
    let load_id = Uuid::new_v4();

    let path = format!("server_data/files/{}", load_id);
    DirBuilder::new().create(&path)?;
    input.stream_to_file(format!("{}/main.lua", &path))?;
    Ok(Json(run_tests(id, &path)?))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, get_tasks, post_solution])
        .mount("/static", StaticFiles::from("server_data/public/static"))
        .attach(DbConn::fairing())
        .launch();
}
