use std::rc::Rc;
use std::cell::RefCell;
use crate::leetcode::TreeNode;


pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {

    fn is_unival_tree_ref(root: Option<&Rc<RefCell<TreeNode>>>) -> bool {

        if root.is_none() {
            return true;
        }
    
        let rnb = root.unwrap().borrow();
        let root_v = rnb.val;
    
    
    
        let left = rnb.left.as_ref();
        let left_r = is_unival_tree_ref(left);
        let left_v = if left.is_none() { root_v } else { left.unwrap().borrow().val };
    
    
        let right = rnb.right.as_ref();
        let right_r = is_unival_tree_ref(right);
        let right_v = if right.is_none() { root_v } else { right.unwrap().borrow().val };
    
        return left_r && right_r && root_v == left_v && root_v == right_v;
    }
    

    return is_unival_tree_ref(root.as_ref());
    
}

