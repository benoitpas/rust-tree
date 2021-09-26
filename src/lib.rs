use std::rc::Rc;

#[derive(Debug)] // We could have implemented the Diplay trait instead (https://doc.rust-lang.org/std/fmt/trait.Display.html)
pub enum Node<T> {
    Leaf,
    Branch(Rc<T>, Rc<Node<T>>, Rc<Node<T>>) 
}

//mention int types including isize
pub fn add_id<T> (tree: Rc<Node<T>>, i: isize ) -> (Rc<Node<(Rc<T>, isize)>>,isize) {
    match &*tree { // explain &*: tree of type Rc<Node<T>>, *tree of type Node<T>, then borrow the value with &*tree
        Node::Leaf => (Rc::new(Node::Leaf), i),
        Node::Branch(value, left, right) => {
            let new_left = add_id(Rc::clone(left),i);
            let new_right = add_id(Rc::clone(right),new_left.1+1);
            (Rc::new(Node::Branch(Rc::new((Rc::clone(value),new_left.1)),new_left.0,new_right.0)),new_right.1)
        }
    }
}

pub fn add_ref_count<T:std::fmt::Debug>(tree: Rc<Node<T>>) -> String {
    let s = match &*tree {
        Node::Leaf => "".to_string(),
        Node::Branch(value, left, right) => format!("([{:?},{:?}],{:?},{:?})", 
            value, Rc::strong_count(value), 
            add_ref_count(Rc::clone(&left)),
            add_ref_count(Rc::clone(&right)))
   };
   s.replace("\"","")
}


// For branch I have used tupple, could have used named fields (as only 2 parameters and very simple example, I prefer tupple)
// Creating the recursive datastruture, first surprise is the lifetime anotations 
// Lifetime cycle https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html 'a
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}