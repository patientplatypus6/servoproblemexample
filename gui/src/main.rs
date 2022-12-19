use ipc_channel::ipc::{self, IpcOneShotServer, IpcSender, IpcReceiver};
use std::collections::HashMap;
use std::process::Command as ProcessCommand;
use std::env;
use std::io;
use std::{thread, time};

type Data = Vec<(String, String)>;

fn main() {
    println!("Inside GUI process");
    let args: Vec<String> = env::args().collect();
    let (s1, r1): (IpcSender<Data>, IpcReceiver<Data>) = ipc::channel().unwrap();
    let s0 = IpcSender::connect(args[1].clone()).unwrap();
    s0.send(s1).unwrap();

    loop {
        match r1.try_recv() {
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
