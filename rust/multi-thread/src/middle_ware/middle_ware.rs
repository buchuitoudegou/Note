use std::net::TcpStream;
use std::sync::mpsc;
use std::io::Write;

pub struct Context<'a> {
  pub method: &'a str,
  pub path: &'a str,
  pub resp: String,
}


type MiddleWare = Box<dyn FnOnce(&mut Context) + 'static + Send>;

pub struct ReqDealer<'a> {
  ctx: Context<'a>,
  sender: mpsc::Sender<MiddleWare>,
  receiver: mpsc::Receiver<MiddleWare>,
  handler_num: usize,
  stream: &'a TcpStream,
}

impl<'a> ReqDealer<'a> {
  pub fn new(stream: &'a TcpStream, req_str: &'a String) -> ReqDealer<'a> {
    let str_split: Vec<&str> = req_str.as_str().split("\n").collect();
    let ctx_str: Vec<&str> = str_split[0].split(" ").collect();
    let ctx = Context {
      method: ctx_str[0],
      path: ctx_str[1],
      resp: String::from(""),
    };
    let (sender, receiver) = mpsc::channel();
    ReqDealer {
      ctx,
      sender,
      receiver,
      handler_num: 0,
      stream,
    }
  }
  pub fn register(&mut self, handler: MiddleWare) {
    self.sender.send(handler).unwrap();
    self.handler_num += 1;
  }
  pub fn handle(&mut self) {
    for _ in 0..self.handler_num {
      let handler = self.receiver.recv().unwrap();
      handler(&mut self.ctx);
    }
    self.stream.write(self.ctx.resp.as_bytes());
  }
}