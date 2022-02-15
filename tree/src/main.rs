use std::rc::Rc;
use std::cell::RefCell;
fn main() {
    let testTree = TreeNode::create(88);
    println!("{:?}", testTree);
}

#[derive(Debug)]
struct TreeNode {
    data: i64,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn create(new_data: i64) -> TreeNode {
        TreeNode{
            data: new_data,
            left: None,
            right: None
        }
    }

    fn insert(&mut self, new_data: i64) -> TreeNode {
        if self.data < new_data {
            match self.right {
                None => self.right = Some(TreeNode::create(new_data)),
                Some(_) => self.right.insert(new_data),
            }
        }
    }
}
