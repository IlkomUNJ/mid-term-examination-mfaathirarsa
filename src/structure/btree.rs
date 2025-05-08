use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub type BTreeNodeLink = Rc<RefCell<BTreeNode>>;

#[derive(Debug)]
pub struct BTreeNode {
    pub children: HashMap<i32, BTreeNodeLink>,
    pub terminal: bool,
}

impl BTreeNode {
    pub fn new() -> BTreeNodeLink {
        Rc::new(RefCell::new(BTreeNode {
            children: HashMap::new(),
            terminal: false,
        }))
    }

    pub fn insert(root: &BTreeNodeLink, number: i32) {
        let mut current = Rc::clone(root);
        for digit_char in number.to_string().chars() {
            let digit = digit_char.to_digit(10).unwrap() as i32;
            let next = current.borrow_mut().children.entry(digit).or_insert_with(BTreeNode::new);
            current = Rc::clone(next);
        }
        current.borrow_mut().terminal = true;
    }

    pub fn lookup(node: &BTreeNodeLink, keys: Vec<i32>) -> bool {
        let mut current = Rc::clone(node);
        for digit in keys {
            let next = current.borrow().children.get(&digit);
            match next {
                Some(n) => current = Rc::clone(n),
                None => return false,
            }
        }
        current.borrow().terminal
    }
}
