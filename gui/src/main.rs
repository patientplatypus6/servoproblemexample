use ipc_channel::ipc::{self, IpcOneShotServer, IpcSender, IpcReceiver};
use std::collections::HashMap;
use std::process::Command as ProcessCommand;
use std::env;
use std::io;
use std::{thread, time};



fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    println!("value of args {:?}", args);

    let mut s = args[1].clone();
    println!("&cloned_string: {:?}", args[1].clone());
    let tx2 = IpcSender::<Vec<(String, String)>>::connect(args[1].clone()).unwrap();
}
