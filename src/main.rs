mod _344;
mod leetcode;

use std::rc::Rc;
use std::cell::RefCell;
use crate::leetcode::TreeNode;

fn main() {

    let treeTestCase =  Option::from(Rc::from(RefCell::from(TreeNode {val: 0, 
        left: Option::from(Rc::from(RefCell::from(TreeNode {val: 0, left: None, right: None}))), 
        right: Option::from(Rc::from(RefCell::from(TreeNode {val: 1, left: None, right: None})))
    })));

    let mut case = vec!('1', '2' , '3' , '4', '5');
    _344::reverse_string(&mut case);
    println!("{:?}", case);
}
