use rust_tree::Node;
use rust_tree::add_id;
use std::rc::Rc;

fn main() {
    let leaf = Rc::new(Node::Leaf); //mention int types including isize
    let nd = Rc::new(Node::Branch(Rc::new('d'),Rc::clone(&leaf),Rc::clone(&leaf))); //explicit reference passing + clone
    let ne = Rc::new(Node::Branch(Rc::new('e'),Rc::clone(&leaf),Rc::clone(&leaf))); // I do like in Scala no having to bother with ';'
    let nb = Rc::new(Node::Branch(Rc::new('b'),Rc::clone(&leaf),Rc::clone(&leaf)));
    let nc = Rc::new(Node::Branch(Rc::new('c'),nd,ne)); //no need for clone here as only 1 usage (let removed)
    let na = Rc::new(Node::Branch(Rc::new('a'),nb,nc));
    println!("{:?}", na);

    let nai = add_id(na,0);
    println!("{:?}", nai);

    // general remark -> very clear and helpful error messages

    //one of the conclusions:
    //* rust takes some of the most accepted useful features of functional languages (like immutability, lambdas,algebraic data tyoes and pattern matching ), 
    // adds ome specific to haskell (like tyoe classes) and packages them in a language with deterministic performance (i.e. no garbage collection.)
    // Very attractive alternative to C++ which already has lambdas and where pattern matching is on its way (https://github.com/mpark/patterns ) 
    // but targetted to 2023
}
