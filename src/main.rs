use rust_tree::Node;
use rust_tree::add_id;
use rust_tree::add_ref_count;
use std::rc::Rc;

fn main() {
    fn create_node<T>(v:T) -> Rc<Node<T>> { // Type inference is more at the scala level than Haskell/Caml
        let leaf = Rc::new(Node::Leaf); 
        Rc::new(Node::Branch(Rc::new(v),Rc::clone(&leaf),Rc::clone(&leaf))) //explicit reference passing + clone
    };
    let nd = create_node('d'); // I do like in Scala no having to bother with ';'
    let ne = create_node('e'); 
    let nb = create_node('b');
    let nc = Rc::new(Node::Branch(Rc::new('c'),nd,ne)); //no need for clone here as only 1 usage (let removed)
    let na = Rc::new(Node::Branch(Rc::new('a'),nb,nc));
    println!("{:?}", Rc::clone(&na));
    println!("{}", add_ref_count(Rc::clone(&na)));

    let nai = add_id(Rc::clone(&na),0);
    println!("{:?}", nai);
    println!("{}", add_ref_count(Rc::clone(&na)));
    println!("{}", add_ref_count(Rc::clone(&nai.0)));

    // general remark -> very clear and helpful error messages (especially type errors as well as memory management once I understood itS)

    //one of the conclusions:
    //* rust takes some of the most accepted useful features of functional languages (like immutability, lambdas,algebraic data tyoes and pattern matching ), 
    // adds ome specific to haskell (like type classes) and packages them in a language with deterministic performance (i.e. no garbage collection.)
    // Very attractive alternative to C++ which already has lambdas and where pattern matching is on its way (https://github.com/mpark/patterns ) 
    // but targetted to 2023
}
