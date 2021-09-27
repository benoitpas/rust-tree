use std::rc::Rc;

// For branch I have used tupple, could have used named fields (as only 2 parameters and very simple example, I prefer tupple)
// Creating the recursive datastruture, first surprise is the lifetime anotations
// Lifetime cycle https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html 'a


#[derive(Debug)] // We could have implemented the Diplay trait instead (https://doc.rust-lang.org/std/fmt/trait.Display.html)
pub enum Node<T> {
    Leaf,
    Branch(Rc<T>, Rc<Node<T>>, Rc<Node<T>>) 
}

// Automatically available in Haskell/Scala
impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self,other) {
            (Node::Leaf ,Node::Leaf ) => true,
            (Node::Branch(s_value, s_left, s_right),Node::Branch(o_value, o_left, o_right) )
                => o_value == s_value && s_left == o_left && s_right == o_right,
            (_,_) => false
        }
    }
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




pub fn example_tree() -> Rc<Node<char>> {

    fn create_node<T>(v:T) -> Rc<Node<T>> { // Type inference is more at the scala level than Haskell/Caml
        let leaf = Rc::new(Node::Leaf);
        Rc::new(Node::Branch(Rc::new(v),Rc::clone(&leaf),Rc::clone(&leaf))) //explicit reference passing + clone
    }

    let nd = create_node('d'); // I do like in Scala no having to bother with ';'
    let ne = create_node('e');
    let nb = create_node('b');
    let nc = Rc::new(Node::Branch(Rc::new('c'),nd,ne)); //no need for clone here as only 1 usage (let removed)
    let na = Rc::new(Node::Branch(Rc::new('a'),nb,nc));
    na
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_id() {
        let na = example_tree();

        fn create_node<T>(v:T, i:isize) -> Rc<Node<(Rc<T>,isize)>> { // Type inference is more at the scala level than Haskell/Caml
            let leaf = Rc::new(Node::Leaf);
            Rc::new(Node::Branch(Rc::new((Rc::new(v),i)),Rc::clone(&leaf),Rc::clone(&leaf))) //explicit reference passing + clone
        }
        let nd = create_node('d',2);
        let ne = create_node('e',4);
        let nb = create_node('b',0);
        let nc = Rc::new(Node::Branch(Rc::new((Rc::new('c'),3)),nd,ne)); //no need for clone here as only 1 usage (let removed)
        let na_expected = Rc::new(Node::Branch(Rc::new((Rc::new('a'),1)),nb,nc));

        assert_eq!(na_expected, add_id(na,0).0);
    }
}