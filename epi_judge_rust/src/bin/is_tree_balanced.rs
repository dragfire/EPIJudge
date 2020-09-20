use epi_judge_rust::tree::{Tree, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

/// Problem: Write a program that takes as input the root of a binary tree and checks whether
/// the tree is height-balanced.
fn is_tree_balanced<T>(tree: Tree<i32>) -> bool {
    helper(tree.root()).1
}

fn helper(curr: Option<Rc<RefCell<TreeNode<i32>>>>) -> (i32, bool) {
    let mut height = 0;
    let mut balanced = true;

    if let Some(node) = curr {
        let l = helper(node.borrow().left.clone());
        let r = helper(node.borrow().right.clone());
        if !l.1 {
            return l;
        }
        if !r.1 {
            return r;
        }
        balanced = (l.0 - r.0).abs() <= 1;
        height = std::cmp::max(l.0, r.0) + 1;
    }

    (height, balanced)
}

fn main() {
    epi_judge_rust::run_tests(
        "is_tree_balanced.tsv",
        |data| -> epi_judge_rust::Result<()> {
            let nodes = serde_json::from_str::<Vec<Option<i32>>>(&data[0]).unwrap();
            let expected = serde_json::from_str::<bool>(&data[1]).unwrap();

            let mut tree = Tree::new();
            tree.build_tree(nodes);

            let actual = is_tree_balanced::<i32>(tree);

            epi_judge_rust::try_assert!(actual, expected)
        },
    );
}
