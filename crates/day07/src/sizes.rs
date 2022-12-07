use std::rc::Rc;

use crate::parser::{FileType, Node};

pub fn dir_sizes(node: &Node) -> Vec<usize> {
    let mut output = vec![];

    output.push(node.size());

    for child in &node.children {
        let cloned_child = Rc::clone(child);
        let borrowed_child = cloned_child.borrow();
        match borrowed_child.value {
            FileType::Directory(_) => {
                let mut sizes = dir_sizes(&borrowed_child);
                output.append(&mut sizes);
            }
            FileType::File(_, _) => {}
        }
    }

    output
}
