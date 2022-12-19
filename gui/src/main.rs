use ipc_channel::ipc::{self, IpcOneShotServer, IpcSender, IpcReceiver};
use std::collections::HashMap;
use std::process::Command as ProcessCommand;
use std::env;
use std::io;
use std::{thread, time};



fn main() {
    println!("Inside GUI process");
    let datavec = vec![("Peter".to_string(), "36".to_string())];
    let args: Vec<String> = env::args().collect();
    let (s1, r1): (IpcSender<Vec<(String, String)>>, IpcReceiver<Vec<(String, String)>>) = ipc::channel().unwrap();
    s1.send(datavec).unwrap();
    let s0 = IpcSender::connect(args[1].clone()).unwrap();
    s0.send(s1).unwrap();
    loop {
        std::thread::sleep_ms(1000);
    }
}
