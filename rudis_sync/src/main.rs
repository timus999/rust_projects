use lazy_static::lazy_static;
use resp::Decoder;
use resp::Value;
use std::collections::HashMap;
use std::env;
use std::io::{BufReader, Write};
use std::net::Shutdown;
use std::net::{TcpListener, TcpStream};
use std::sync::Mutex;
use std::thread;
mod command;

use crate::command::process_client_request;

type STORE = Mutex<HashMap<String, String>>;

lazy_static! {
    static ref RUDIS_DB: STORE = Mutex::new(HashMap::new());
}

fn main() {
    let addr = env::args()
        .skip(1)
        .next()
        .unwrap_or("127.0.0.1:6378".to_owned());

    let listener = TcpListener::bind(&addr).unwrap();
    println!("rudis_sync listening on {} ...", addr);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("New connection from: {:?}", stream);
        thread::spawn(move || {
            handle_client(stream);
        });
    }
}

fn handle_client(stream: TcpStream) {
    let mut writer = stream.try_clone().unwrap();
    let mut reader = BufReader::new(&stream);
    loop {
        let mut decoder = Decoder::new(&mut reader);
        match decoder.decode() {
            Ok(v) => {
                if v == Value::Null {
                    println!("Client disconnected.");
                    break;
                }
                let reply = process_client_request(v);
                if writer.write_all(&reply).is_err() {
                    break;
                }
            }
            Err(e) => {
                println!("Error decoding from stream: {:?}. Closing connection.", e);
                break;
            }
        }
    }
}
