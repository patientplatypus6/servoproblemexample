use std::process::Command;
use ipc_channel::ipc::{self, IpcOneShotServer, IpcSender, IpcReceiver};
use std::env;
use serde::*;
use std::{thread, time};

// type Data = (String, String);
type Data = Vec<(String, String)>;
type Bootstrap = (IpcSender<Data>, IpcReceiver<Data>);

fn main() {
    println!("Inside process_handler");
    let (server0, server_name0) = IpcOneShotServer::<Bootstrap>::new().unwrap();
    println!("server_name0 in process_handler: {:?}", server_name0);
    let guiserver = spawn_server(
        "/Users/peterweyand/Code/rustprojects/project1_2/src/rungui.sh".to_string(),
        &server_name0
    );
    let (_receiver, (sender, receiver)): (IpcReceiver<Bootstrap>, Bootstrap) = server0.accept().unwrap();
    let data = vec![("Peter".to_string(), "36".to_string())];
    sender.send(data);

    loop {
        match receiver.try_recv() {
            Ok(res) => {
                println!("Received data in main...{:?}", res);
                break;
            },
            Err(_) => {
                println!("Still waiting in main...");
                std::thread::sleep_ms(1000);
            }
        }
    }
}

pub fn spawn_server(address:String, server_name0: &str) -> std::process::Child {
    Command::new(address) 
        .arg(server_name0)
        .spawn()
        .expect("failed to execute server process")
}
