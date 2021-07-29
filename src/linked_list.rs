/// # Linked List Module
///
/// This is the first implementation of a linked list that I managed to do
/// from scratch. I think it's not too shabby.
pub mod i32_linked_list {
    
    /// # Node Struct
    ///
    /// A simple `Node` implementation that can hold a `i32`.
    #[derive(Debug, Clone)] 
    pub struct Node {
        next_node :  Option <Box <Node>> ,
        value: i32 ,
    }
    
    /// This is a simple utility function to create a single `Node`.
    ///
    /// # Arguments
    /// * `val` - the intialisation value of the `Node`.
    ///
    /// # Examples
    ///
    /// ```
    /// use linked_list::i32_linked_list::create_1node;
    /// create_1node (32); // -> returns a node holding the
    ///                    //    value 32
    /// ```
    pub fn create_1node (val:i32) -> Node {
        Node {
            next_node: None,
            value: val,
        }
    }
    
    /// Similar to 'create_1node` this function will create a single `Node` but
    /// this time populate it with a defaul value.
    pub fn create_node () -> Node {
        let a : Node  = Node {
            next_node: None,
            value: 32,
        };
        return a;
    }

    /// Will link two `Nodes`. If the first `Node` is already linked, it will
    /// carry on and add it to the linked list of `Nodes`.
    pub fn link_nodes (a: Node, b: Node) -> Node {
        match a.next_node {
            None => Node {
                value: a.value,
                next_node: Some(Box::new(b)),
            },
            Some(c) => Node {
                value: a.value,
                next_node: Some (Box::new(link_nodes(*c,b))),
            }
        }
    }
}

pub mod polymorphic_linked_list  {
    /// # Polymorphic Node Struct
    ///
    /// A simple `Node` implementation that can hold any value.
    #[derive(Debug, Clone)] 
    pub struct Node <A> {
        next_node :  Option <Box <Node::<A>>> ,
        value: A ,
    }
    
    /// This is a simple utility function to create a single `Node`.
    ///
    /// # Arguments
    /// * `val` - the intialisation value of the `Node`.
    ///
    /// # Examples
    ///
    /// ```
    /// use linked_list::polymorphic_linked_list::create_1node;
    /// create_1node::<i32> (32);    // -> returns a node holding the
    ///                              //    value `32`.
    /// create_1node::<bool> (true); // -> returns a node holding the
    ///                              //    value `true`.
    /// ```
    pub fn create_1node <A> (val:A) -> Node <A> {
        Node::<A> {
            next_node: None,
            value: val,
        }
    }
    
    /// Will link two `Nodes`. If the first `Node` is already linked, it will
    /// carry on and add it to the linked list of `Nodes`.
    pub fn link_nodes <A> (a: Node<A>, b: Node<A>) -> Node<A> {
        match a.next_node {
            None => Node::<A> {
                value: a.value,
                next_node: Some(Box::new(b)),
            },
            Some(c) => Node::<A> {
                value: a.value,
                next_node: Some (Box::new(link_nodes(*c,b))),
            }
        }
    }

    /// Equivalent of !! in Haskell.
    pub fn get_node <A> (a: i32, x: &Node<A>) -> &Node<A>{
        if a < 0 { x }
        else {
            match &x.next_node {
                None => &x,
                Some (c) => get_node (a-1, &c)
            }
        }
    }
}
