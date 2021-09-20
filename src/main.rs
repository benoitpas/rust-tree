use rust_tree::Node;

fn main() {
    let leaf = Node::<char>::Leaf; //mention int types including isize
    let nd = Node::<char>::Branch('d',&leaf,&leaf); //explicit reference passing
    let ne = Node::<char>::Branch('e',&leaf,&leaf); // I do like in Scala no having to bother with ';'
    let nc = Node::<char>::Branch('c',&nd,&ne);
    let nb = Node::<char>::Branch('b',&leaf,&leaf);
    let na = Node::<char>::Branch('c',&nb,&nc);
    println!("{}", rust_tree::to_string(&na))

    // general remark -> very clear and helpful error messages

    //one of the conclusions:
    //* rust takes some of the most accepted useful features of functional languages (like immutability, lambdas,algebraic data tyoes and pattern matching ), 
    // adds ome specific to haskell (like tyoe classes) and packages them in a language with deterministic performance (i.e. no garbage collection.)
    // Very attractive alternative to C++ which already has lambdas and where pattern matching is on its way (https://github.com/mpark/patterns ) 
    // but targetted to 2023
}
