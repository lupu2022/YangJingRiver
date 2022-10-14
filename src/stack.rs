use std::cell::{RefCell, RefMut, Ref};
use std::collections::HashMap;
use std::rc::Rc;

use crate::TNT;
use crate::vector::Vector;

#[derive(Debug, PartialEq, Clone)]
pub struct SharedVector(Rc<RefCell<Vector<TNT>>>);

impl SharedVector {
    pub fn new(v: Vector<TNT>) -> Self {
        SharedVector(Rc::new( RefCell::new(v) ))
    }

    pub fn vec(&self) -> Ref<'_, Vector<TNT>> {
        self.0.borrow()
    }

    pub fn vec_mut(&self) -> RefMut<'_, Vector<TNT>> {
        self.0.borrow_mut()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum YjrItem {
    S( String ),        // string, used as flag or hash key
    N( TNT ),           // number
    V( SharedVector ),  // vector
}

#[derive(Debug)]
pub struct YjrStack {
    data:   Vec<YjrItem>
}

impl YjrItem {
    pub fn is_string(&self) -> bool {
        match self {
            YjrItem::S(_) => true,
            _ => false,
        }
    }
    pub fn is_number(&self) -> bool {
        match self {
            YjrItem::N(_) => true,
            _ => false,
        }
    }
    pub fn is_vector(&self) -> bool {
        match self {
            YjrItem::V(_) => true,
            _ => false,
        }
    }

    // consuming
    pub fn as_string(self) -> String {
        match self {
            YjrItem::S(s) => s,
            _ => panic!("Item is not string")
        }
    }

    pub fn as_number(self) -> TNT {
        match self {
            YjrItem::N(n) => n,
            _ => panic!("Item is not number")
        }
    }

    pub fn as_vector(self) -> SharedVector {
        match self {
            YjrItem::V(v) => v,
            _ => panic!("Item is not vector")
        }
    }
}

impl YjrStack {
    pub fn new() -> Self {
        YjrStack {
            data: Vec::new(),
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn top(&self) -> &YjrItem {
        self.data.last().unwrap()
    }

    pub fn pop(&mut self) -> YjrItem {
        self.data.pop().unwrap()
    }

    pub fn drop(&mut self) {
        self.data.pop();
    }

    pub fn dup(&mut self) {
        let new_item = self.top().clone();
        self.data.push(new_item);
    }

    pub fn dup2(&mut self) {
        let top2 = self.data[ self.data.len() - 2].clone();
        let top1 = self.data[ self.data.len() - 1].clone();

        self.data.push(top2);
        self.data.push(top1);
    }

    pub fn swap(&mut self) {
        let top1 = self.data.pop().unwrap();
        let top2 = self.data.pop().unwrap();

        self.data.push(top1);
        self.data.push(top2);
    }

    pub fn rot(&mut self) {
        let top1 = self.data.pop().unwrap();
        let top2 = self.data.pop().unwrap();
        let top3 = self.data.pop().unwrap();

        self.data.push(top2);
        self.data.push(top1);
        self.data.push(top3);
    }

    pub fn push_string<T: ToString>(&mut self, s: T) {
        let item = YjrItem::S(s.to_string());
        self.data.push( item );
    }

    pub fn push_string_list(&mut self, sl: Vec<String>) {
        let lsize = sl.len();
        for s in sl {
            self.push_string(s);
        }
        self.push_number(lsize as TNT);
    }

    pub fn push_number(&mut self, n: TNT) {
        let item = YjrItem::N(n);
        self.data.push(item);
    }

    pub fn push_number_list(&mut self, sl: Vec<TNT>) {
        let lsize = sl.len();
        for s in sl {
            self.push_number(s);
        }
        self.push_number(lsize as TNT);
    }

    pub fn push_vector(&mut self, v: SharedVector) {
        let item = YjrItem::V(v);
        self.data.push(item);
    }

    pub fn push_vector_list(&mut self, sl: Vec<SharedVector>) {
        let lsize = sl.len();
        for s in sl {
            self.push_vector(s);
        }
        self.push_number(lsize as TNT);
    }

    pub fn pop_string(&mut self) -> String {
        self.data.pop().unwrap().as_string()
    }

    pub fn pop_number(&mut self) -> TNT {
        self.data.pop().unwrap().as_number()
    }

    pub fn pop_vector(&mut self) -> SharedVector {
        self.data.pop().unwrap().as_vector()
    }

    pub fn pop_string_list(&mut self) -> Vec<String> {
        let lsize = self.pop_number() as usize;
        let mut ret = vec![String::new(); lsize];
        for i in 0..lsize {
            ret[lsize - i - 1] = self.pop_string();
        }
        return ret;
    }

    pub fn pop_number_list(&mut self) -> Vec<TNT> {
        let lsize = self.pop_number() as usize;
        let mut ret = vec![0.0; lsize];
        for i in 0..lsize {
            ret[lsize - i - 1] = self.pop_number();
        }
        return ret;
    }

    pub fn pop_vector_list(&mut self) -> Vec<SharedVector> {
        let lsize = self.pop_number() as usize;
        let mut ret = vec![];
        for _ in 0..lsize {
            ret.push( self.pop_vector() );
        }
        ret.reverse();
        ret
    }
}

pub type YjrHash = HashMap<String, YjrItem>;


#[cfg(test)]
mod tests {
    use crate::stack::YjrStack;
    use crate::stack::SharedVector;

    #[test]
    fn simple_test() {
        let mut stack = YjrStack::new();

        stack.push_number(3.14);
        stack.push_string("Hello World");
        stack.push_string("Hello World".to_string());

        stack.push_vector( SharedVector::new(vector![1.0, 2.0, 3.0, 4.0, 5.0]) );

        stack.push_number(1949.0);
        stack.push_number(1979.0);
        stack.push_number(2.0);

        let l = stack.pop_number_list();
        let v = stack.pop_vector();
        let s = stack.pop_string();

        stack.push_number_list( vec![1.0, 2.0, 3.0] );

        println!("{:?}", stack);
        println!("{:?}", l);
        println!("{:?}", v.vec() );
        println!("{}", s);
    }
}
