// TODO: Make this work with generics

#[derive(Clone)]
pub struct Node {
  data: i32,
  left: Option<Box<Node>>,
  right: Option<Box<Node>>
}

#[derive(Clone)]
pub struct BinarySearchTree {
  root: Option<Box<Node>>
}

impl BinarySearchTree {

  pub fn new() -> Self {
    Self { root: None }
}

  pub fn add(&mut self, value: i32) {
    if let Some(root) = self.root {
      self.root = self.addRec(self.root, value);
    } else {
      self.root = Some(Box::new(Node {
        data: value,
        left: None,
        right: None
      }));
    }
  }

  fn addRec(&mut self, current: Option<Box<Node>>, value: i32) -> Option<Box<Node>> {
    if let Some(curr) = current {
      if value > curr.data {
        curr.right = self.addRec(curr.right, value);
      } else {
        curr.left = self.addRec(curr.left, value);
      }
    } else {
      return Some(Box::new(Node {
        data: value,
        left: None,
        right: None
      }));
    }
    return current;
  }

  pub fn remove(&mut self, value: i32) {
    if let Some(root) = self.root {
      self.root = self.removeRec(Some(root), value);
    }
  }

  fn removeRec(&mut self, current: Option<Box<Node>>, value: i32) -> Option<Box<Node>> {
    if let Some(optCurr) = current {
      let mut curr = *optCurr;
      if value > curr.data {
        curr.right = self.removeRec(curr.right, value);
      } else if value < curr.data {
        curr.left = self.removeRec(curr.left, value);
      } else if value == curr.data {
        // remove leaf
        if curr.left.is_none() && curr.right.is_none() {
          return None;
        } else if curr.left.is_none() {
          return curr.right;
        } else if curr.right.is_none() {
          return curr.left;
        } else if let Some(left) = curr.left {
          if let Some(right) = curr.right {
            let ancestor = self.getAncestor(curr.left);
            if let Some(someAncestor) = ancestor {
              curr.left = self.removeRec(curr.left, (*someAncestor).data);
              curr.data = (*someAncestor).data;
            }
          }
        }
      }
    }
    return current
  }

  fn getAncestor(&mut self, current: Option<Box<Node>>) -> Option<Box<Node>> {
    if let Some(curr) = current {
      if curr.right.is_none() {
        return Some(curr);
      } else {
        return self.getAncestor(curr.right);
      }
    }
    return None;
  }
  
  pub fn printInOrder(&mut self) {
    self.printInOrderRec(self.root)
  }

  fn printInOrderRec(&mut self, current: Option<Box<Node>>) {
    if let Some(curr) = current {
      self.printInOrderRec((*curr).left);
      println!("{}", (*curr).data);
      self.printInOrderRec((*curr).right);
    }
  }
}