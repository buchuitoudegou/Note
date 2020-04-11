struct Context {
  method: String,
}

type Handler = Box<dyn Fn(&Context)>;

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
    for i in 0..self.handlers.len() {
      let new_ctx = Context {
        method: String::from(format!("{}", i)),
      };
      self.handlers[i](&new_ctx);
    }
  }
}

fn main() {
  let mut a = Dealer::new();
  a.register(Box::new(|s: &Context| {
    println!("ab: {}", s.method);
  }));
  a.register(Box::new(|s: &Context| {
    println!("cd: {}", s.method);
  }));
  a.deal();
}