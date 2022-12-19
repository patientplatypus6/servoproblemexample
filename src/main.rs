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
    let (receiver, sendervec): (IpcReceiver<Vec<(String,String)>>, Vec<_>) = server0.accept().unwrap();
    loop {
        match receiver.try_recv() {
            Ok(res) => {
                // Do something interesting with your result
                std::thread::sleep_ms(1000);
                println!("Received data...{:?}", res);
                //I don't know what type signature res is supposed to have, but it should receive the datavec. Then I can update that in main and in gui I can access via r1
                break;
            },
            Err(_) => {
                // Do something else useful while we wait
                std::thread::sleep_ms(1000);
                println!("Still waiting...");
            }
        }
    }
}

pub fn spawn_server(address:String, test_name: &str) -> std::process::Child {
    Command::new(address) 
        .arg(test_name)
        .spawn()
        .expect("failed to execute server process")
}
