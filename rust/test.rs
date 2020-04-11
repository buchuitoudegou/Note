struct Context<'a> {
  method: &'a str,
}

type Handler = Box<dyn Fn(&mut Context)>;

struct Dealer {
  handlers: Vec<Handler>,
}

impl Dealer {
  fn new() -> Dealer {
    Dealer {
      handlers: vec![],
    }
  }
  fn register(&mut self, handler: Handler) {
    self.handlers.push(handler);
  }
  fn deal(&self) {
    let string = "abcd";
    for i in 0..self.handlers.len() {
      let mut new_ctx = Context {
        method: string,
      };
      self.handlers[i](&mut new_ctx);
    }
  }
}

fn main() {
  let mut a = Dealer::new();
  a.register(Box::new(|s: &mut Context| {
    println!("ab: {}", s.method);
  }));
  a.register(Box::new(|s: &mut Context| {
    println!("cd: {}", s.method);
  }));
  a.deal();
}