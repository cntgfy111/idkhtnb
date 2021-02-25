use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Theme {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Serialize)]
pub struct Task {
    pub id: i32,
    pub theme: i32,
    pub text: String,
    pub input: String,
    pub output: String,
}
