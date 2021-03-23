use std::fmt;

macro_rules! insert_at {
    ($node:expr, $value:expr) => {
        {
            (move || {
                if let Some(ref mut node) = $node.as_mut() {
                    node.insert($value);
                    return
                }
                $node = Some(Box::new(BinaryNode::new($value)));
            })()
        }
    };
}

macro_rules! fmt_node {
    ($fmt:expr, $node:expr) => {
        if let Some(ref node) = $node.as_ref() {
            node.fmt($fmt)?;
        }
    };
}


#[derive(Debug)]
struct BinaryNode<T> where T: Ord {
    data: T,
    left: Option<Box<BinaryNode<T>>>,
    right: Option<Box<BinaryNode<T>>>,
}

#[derive(Debug)]
pub struct BinaryTree<T> where T: Ord {
    root: Option<Box<BinaryNode<T>>>,
}


impl<T> BinaryTree<T> where T: Ord {
    pub fn new() -> Self {
        BinaryTree {
            root: None,
        }
    }

    pub fn insert(&mut self, data: T){
        insert_at!(self.root, data);
    }
}

impl<T> fmt::Display for BinaryTree<T> where T: Ord + fmt::Display {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "[")?;
        fmt_node!(fmt, self.root);
        write!(fmt, "]")?;
        Ok(())
    }
}

impl<T> BinaryNode<T> where T: Ord {
    fn new(data: T) -> Self {
        BinaryNode {
            data: data,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, data: T) {
        if data <= self.data {
            insert_at!(self.left, data);
        }
        else {
            insert_at!(self.right, data);
        }
    }
}


impl<T> fmt::Display for BinaryNode<T> where T: Ord + fmt::Display {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt_node!(fmt, self.left);
        write!(fmt, "{}, ", self.data)?;
        fmt_node!(fmt, self.right);
        Ok(())
    }
}


// TODO: write proper tests
#[cfg(test)]
mod tests {
    #[test]
    fn binary_tree_new() {
        let mut tree = BinaryTree::new();
        tree.insert(2);
        tree.insert(0);
        tree.insert(3);
        tree.insert(1);
        tree.insert(4);
        println!("{:#?}", tree);
        println!("{}", tree);
        assert_eq!(4,4);
    }
}

