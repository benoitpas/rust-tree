use rust_tree::add_id;
use rust_tree::add_ref_count;
use rust_tree::example_tree;
use std::rc::Rc;

fn main() {

    let na = example_tree();
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
    // * memory management main surprise: I did write C/C++ code long time ago but I didn't use much smart pointers at the time, mostly malloc/free and then new/delete (it was before boost becane mainstream).
    // rust model is very well thought out and a huge improvement compared to C++.
}
