use std::env;
use std::fs;
use std::io;
use std::process::{Command, Output, Stdio};

use actix::prelude::*;
use actix_web_actors::ws;
use async_trait::async_trait;
use futures::executor::block_on;

use std::borrow::{Borrow, BorrowMut};

pub enum RCONCmdType {
    CONNECT = 1,
    DISCONNECT = 2,
    MSG = 3,
}

/// Define message
#[derive(Message)]
#[rtype(result = "Result<String, std::io::Error>")]
pub struct RCONCmd {
    pub cmd_type: RCONCmdType,
    pub body: String,
}

pub struct RCONActor {
    pub rcon: Option<rcon::Connection>,
}

impl Actor for RCONActor {
    type Context = Context<Self>;
    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("RCON Actor is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("RCON Actor is stopped");
    }
}

fn connect() -> Result<rcon::Connection, rcon::Error> {
    block_on(create_rcon().connect(
        "127.0.0.1",
        &env::var("RCON_PWD").expect("No RCON password specified!"),
    ))
}

/// RCON command handler
impl Handler<RCONCmd> for RCONActor {
    type Result = Result<String, std::io::Error>;

    fn handle(&mut self, cmd: RCONCmd, ctx: &mut Context<Self>) -> Self::Result {
        match cmd.cmd_type {
            RCONCmdType::CONNECT => {
                println!("[RCONActor] received connect message.");
                if self.rcon.is_some() {
                    return Ok(String::from("already connected."));
                } else {
                    println!("connecting to rcon....");
                    self.rcon = Some(connect().unwrap());
                    return Ok(String::from("connecting to rcon..."));
                }
                // match attempt {
                //     Ok(conn) => ctx.text("RCON Connected."),
                //     Err(err) => ctx.text("Failed to connect to RCON."),
                // }
                //Ok(String::new())
            }
            RCONCmdType::DISCONNECT => {}
            RCONCmdType::MSG => {
                println!("[RCONActor] received msg message.");
                match &mut self.rcon {
                    Some(r) => return Ok(block_on(r.cmd(&cmd.body)).unwrap()),
                    None => return Ok(String::from("not connected to rcon")),
                }
            }
        }

        Ok(String::new())
    }
}

fn remove_chars(s: &str, n: usize) -> String {
    let mut iter = s.chars();
    iter.by_ref().nth(n);
    String::from(iter.as_str())
}

pub fn create_rcon() -> rcon::Builder {
    rcon::Builder::new().enable_minecraft_quirks(true)
}

pub fn start_server() -> io::Result<()> {
    let mut path = fs::canonicalize(env::var("SERVER_JAR").expect("No server JAR specified!"))?;

    let path_str = remove_chars(path.to_str().expect("could not trim path string!"), 3);
    path.pop();
    let pwd_str = remove_chars(path.to_str().expect("could not trim path string!"), 3);

    println!(
        "Server jar path: {}",
        path.file_name()
            .expect("failed to extract jar file name!")
            .to_str()
            .expect("failed to convert path to string!")
    );
    println!("Server working directory: {}", pwd_str);

    Command::new("java")
        .current_dir(pwd_str)
        .arg("-jar")
        .arg(path_str)
        .stdout(Stdio::null())
        .spawn()
        .expect("failed to execute child");
    //.arg("-nogui")

    Ok(())
}

pub fn stop_server() {}
