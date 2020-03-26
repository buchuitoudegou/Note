use std::fmt::Display;
struct SelfVec<T> {
    container: Vec<T>,
    idx: usize,
}

impl<T> Iterator for SelfVec<T>
    where T: Copy {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.container.len() {
            self.idx += 1;
            Some(self.container[self.idx - 1])
        } else {
            None
        }
    }
}

impl<T> SelfVec<T>
    where T: Copy {
    fn new() -> SelfVec<T> {
        SelfVec {
            container: Vec::new(),
            idx: 0,
        }
    }
    fn get(&mut self, idx: usize) -> Option<T> {
        if idx < self.container.len() {
            Some(self.container[idx])
        } else {
            None
        }
    }

    fn set(&mut self, idx: usize, new_val: T) -> Option<T> {
        if idx < self.container.len() {
            self.container[idx] = new_val;
            Some(new_val)
        } else {
            None
        }
    }
    fn push_back(&mut self, new_val: T) {
        self.container.push(new_val);
    }
    fn pop_back(&mut self) {
        if self.container.len() > 0 {
            self.container.pop();
        }
    }
    fn len(&self) -> usize {
        self.container.len()
    }
}

fn display<T>(a: &mut SelfVec<T>)
 where T: Copy + Display {
    println!("current len: {:?}", a.len());
    for i in 0..a.len() {
        print!("{} ", a.get(i).unwrap());
    }
    println!("");
}

fn main() {
    let mut a: SelfVec<u32> = SelfVec::new();
    a.push_back(1);
    a.push_back(2);
    a.push_back(3);
    display(&mut a);
    a.pop_back();
    display(&mut a);
    a.set(0, 1000);
    display(&mut a);
}