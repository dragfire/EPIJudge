use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Eq, PartialEq, Debug)]
pub struct TreeNode<T> {
    pub data: T,
    pub left: Option<Rc<RefCell<TreeNode<T>>>>,
    pub right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> std::fmt::Display for TreeNode<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Node: {}", self.data)
    }
}

impl<T> TreeNode<T> {
    pub fn new(data: T) -> Self {
        TreeNode {
            data,
            left: None,
            right: None,
        }
    }
}

/// Wrap a TreeNode as root node.
///
pub struct Tree<T>(Option<Rc<RefCell<TreeNode<T>>>>);

impl<T> Tree<T>
where
    T: std::fmt::Display,
{
    pub fn new() -> Self {
        Tree(None)
    }

    pub fn root(&self) -> Option<Rc<RefCell<TreeNode<T>>>> {
        self.0.clone()
    }

    pub fn build_tree(&mut self, data: Vec<Option<T>>) {
        let mut nodes: Vec<Option<Rc<RefCell<TreeNode<T>>>>> = data
            .into_iter()
            .map(|d| d.map(TreeNode::new).map(RefCell::new).map(Rc::new))
            .collect::<Vec<_>>();

        let mut candidate_children: VecDeque<Option<Rc<RefCell<TreeNode<T>>>>> =
            VecDeque::from(nodes.clone());

        candidate_children.pop_front();

        for tnode in nodes.iter() {
            if candidate_children.is_empty() {
                break;
            }
            if let Some(node) = tnode {
                if let Some(n) = candidate_children.pop_front() {
                    if let Some(ref n) = n {
                        node.borrow_mut().left = Some(n.clone());
                    }
                }
                if let Some(n) = candidate_children.pop_front() {
                    if let Some(ref n) = n {
                        node.borrow_mut().right = Some(n.clone());
                    }
                }
            }
        }

        self.0 = nodes.swap_remove(0);
    }

    pub fn print_inorder(&self) {
        let curr = self.0.clone();
        Tree::print_inorder_helper(curr);
    }

    fn print_inorder_helper(curr: Option<Rc<RefCell<TreeNode<T>>>>) {
        match curr {
            Some(ref node) => {
                let cloned_node = node.clone();
                Tree::print_inorder_helper(cloned_node.borrow().left.clone());
                print!("{}, ", cloned_node.borrow());
                Tree::print_inorder_helper(cloned_node.borrow().right.clone());
            }
            None => return,
        }
    }
}

#[test]
fn test_tree() {
    let v = vec![Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(6)];
    let mut tree = Tree::new();
    tree.build_tree(v);
    tree.print_inorder();
}
