#![allow(non_snake_case)]
use linked_list as lib;
use lib::i32_linked_list as i;
use lib::polymorphic_linked_list as p;

pub fn main () {
    
    let _fst = i::create_1node(1); 
    let _snd = i::create_1node(2);
    let _trd = i::create_1node(3);
    let _frt = i::create_node();

    let mut fst = i::link_nodes (_fst, _snd);
    fst = i::link_nodes (fst, _trd);
    fst = i::link_nodes (fst, i::create_node());
    fst = i::link_nodes (fst, _frt);

    println!("{:?}", fst);

    let _pFst = p::create_1node::<bool> (true);
    let _pSnd = p::create_1node::<bool> (false);
    let pFst = p::link_nodes (_pFst, _pSnd);

    println!("{:?}", pFst);

    // Polymorphic Node of Nodes 
    let _pnFst = p::create_1node::<i::Node> (i::create_node());
    let _pnSnd = p::create_1node::<i::Node> (i::create_1node(1));
    // Linking Polymorphic Node of Nodes 
    let pnFst = p::link_nodes (_pnFst, _pnSnd);

    println!("And the last one: {:?}", &pnFst);

    // Let's get the 2nd element of pnFst

    let nElement = 1;
    println!("Element {:?} is: {:?}", nElement, p::get_node(1,&pnFst));
}
