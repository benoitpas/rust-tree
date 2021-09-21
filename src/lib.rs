pub enum Node<'a> {
    Leaf,
    Branch(&'a dyn std::fmt::Display, &'a Node<'a>, &'a Node<'a>) //Need to explain 'dyn'
}

// to finish https://doc.rust-lang.org/std/fmt/trait.Display.html
impl<'a> std::fmt::Display for Node<'a> {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> { 
        fn add_comma(input : String) -> String { 
            if input == "" {input} else {" ,".to_string() + &input}
        }
    
        let s = match self {
            Node::Leaf => "".to_string(),
            Node::Branch(value, left, right) => format!("({}{}{})",value.to_string(),add_comma(left.to_string()),add_comma(right.to_string()))
        };
        write!(f, "{}", s)
    }
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