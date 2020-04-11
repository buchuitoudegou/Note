use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::thread;
use std::time::Duration;

mod middle_ware;
mod thread_pool;

use thread_pool::thread_pool::ThreadPool;
use middle_ware::middle_ware::ReqDealer;
use middle_ware::middle_ware::Context;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let thread_pool = ThreadPool::new(2);

    
    for stream in listener.incoming() {
        thread_pool.execute(move || {
            handle_connection(stream.unwrap());
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let req_str = String::from_utf8((&buffer[..]).to_vec()).unwrap();
    let mut req_dealer = ReqDealer::new(&stream, &req_str);
    req_dealer.register(Box::new(|ctx: &mut Context| {
        if ctx.path == "/sleep" {
            ctx.resp = String::from("sleeping\n");
        }
    }));
    req_dealer.register(Box::new(|ctx: &mut Context| {
        if ctx.path == "/" {
            ctx.resp = String::from("normal req\n");
        }
    }));
    req_dealer.handle();
}