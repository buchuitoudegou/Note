// 如下为选择 Box<T>，Rc<T> 或 RefCell<T> 的理由：
// 1. Rc<T> 允许相同数据有多个所有者；Box<T> 和 RefCell<T> 有单一所有者。
// 2. Box<T> 允许在编译时执行不可变或可变借用检查；Rc<T>仅允许在编译时执行不可变借用检查；RefCell<T> 允许在运行时执行不可变或可变借用检查。
// 3. 因为 RefCell<T> 允许在运行时执行可变借用检查，所以我们可以在即便 RefCell<T> 自身是不可变的情况下修改其内部的值。
use std::rc::Rc;
use std::cell::RefCell;

struct Node {
  val: i32,
  next: Option<Rc<RefCell<Node> > >
}

struct List {
  head: Option<Rc<RefCell<Node> > >,
}

impl List {
  fn new() -> List {
    List {
      head: None,
    }
  }
  fn get(&self, idx: usize) -> Option<i32> {
    match &self.head {
      Some(rc_ref) => {
        let mut cur = Rc::clone(&rc_ref);
        let mut t1;
        for _ in 0..idx {
          t1 = Rc::clone(&cur);
          match &(*t1).borrow().next {
            Some(node) => {
              cur = Rc::clone(&node);
            },
            None => { return None; },
          };
        }
        return Some((*cur).borrow().val);
      },
      None => None,
    }
  }
  fn set(&mut self, idx: usize, new_val: i32) -> Option<i32> {
    match &self.head {
      Some(rc_ref) => {
        let mut cur = Rc::clone(&rc_ref);
        let mut t1;
        for _ in 0..idx {
          t1 = Rc::clone(&cur);
          match &(*t1).borrow().next {
            Some(node) => {
              cur = Rc::clone(&node);
            },
            None => { return None; },
          };
        }
        (*cur).borrow_mut().val = new_val;
        return Some(new_val);
      },
      None => None,
    }
  }
  fn push(&mut self, new_val: i32) {
    match &self.head {
      Some(rc_ref) => {
        let mut tmp = Rc::clone(&rc_ref);
        let mut t1;
        loop {
          t1 = Rc::clone(&tmp);
          match &(*t1).borrow().next {
            Some(node) => {
              tmp = Rc::clone(&node);
            },
            None => {
              break;
            }
          };
        }
        (*tmp).borrow_mut().next = Some(Rc::new(RefCell::new(Node {
          val: new_val,
          next: None,
        })));
      },
      None => {
        self.head = Some(Rc::new(RefCell::new(Node {
          val: new_val,
          next: None,
        })));
      }
    }
  }
  fn display(&self) {
    match &self.head {
      Some(rc_ref) => {
        let mut cur = Rc::clone(&rc_ref);
        let mut t1;
        print!("number: ");
        loop {
          print!("{} ", cur.borrow().val);
          t1 = Rc::clone(&cur);
          match &(*t1).borrow().next {
            Some(node) => {
              cur = Rc::clone(&node);
            },
            None => {
              println!("");
              break;
            }
          }
        }
      },
      None => {
        println!("empty list");
      }
    };
  }
}

fn main() {
  let mut a = List::new();
  a.display();
  a.push(1);
  a.push(2);
  a.push(3);
  a.set(0, 100);
  a.set(10, 9);
  a.display();
  println!("{} {}", a.get(2).unwrap(), a.get(0).unwrap());
}