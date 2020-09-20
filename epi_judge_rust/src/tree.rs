use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug, Eq, PartialEq)]
pub struct TreeNode<T> {
    data: T,
    left: Option<Rc<RefCell<TreeNode<T>>>>,
    right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> TreeNode<T>
where
    T: Clone + Debug,
{
    pub fn new(data: T) -> Self {
        TreeNode {
            data,
            left: None,
            right: None,
        }
    }
}

struct Tree<T>(Option<TreeNode<T>>);

impl<T> Tree<T>
where
    T: Clone + Debug,
{
    pub fn new() -> Self {
        Tree(None)
    }

    fn build_tree(&mut self, data: Vec<Option<T>>) {
        let mut nodes: Vec<Option<RefCell<TreeNode<T>>>> = data
            .into_iter()
            .map(|d| d.map(TreeNode::new).map(RefCell::new))
            .collect::<Vec<_>>();

        let mut candidate_children: VecDeque<&Option<RefCell<TreeNode<T>>>> =
            nodes.iter().collect::<VecDeque<_>>();

        candidate_children.pop_front();

        for tnode in nodes.iter() {
            if let Some(node) = tnode {
                if let Some(n) = candidate_children.pop_front() {
                    if let Some(ref n) = n {
                        node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(
                            n.borrow_mut().data.clone(),
                        ))));
                    }
                }
                if let Some(n) = candidate_children.pop_front() {
                    if let Some(ref n) = n {
                        node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(
                            n.borrow_mut().data.clone(),
                        ))));
                    }
                }
            }
        }

        self.0 = Some(nodes.swap_remove(0).unwrap().into_inner());
    }

    fn print_inorder(&self) {
        let mut queue = VecDeque::new();
        queue.push_front(self.0.as_ref());
        println!("{:?}", self.0.as_ref());
    }
}

#[test]
fn test_tree() {
    let v = vec![
        Some(1),
        None,
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        None,
        Some(6),
    ];
    let mut tree = Tree::new();
    tree.build_tree(v);
    tree.print_inorder();
}
