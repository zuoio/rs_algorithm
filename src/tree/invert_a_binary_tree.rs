use std::{cell::RefCell, rc::Rc};

use crate::common::{Solution, tree::TreeNode};

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.as_ref() {
            let mut node_ref = node.borrow_mut();
            let left = node_ref.left.take();
            let right = node_ref.right.take();

            node_ref.left = right;
            node_ref.right = left;

            Self::invert_tree(node_ref.left.clone());
            Self::invert_tree(node_ref.right.clone());
        }
        root
    }
}
