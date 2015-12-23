use std::marker::PhantomData;
use std::cmp::Ordering;

#[derive(Copy, Clone)]
struct PhantomVertex;
#[derive(Copy, Clone)]
struct PhantomFace;
#[derive(Copy, Clone)]
struct PhantomEdge;
#[derive(Copy, Clone)]
struct PhantomHalfedge;
pub type Vertex = Handle<PhantomVertex>;
pub type Face = Handle<PhantomFace>;
pub type Edge = Handle<PhantomEdge>;
pub type Halfedge = Handle<PhantomHalfedge>;

#[derive(Copy, Clone)]
pub struct Handle<A> {
    type_ : PhantomData<A>,
    index_ : Option<usize>,
}

impl<A> Handle<A> {
    pub fn invalid() -> Handle<A> {
        Handle {
            type_ : PhantomData,
            index_ : None,
        }
    }

    pub fn new(idx : usize) -> Handle<A> {
        Handle {
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

impl<A> Ord for Handle<A> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index_.cmp(&other.index_)
    }
}

impl<A> PartialOrd for Handle<A> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.index_.cmp(&other.index_))
    }
}

impl<A> PartialEq for Handle<A> {
    fn eq(&self, other: &Self) -> bool {
        self.index_ == other.index_
    }
}
impl<A> Eq for Handle<A> { }

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn invalid() {
        let handle = Vertex::invalid();
        assert!(!handle.is_valid());
        assert!(handle.idx().is_none());
    }

    #[test]
    fn idx() {
        let idx = 42;
        let handle = Vertex::new(idx);
        assert!(handle.is_valid());

        assert!(handle.idx().is_some());
        assert!(handle.idx().unwrap() == idx);
    }

    #[test]
    fn reset() {
        let idx = 42;
        let mut handle = Vertex::new(idx);
        assert!(handle.is_valid());

        handle.reset();
        assert!(!handle.is_valid());
    }

    #[test]
    fn cmp() {
        let idx1 = 42;
        let idx2 = 13;
        let h1 = Vertex::new(idx1);
        let h2 = Vertex::new(idx2);
        let h3 = Vertex::new(idx1);

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
