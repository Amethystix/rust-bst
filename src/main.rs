mod bst;

fn main() {
    let mut bst = bst::BinarySearchTree::new();
    bst.add(5);
    bst.add(1);
    bst.add(10);
    bst.printInOrder();
}
