use std::fmt::Debug;

#[derive(Debug)]
pub struct BTree<T: Ord> {
    v: Option<T>,
    l: Option<Box<BTree<T>>>,
    r: Option<Box<BTree<T>>>,
}

impl<T: Ord> BTree<T>
where
    T: Debug,
{
    pub fn new(v: Option<T>) -> BTree<T> {
        return BTree {
            v,
            l: None,
            r: None,
        };
    }

    /// Insert [`nv`] into [`BTree<T>`].
    pub fn insert(&mut self, nv: T) {
        match &self.v {
            Some(v) => {
                let target_node = if nv < *v { &mut self.l } else { &mut self.r };

                match target_node {
                    Some(ref mut subnode) => subnode.insert(nv),
                    None => {
                        let new_node = BTree {
                            v: Some(nv),
                            l: None,
                            r: None,
                        };
                        let boxed_node = Some(Box::new(new_node));

                        *target_node = boxed_node;
                    }
                }
            }
            None => self.v = Some(nv),
        }
    }

    /// Returns the is empty of this [`BTree<T>`].
    pub fn is_empty(&self) -> bool {
        return self.v.is_none();
    }

    /// Truthy if [`fv`] exists in this [`BTree<T>`].
    pub fn find(&self, fv: T) -> bool {
        match &self.v {
            Some(v) => {
                if *v == fv {
                    return true;
                }

                let node = if fv < *v { &self.l } else { &self.r };

                return match node {
                    Some(ref leaf) => leaf.find(fv),
                    None => false,
                };
            }
            None => false,
        }
    }

    pub fn print_inorder(&self) {
        match &self.v {
            Some(v) => {
                match &self.l {
                    Some(node) => node.print_inorder(),
                    None => {}
                }
                println!("{:?}", v);
                match &self.r {
                    Some(node) => node.print_inorder(),
                    None => {}
                }
            }
            None => return,
        }
    }
}
