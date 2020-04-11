use super::super::context::context::Context;

pub type Handler = Box<dyn Fn(&mut Context)>;

pub trait MiddleWare {
  fn apply(&self, ctx: &mut Context);
}
