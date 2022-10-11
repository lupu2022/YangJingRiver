use crate::vector::Vector;

type TNT = f32;

#[derive(Debug, PartialEq, Clone)]
pub enum YjrItem {
    S( String ),            // symbol, used as flag or hash key
    N( TNT ),               // number
    V( Vector<TNT> ),       // vector
    M( Vector<TNT> ),       // matrix
}

#[derive(Debug)]
pub struct YjrStack {
    data:   Vec<YjrItem>
}

impl YjrItem {
    pub fn is_symbol(&self) -> bool {
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
    pub fn is_matrix(&self) -> bool {
        match self {
            YjrItem::M(_) => true,
            _ => false,
        }
    }

    // consuming
    pub fn as_symbol(self) -> String {
        match self {
            YjrItem::S(s) => s,
            _ => panic!("Item is not symbol")
        }
    }

    pub fn as_number(self) -> TNT {
        match self {
            YjrItem::N(n) => n,
            _ => panic!("Item is not number")
        }
    }

    pub fn as_vector(self) -> Vector<TNT> {
        match self {
            YjrItem::V(v) => v,
            _ => panic!("Item is not number")
        }
    }

    pub fn as_matrix(self) -> Vector<TNT> {
        match self {
            YjrItem::M(v) => v,
            _ => panic!("Item is not number")
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

    pub fn push_symbol<T: ToString>(&mut self, s: T) {
        let item = YjrItem::S(s.to_string());
        self.data.push( item );
    }

    pub fn push_number(&mut self, n: TNT) {
        let item = YjrItem::N(n);
        self.data.push(item);
    }

    pub fn push_vector(&mut self, v: Vector<TNT>) {
        let item = YjrItem::V(v);
        self.data.push(item);
    }

    pub fn push_symbol_list(&mut self, sl: Vec<String>) {
        let lsize = sl.len();
        for s in sl {
            self.push_symbol(s);
        }
        self.push_number(lsize as TNT);
    }

    pub fn push_number_list(&mut self, sl: Vec<TNT>) {
        let lsize = sl.len();
        for s in sl {
            self.push_number(s);
        }
        self.push_number(lsize as TNT);
    }

    pub fn push_vector_list(&mut self, sl: Vec<Vector<TNT>>) {
        let lsize = sl.len();
        for s in sl {
            self.push_vector(s);
        }
        self.push_number(lsize as TNT);
    }

    pub fn pop_symbol(&mut self) -> String {
        self.data.pop().unwrap().as_symbol()
    }

    pub fn pop_number(&mut self) -> TNT {
        self.data.pop().unwrap().as_number()
    }

    pub fn pop_vector(&mut self) -> Vector<TNT> {
        self.data.pop().unwrap().as_vector()
    }

    pub fn pop_symbol_list(&mut self) -> Vec<String> {
        let lsize = self.pop_number() as usize;
        let mut ret = Vec::new();
        for _ in 0..lsize {
            ret.push( self.pop_symbol() );
        }
        return ret;
    }

    pub fn pop_number_list(&mut self) -> Vec<TNT> {
        let lsize = self.pop_number() as usize;
        let mut ret = Vec::new();
        for _ in 0..lsize {
            ret.push( self.pop_number() );
        }
        return ret;
    }

    pub fn pop_vector_list(&mut self) -> Vec<Vector<TNT>> {
        let lsize = self.pop_number() as usize;
        let mut ret = Vec::new();
        for _ in 0..lsize {
            ret.push( self.pop_vector() );
        }
        return ret;
    }
}

#[cfg(test)]
mod tests {
    use crate::vector::Vector;
    use crate::stack::YjrStack;

    #[test]
    fn simple_test() {
        let mut stack = YjrStack::new();

        stack.push_number(3.14);
        stack.push_symbol("Hello World");
        stack.push_symbol("Hello World".to_string());

        println!("{:?}", stack);
    }
}
