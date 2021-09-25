use std::rc::Rc;

#[derive(Debug)] // We could have implemented the Diplay trait instead (https://doc.rust-lang.org/std/fmt/trait.Display.html)
pub enum Node<T> {
    Leaf,
    Branch(Rc<T>, Rc<Node<T>>, Rc<Node<T>>) 
}

pub fn add_id<T> (tree: Rc<Node<T>>, i: isize ) -> (Rc<Node<(Rc<T>, isize)>>,isize) {
    match &*tree { // explain &*
        Node::Leaf => (Rc::new(Node::Leaf), i),
        Node::Branch(value, left, right) => {
            let new_left = add_id(Rc::clone(left),i);
            let new_right = add_id(Rc::clone(right),new_left.1+1);
            (Rc::new(Node::Branch(Rc::new((Rc::clone(value),new_left.1)),new_left.0,new_right.0)),new_right.1)
        }
    }
}

//pub fn add_refCount<T> () (tree: Rc<Node<T>>) -> (Rc<Node<T,isize>>) {
//    match &*tree {
//
//   }
//}


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