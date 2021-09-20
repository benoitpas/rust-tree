pub enum Node<'a, Display> {
    Leaf,
    Branch(Display,& 'a Node<'a,Display>, &'a Node<'a,Display>)
}

//use std::fmt::Display;
//impl<'a,T:Display> Node<'a,T> {
//    fn display(&self) {
//        match self {
//            Node::Leaf =>
//            println!("({},{},{})", self.)
//
 //       }
 //   }
//}

// to finish https://doc.rust-lang.org/std/fmt/trait.Display.html
impl<'a,T> std::fmt::Display for Node<'a,T> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> { 
        todo!() 
    }
}

pub fn to_string<'a,Display>(tree :&'a Node<'a,Display>) -> String {
    match tree {
        Node::Leaf => "",
//            println!("({},{},{})", self.)
        Node::Branch(value, left, right) => "(" + value.to_string() + "," + to_string(left) + "," + to_string(right) + ")"
    }.to_string()
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