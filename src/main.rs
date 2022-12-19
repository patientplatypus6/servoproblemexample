use std::process::Command;
use ipc_channel::ipc::{self, IpcOneShotServer, IpcSender, IpcReceiver};
use std::env;
use serde::*;
use std::{thread, time};

// type Data = (String, String);
type Data = Vec<(String, String)>;

fn main() {
    println!("Inside process_handler");
    let (server0, server_name0) = IpcOneShotServer::<IpcSender<Data>>::new().unwrap();
    println!("server_name0 in process_handler: {:?}", server_name0);
    let guiserver = spawn_server(
        "/Users/peterweyand/Code/rustprojects/project1_2/src/rungui.sh".to_string(),
        &server_name0
    );
    let (_receiver, sender): (IpcReceiver<IpcSender<Data>>, IpcSender<Data>) = server0.accept().unwrap();

    let data = vec![("Peter".to_string(), "36".to_string())];
    println!("value of _receiver {:?}", _receiver);
    sender.send(data);

    loop {
        match _receiver.try_recv() {
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

pub fn spawn_server(address:String, test_name: &str) -> std::process::Child {
    Command::new(address) 
        .arg(test_name)
        .spawn()
        .expect("failed to execute server process")
}
