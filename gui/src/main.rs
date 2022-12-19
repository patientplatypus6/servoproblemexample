use ipc_channel::ipc::{self, IpcOneShotServer, IpcSender, IpcReceiver};
use std::collections::HashMap;
use std::process::Command as ProcessCommand;
use std::env;
use std::io;
use std::{thread, time};

type Data = Vec<(String, String)>;
type Bootstrap = (IpcSender<Data>, IpcReceiver<Data>);

fn main() {
    println!("Inside GUI process");
    let args: Vec<String> = env::args().collect();
    println!("value of args {:?}", args);
    let (to_child, from_parent): (IpcSender<Data>, IpcReceiver<Data>) = ipc::channel().unwrap();
    let (to_parent, from_child): (IpcSender<Data>, IpcReceiver<Data>) = ipc::channel().unwrap();
    let bootstrap = IpcSender::connect(args[1].clone()).unwrap();
    bootstrap.send((to_child, from_child)).unwrap();
    let data = from_parent.recv().unwrap();
    println!("child received {:?}", data);
    to_parent.send(vec![("Dagne".to_string(), "8".to_string())]);

    loop {
        match from_parent.try_recv() {
            Ok(res) => {
                println!("Received data in gui...{:?}", res);
                break;
            },
            Err(_) => {
                println!("Still waiting in gui...");
                std::thread::sleep_ms(1000);
            }
        }
    }
}
