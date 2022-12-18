use std::process::Command;
use ipc_channel::ipc::{self, IpcOneShotServer, IpcSender, IpcReceiver};
use std::env;
use serde::*;
use std::{thread, time};

fn main() {
    println!("Inside process_handler");
    let (server0, server_name0) = IpcOneShotServer::<Vec<(String, String)>>::new().unwrap();
    println!("server_name0 in process_handler: {:?}", server_name0);
    let guiserver = spawn_server(
        "/Users/peterweyand/Code/rustprojects/project1_2/src/rungui.sh".to_string(), 
        &server_name0
    );
}

pub fn spawn_server(address:String, test_name: &str) -> std::process::Child {
    Command::new(address) 
        .arg(test_name)
        .spawn()
        .expect("failed to execute server process")
}
