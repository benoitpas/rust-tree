enum Node<'a,T> {
    Leaf,
    Branch(&'a Node<'a,T>,T),
}

//fn id<'a>(tree :&'a Node) -> &Node<'a> {
//    id
//}

// For branch I have used tupple, could have used named fields (as only 2 parameters and very simple example, I prefer tupple)
// Lifetime cycle https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html 'a
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}