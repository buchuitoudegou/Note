use std::rc::Rc;

struct Node {
    val: i32,
    next: Option<Box<Node> >
}


fn main() {
    // Box
    // let head = Node {
    //     val: 0,
    //     next: None,
    // };
    // let mut cur = &mut Box::new(head);
    // for i in 1..4 {
    //     (*cur).next = Some(Box::new(Node {
    //         val: i,
    //         next: None,
    //     }));
    //     cur = (*cur).next.as_ref().unwrap(); // err
    // }

    // let tail = Node {
    //     val: 4,
    //     next: None,
    // };
    // let mut cur = Box::new(tail);
    // for i in 1..4 {
    //     cur = Box::new(Node {
    //         val: 4 - i,
    //         next: Some(cur),
    //     });
    // }
    // for i in 0..4 {
    //     println!("{}", cur.val);
    //     cur = match &cur.next {
    //         Some(i) => Box::new(**i), // err: Node不可以移动，也没办法复制Box指针
    //         None => break,
    //     };
    // }

    // Rc
    // let head = Node {
    //     val: 0,
    //     next: None,
    // };
    // let mut cur = Rc::new(head);
    // for i in 1..4 {
    //     (*cur).next = Some(Rc::new(Node {
    //         val: i,
    //         next: None,
    //     })); // err: Rc指针指向的内存块不可以改变
    // }
    
    // let tail = Node {
    //     val: 4,
    //     next: None,
    // };
    // let mut cur = Rc::new(tail);
    // for i in 1..4 {
    //     cur = Rc::new(Node {
    //         val: 4 - i,
    //         next: Some(Rc::clone(&cur)),
    //     });
    // }
    // for i in 0..4 {
    //     println!("{}", cur.val);
    //     cur = match &cur.next {
    //         Some(i) => Rc::clone(i),
    //         None => break,
    //     };
    // }
    let head = Rc::new(RefCell::new(Node {
        val: 0,
        next: None,
      }));
      let mut cur = Rc::clone(&head);
      for i in 1..4 {
        let new_node = Rc::new(RefCell::new(Node {
          val: i,
          next: None,
        }));
        (*cur).borrow_mut().next = Some(Rc::clone(&new_node));
        cur = Rc::clone(&new_node);
      }
      let mut cur = Rc::clone(&head);
      loop {
        println!("{}", (*cur).borrow().val);
        let tmp = Rc::clone(&cur);
        match  &(*tmp).borrow().next {
          Some(node) => {
            cur = Rc::clone(&node);
          },
          None => {
            break;
          }
        };
      }
}

// 1. 直接用引用的问题：在循环创建Node的时候，Node被放在栈中
// 2. 用Box的问题：
//     1）Box能将Node放在堆中，而且可以修改
//     2）但对于同一个内存块，Box只能有一个指针指向
//     3) 可以从尾到头构造链表；但是从头到尾遍历读取的时候，没有办法复制（移动）Box或者里面的Node
//     4）从头到尾无法复制（移动）Box指针。
//     4) 从头到尾构造链表的时候，如果使用&mut类型作为中间变量，但是在展开（unwrap）新节点的时候，无法获得其可变引用
// 3. 用Rc的问题：
//     1）Rc能将Node放在堆中，但是不可以修改这块内存的数据
//     2）对于同一个内存块，Rc可以通过clone增加指针数量
//     3）无法从头到尾构造；Rc指针不能修改内存块
//     4）可以从尾到头构造，且可以读取