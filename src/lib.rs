//TODO: &str
//TODO: Normal api with named fields
//TODO: clear dir after test except main.lua
//TODO: isolation of proccess. Now anybody can delete whole server :)

#[macro_use]
extern crate diesel;

use std::{
    fs::File,
    io::{Seek, Write},
    path::Path,
    process::{Command, Stdio},
    str::from_utf8,
};

use diesel::prelude::*;

pub mod models;
pub mod schema;

use models::{Task, Theme};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::str;

static MEMORY_LEAK_ERROR_MESSAGE: &str =
    "Процес 'убит' операционной системой. Возможно, программа, потребляет слишком много памяти";

#[derive(Serialize, Deserialize)]
struct Test(Vec<TestIdentity>);

impl Iterator for Test {
    type Item = TestIdentity;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

#[derive(Serialize, Deserialize)]
struct TestIdentity {
    input: String,
    output: String,
}

#[derive(Serialize)]
pub enum StatusCode {
    OK,
    WA(usize, String, String),
    RE(usize, String),
}

pub fn load_tasks(conn: &PgConnection) -> Result<(Vec<Theme>, Vec<Task>), diesel::result::Error> {
    use self::schema::tasks::dsl::*;
    use self::schema::themes::dsl::*;

    let themes_res = themes.load::<Theme>(conn)?;
    let tasks_res = tasks.load::<Task>(conn)?;

    Ok((themes_res, tasks_res))
}

pub fn run_tests(task_id: i32, path_to_dir: &str) -> Result<StatusCode, Box<dyn Error>> {
    let test_file = File::open(format!("server_data/tests/{}.ron", task_id))?;
    let test: Test = ron::de::from_reader(test_file)?;

    let mut command = Command::new("lua");
    command
        .current_dir(path_to_dir)
        .arg("main.lua")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    for t in test.into_iter().enumerate() {
        let TestIdentity { input, output } = t.1;
        let test_num = t.0;

        let mut child = command.spawn()?;
        let stdin = child.stdin.as_mut().ok_or("Cannot reach child stdin")?;
        stdin.write_all(input.as_bytes())?;

        let res = child.wait_with_output()?;
        match res.status.code() {
            Some(code) => match code {
                0 => {
                    if output != "" {
                        let user_output = str::from_utf8(&res.stdout)?;
                        if user_output.trim() != output {
                            return Ok(StatusCode::WA(test_num, output, user_output.to_owned()));
                        }
                    }
                }
                1 => {
                    let error_msg = str::from_utf8(&res.stderr)?;
                    return Ok(StatusCode::RE(test_num, error_msg.to_owned()));
                }
                _ => {
                    return Ok(StatusCode::RE(
                        test_num,
                        "Эта ошибка не может появиться".to_owned(),
                    ))
                }
            },
            None => {
                return Ok(StatusCode::RE(
                    test_num,
                    MEMORY_LEAK_ERROR_MESSAGE.to_owned(),
                ))
            }
        }
    }
    Ok(StatusCode::OK)
}
