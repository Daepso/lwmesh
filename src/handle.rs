use std::marker::PhantomData;
use std::cmp::Ordering;

struct PhantomVertex;
struct PhantomFace;
struct PhantomEdge;
struct PhantomHalfedge;
pub type Vertex = BaseHandle<PhantomVertex>;
pub type Face = BaseHandle<PhantomFace>;
pub type Edge = BaseHandle<PhantomEdge>;
pub type Halfedge = BaseHandle<PhantomHalfedge>;

pub struct BaseHandle<A> {
    type_ : PhantomData<A>,
    index_ : Option<usize>,
}

impl<A> BaseHandle<A> {
    pub fn invalid() -> BaseHandle<A> {
        BaseHandle {
            type_ : PhantomData,
            index_ : None,
        }
    }

    pub fn new(idx : usize) -> BaseHandle<A> {
        BaseHandle {
            type_ : PhantomData,
            index_ : Some(idx),
        }
    }

    pub fn idx(&self) -> Option<usize> {
        self.index_
    }

    pub fn reset(&mut self) {
        self.index_ = None;
    }

    pub fn is_valid(&self) -> bool {
        self.index_.is_some()
    }
}

impl<A> Ord for BaseHandle<A> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index_.cmp(&other.index_)
    }
}

impl<A> PartialOrd for BaseHandle<A> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.index_.cmp(&other.index_))
    }
}

impl<A> PartialEq for BaseHandle<A> {
    fn eq(&self, other: &Self) -> bool {
        self.index_ == other.index_
    }
}
impl<A> Eq for BaseHandle<A> { }

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn base_handle_invalid() {
        let handle : Vertex = Vertex::invalid();
        assert!(!handle.is_valid());
        assert!(handle.idx().is_none());
    }

    #[test]
    fn base_handle_idx() {
        let idx = 42;
        let handle : Vertex = Vertex::new(idx);
        assert!(handle.is_valid());

        assert!(handle.idx().is_some());
        assert!(handle.idx().unwrap() == idx);
    }

    #[test]
    fn base_handle_reset() {
        let idx = 42;
        let mut handle : Vertex = Vertex::new(idx);
        assert!(handle.is_valid());

        handle.reset();
        assert!(!handle.is_valid());
    }

    #[test]
    fn base_handle_cmp() {
        let idx1 = 42;
        let idx2 = 13;
        let h1 : Vertex = Vertex::new(idx1);
        let h2 : Vertex = Vertex::new(idx2);
        let h3 : Vertex = Vertex::new(idx1);

        assert!(h1!=h2);
        assert!(h1>h2);
        assert!(h2<h1);
        assert!(h1==h3);
        assert!(h1<=h3);
        assert_eq!(h2.cmp(&h1), Ordering::Less);
        assert_eq!(h1.cmp(&h2), Ordering::Greater);
        assert_eq!(h1.cmp(&h3), Ordering::Equal);
    }
}
