use super::context::context::{Context};
use std::net::TcpListener;
use std::str;
use std::io::prelude::*;
use crate::req_dealer::middle_ware::{MiddleWare};
// use std::net::TcpStream;
use std::io::Write;

pub struct ReqDealer {
  listener: Option<TcpListener>,
  comp_list: Vec<Box<dyn MiddleWare> >
}

impl ReqDealer {
  pub fn new() -> ReqDealer {
    ReqDealer {
      listener: None,
      comp_list: vec![],
    }
  }
  pub fn at(&mut self, addr: &str) -> Result<&'static str, std::io::Error>{
    match TcpListener::bind(addr) {
      Ok(listener) => {
        self.listener = Some(listener);
        Ok("successfully binding")
      },
      Err(err) => Err(err)
    }
  }
  pub fn listen(&mut self) {
    if let Some(listener) = &self.listener {
      for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();
        let req_str = str::from_utf8(&buffer).unwrap();
        let mut ctx = Context::new(&req_str);
        self.handle(&mut ctx);
        stream.write(ctx.resp.body.as_bytes());
      }
    } else {
      panic!("unable to listen");
    }
  }
  fn handle(&self, ctx: &mut Context) {
    for i in 0..self.comp_list.len() {
      self.comp_list[i].apply(ctx);
    }
  }
  pub fn register(&mut self, component: Box<dyn MiddleWare>) {
    self.comp_list.push(component);
  }
}