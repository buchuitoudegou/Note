use crate::req_dealer::context::Context;
use crate::req_dealer::middle_ware::{MiddleWare, Handler};

pub struct Router {
  handler_list: Vec<Handler>,
}

impl MiddleWare for Router {
  fn apply(&self, ctx: &mut Context) {
    for i in 0..self.handler_list.len() {
      self.handler_list[i](ctx);
    }
  }
}

impl Router {
  pub fn new() -> Router {
    Router {
      handler_list: vec![]
    }
  }
  pub fn register(&mut self, path: &'static str, f: Handler) -> &mut Router {
    let handler = move |ctx: &mut Context| {
      if ctx.req.href == path {
        f(ctx);
      }
    };
    self.handler_list.push(Box::new(handler));
    self
  }
}