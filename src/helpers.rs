use std::cell::RefMut;

use super::*;

/// "Send" message to another `Node`
pub(crate) fn send_msg<T>(node: &mut RefMut<T>, msg: Message) -> Result<(), Error>
where
    T: RaftNode,
{
    node.on_rcv_message(msg)
}

#[macro_export]
macro_rules! easy_connect {
    ($node:expr, $($other_nodes:expr),*) => {
        {
            let nodes = vec![$node, $($other_nodes),*];
            for &ref node in nodes.iter() {
                let mut connected_nodes = Vec::new();
                for &ref other_node in nodes.iter() {
                    if !Rc::ptr_eq(node, other_node) {
                        connected_nodes.push(other_node.clone());
                    }
                }
                node.borrow_mut().connect_nodes(&connected_nodes);
            }
        }
    };
}
