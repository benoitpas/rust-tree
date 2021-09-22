#[derive(Debug)] // We could have implemented the Diplay trait instead (https://doc.rust-lang.org/std/fmt/trait.Display.html)
pub enum Node<'a, T> {
    Leaf,
    Branch(T, &'a Node<'a, T>, &'a Node<'a, T>) 
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