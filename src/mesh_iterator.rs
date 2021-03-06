use mesh::*;
use handle::*;

pub struct VertexIterator<'a> {
    topology_ : &'a Topology,
    curr_ : Vertex,
}

impl<'a> Iterator for VertexIterator<'a> {
    type Item = Vertex;

    fn next(&mut self) -> Option<Vertex> {
        let v = self.curr_;
        self.curr_ = Vertex::new(v.idx()+1);
        if self.topology_.n_vertices() <= v.idx() {
            return None;
        } else {
            return Some(v);
        }
    }
}

pub struct FaceIterator<'a> {
    topology_ : &'a Topology,
    curr_ : Face,
}

impl<'a> Iterator for FaceIterator<'a> {
    type Item = Face;

    fn next(&mut self) -> Option<Face> {
        let f = self.curr_;
        self.curr_ = Face::new(f.idx()+1);
        if self.topology_.n_faces() <= f.idx() {
            return None;
        } else {
            return Some(f);
        }
    }
}

pub struct EdgeIterator<'a> {
    topology_ : &'a Topology,
    curr_ : Edge,
}

impl<'a> Iterator for EdgeIterator<'a> {
    type Item = Edge;

    fn next(&mut self) -> Option<Edge> {
        let e = self.curr_;
        self.curr_ = Edge::new(e.idx()+1);
        if self.topology_.n_edges() <= e.idx() {
            return None;
        } else {
            return Some(e);
        }
    }
}

pub struct HalfedgeIterator<'a> {
    topology_ : &'a Topology,
    curr_ : Halfedge,
}

impl<'a> Iterator for HalfedgeIterator<'a> {
    type Item = Halfedge;

    fn next(&mut self) -> Option<Halfedge> {
        let h = self.curr_;
        self.curr_ = Halfedge::new(h.idx()+1);
        if self.topology_.n_halfedges() <= h.idx() {
            return None;
        } else {
            return Some(h);
        }
    }
}

pub struct VerticesAroundVertexCirculator<'a> {
    topology_ : &'a Topology,
    end_ : Option<Halfedge>,
    curr_ : Option<Halfedge>,
    active_ : bool
}

impl<'a> Iterator for VerticesAroundVertexCirculator<'a> {
    type Item = Vertex;

    fn next(&mut self) -> Option<Vertex> {
        if self.curr_.is_none() {
            return None;
        }
        if self.active_ && self.curr_ == self.end_ {
            return None;
        }
        self.active_ = true;
        let v = self.topology_.to_vertex(self.curr_.unwrap());
        self.curr_ = Some(self.topology_.cw_rotated_halfedge(self.curr_.unwrap()));
        return Some(v);
    }
}

pub struct HalfedgesAroundVertexCirculator<'a> {
    topology_ : &'a Topology,
    end_ : Option<Halfedge>,
    curr_ : Option<Halfedge>,
    active_ : bool
}

impl<'a> Iterator for HalfedgesAroundVertexCirculator<'a> {
    type Item = Halfedge;

    fn next(&mut self) -> Option<Halfedge> {
        if self.curr_.is_none() {
            return None;
        }
        if self.active_ && self.curr_ == self.end_ {
            return None;
        }
        self.active_ = true;
        let h = self.curr_.unwrap();
        self.curr_ = Some(self.topology_.cw_rotated_halfedge(self.curr_.unwrap()));
        return Some(h);
    }
}

pub struct FacesAroundVertexCirculator<'a> {
    topology_ : &'a Topology,
    end_ : Option<Halfedge>,
    curr_ : Option<Halfedge>,
    active_ : bool
}

impl<'a> Iterator for FacesAroundVertexCirculator<'a> {
    type Item = Face;

    fn next(&mut self) -> Option<Face> {
        if self.curr_.is_none() {
            return None;
        }
        if self.active_ && self.curr_ == self.end_ {
            return None;
        }
        self.active_ = true;
        let f = self.topology_.face(self.curr_.unwrap()).unwrap();
        loop {
            self.curr_ = Some(self.topology_.cw_rotated_halfedge(self.curr_.unwrap()));
            if !self.topology_.is_boundary_halfedge(self.curr_.unwrap()) {break;}
        }
        return Some(f);
    }
}

pub struct VerticesAroundFaceCirculator<'a> {
    topology_ : &'a Topology,
    end_ : Halfedge,
    curr_ : Halfedge,
    active_ : bool
}

impl<'a> Iterator for VerticesAroundFaceCirculator<'a> {
    type Item = Vertex;

    fn next(&mut self) -> Option<Vertex> {
        if self.active_ && self.curr_ == self.end_ {
            return None;
        }
        self.active_ = true;
        let v = self.topology_.to_vertex(self.curr_);
        self.curr_ = self.topology_.next_halfedge(self.curr_);
        return Some(v);
    }
}

pub struct HalfedgesAroundFaceCirculator<'a> {
    topology_ : &'a Topology,
    end_ : Halfedge,
    curr_ : Halfedge,
    active_ : bool
}

impl<'a> Iterator for HalfedgesAroundFaceCirculator<'a> {
    type Item = Halfedge;

    fn next(&mut self) -> Option<Halfedge> {
        if self.active_ && self.curr_ == self.end_ {
            return None;
        }
        self.active_ = true;
        let h = self.curr_;
        self.curr_ = self.topology_.next_halfedge(self.curr_);
        return Some(h);
    }
}

pub trait VerticesAround<'a,H,I> {
    fn vertices_around(&'a self, handle : H) -> I;
}

impl<'a> VerticesAround<'a,Vertex,VerticesAroundVertexCirculator<'a> > for Topology {
    /// Iterator over the vertices around a vertex in the `Mesh`
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::*;
    ///
    /// let mut m = Mesh::new();
    /// let mut vvec = Vec::<Vertex>::new();
    /// for _ in 0..3 {
    ///    vvec.push(m.add_vertex());
    /// }
    /// m.add_face(&vvec);
    ///
    /// for v in m.topology.vertices_around(vvec[0]) {
    ///     println!("v{}",v.idx());
    /// }
    /// ```
    fn vertices_around(&'a self, v : Vertex) -> VerticesAroundVertexCirculator<'a> {
        VerticesAroundVertexCirculator::<'a> {
            topology_ : &self,
            end_ : self.halfedge(v),
            curr_ : self.halfedge(v),
            active_ : false
        }
    }
}


impl<'a> VerticesAround<'a,Face,VerticesAroundFaceCirculator<'a> > for Topology {
    /// Iterator over the vertices in a face in the `Mesh`
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::*;
    ///
    /// let mut m = Mesh::new();
    /// let mut vvec = Vec::<Vertex>::new();
    /// for _ in 0..3 {
    ///    vvec.push(m.add_vertex());
    /// }
    /// let f = m.add_face(&vvec).unwrap();
    ///
    /// for v in m.topology.vertices_around(f) {
    ///     println!("v{}",v.idx());
    /// }
    /// ```
    fn vertices_around(&'a self, f : Face) -> VerticesAroundFaceCirculator<'a> {
        VerticesAroundFaceCirculator::<'a> {
            topology_ : &self,
            end_ : self.face_halfedge(f),
            curr_ : self.face_halfedge(f),
            active_ : false
        }
    }
}


pub trait HalfedgesAround<'a,H,I> {
    fn halfedges_around(&'a self, handle : H) -> I;
}

impl<'a> HalfedgesAround<'a,Vertex,HalfedgesAroundVertexCirculator<'a> > for Topology {
    /// Iterator over the halfedges around a vertex in the `Mesh`
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::*;
    ///
    /// let mut m = Mesh::new();
    /// let mut vvec = Vec::<Vertex>::new();
    /// for _ in 0..3 {
    ///    vvec.push(m.add_vertex());
    /// }
    /// m.add_face(&vvec);
    ///
    /// for h in m.topology.halfedges_around(vvec[0]) {
    ///     println!("h{}",h.idx());
    /// }
    /// ```
    fn halfedges_around(&self, v : Vertex) -> HalfedgesAroundVertexCirculator {
        HalfedgesAroundVertexCirculator {
            topology_ : &self,
            end_ : self.halfedge(v),
            curr_ : self.halfedge(v),
            active_ : false
        }
    }
}

impl<'a> HalfedgesAround<'a,Face,HalfedgesAroundFaceCirculator<'a> > for Topology {
    /// Iterator over the halfedges in a face in the `Mesh`
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::*;
    ///
    /// let mut m = Mesh::new();
    /// let mut vvec = Vec::<Vertex>::new();
    /// for _ in 0..3 {
    ///    vvec.push(m.add_vertex());
    /// }
    /// let f = m.add_face(&vvec).unwrap();
    ///
    /// for h in m.topology.halfedges_around(f) {
    ///     println!("h{}",h.idx());
    /// }
    /// ```
    fn halfedges_around(&self, f : Face) -> HalfedgesAroundFaceCirculator {
        HalfedgesAroundFaceCirculator {
            topology_ : &self,
            end_ : self.face_halfedge(f),
            curr_ : self.face_halfedge(f),
            active_ : false
        }
    }
}

pub trait FacesAround<'a,H,I> {
    fn faces_around(&'a self, handle : H) -> I;
}

impl<'a> FacesAround<'a,Vertex,FacesAroundVertexCirculator<'a> > for Topology {
    /// Iterator over the faces around a vertex in the `Mesh`
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::*;
    ///
    /// let mut m = Mesh::new();
    /// let mut vvec = Vec::<Vertex>::new();
    /// for _ in 0..3 {
    ///    vvec.push(m.add_vertex());
    /// }
    /// m.add_face(&vvec);
    ///
    /// for f in m.topology.faces_around(vvec[0]) {
    ///     println!("f{}",f.idx());
    /// }
    /// ```
    fn faces_around(&self, v : Vertex) -> FacesAroundVertexCirculator {
        match self.halfedge(v) {
            None => FacesAroundVertexCirculator {
                topology_ : &self,
                end_ : None,
                curr_ : None,
                active_ : false
            },
            Some(x) => {
                let mut h = x;
                while self.is_boundary_halfedge(h) {
                    h = self.cw_rotated_halfedge(h);
                }
                FacesAroundVertexCirculator {
                    topology_ : &self,
                    end_ : Some(h),
                    curr_ : Some(h),
                    active_ : false
                }
            },
        }
    }
}

impl Topology {
    /// Iterator over the vertices in the `Mesh`
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::Mesh;
    ///
    /// let mut m = Mesh::new();
    /// m.add_vertex();
    /// m.add_vertex();
    /// m.add_vertex();
    ///
    /// for v in m.topology.vertices() {
    ///     println!("v{}",v.idx())
    /// }
    /// ```
    pub fn vertices(&self) -> VertexIterator {
        VertexIterator {
            topology_ : &self,
            curr_ : Vertex::new(0),
        }
    }

    /// Iterator over the faces in the `Mesh`
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::*;
    ///
    /// let mut m = Mesh::new();
    /// let mut vvec = Vec::<Vertex>::new();
    /// for _ in 0..3 {
    ///    vvec.push(m.add_vertex());
    /// }
    /// m.add_face(&vvec);
    ///
    /// for f in m.topology.faces() {
    ///     println!("f{}",f.idx())
    /// }
    /// ```
    pub fn faces(&self) -> FaceIterator {
        FaceIterator {
            topology_ : &self,
            curr_ : Face::new(0),
        }
    }

    /// Iterator over the edges in the `Mesh`
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::*;
    ///
    /// let mut m = Mesh::new();
    /// let mut vvec = Vec::<Vertex>::new();
    /// for _ in 0..3 {
    ///    vvec.push(m.add_vertex());
    /// }
    /// m.add_face(&vvec);
    ///
    /// for e in m.topology.edges() {
    ///     println!("e{}",e.idx())
    /// }
    /// ```
    pub fn edges(&self) -> EdgeIterator {
        EdgeIterator {
            topology_ : &self,
            curr_ : Edge::new(0),
        }
    }

    /// Iterator over the halfedges in the `Mesh`
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::*;
    ///
    /// let mut m = Mesh::new();
    /// let mut vvec = Vec::<Vertex>::new();
    /// for _ in 0..3 {
    ///    vvec.push(m.add_vertex());
    /// }
    /// m.add_face(&vvec);
    ///
    /// for h in m.topology.halfedges() {
    ///     println!("h{}",h.idx())
    /// }
    /// ```
    pub fn halfedges(&self) -> HalfedgeIterator {
        HalfedgeIterator {
            topology_ : &self,
            curr_ : Halfedge::new(0),
        }
    }
}

#[cfg(test)]
mod tests {
    use mesh::*;
    use handle::Vertex;
    use mesh_iterator::VerticesAround;
    use mesh_iterator::HalfedgesAround;
    use mesh_iterator::FacesAround;

    #[test]
    fn iterator_and_properties() {
        let mut m = Mesh::new();
        let vprop = m.properties.add_vertex_property::<u32>("v:my_prop",17).unwrap();
        let fprop = m.properties.add_face_property::<u32>("f:my_prop",17).unwrap();
        let eprop = m.properties.add_edge_property::<u32>("e:my_prop",17).unwrap();
        let hprop = m.properties.add_halfedge_property::<u32>("h:my_prop",17).unwrap();
        let mut vvec = Vec::<Vertex>::new();
        for _ in 0..3 {
            vvec.push(m.add_vertex());
        }
        m.add_face(&vvec);

        for v in m.topology.vertices() {
            m.properties[(vprop,v)] += 1;
            assert_eq!(18,m.properties[(vprop,v)]);
        }

        for f in m.topology.faces() {
            m.properties[(fprop,f)] += 1;
            assert_eq!(18,m.properties[(fprop,f)]);
        }

        for e in m.topology.edges() {
            m.properties[(eprop,e)] += 1;
            assert_eq!(18,m.properties[(eprop,e)]);
        }

        for h in m.topology.halfedges() {
            m.properties[(hprop,h)] += 1;
            assert_eq!(18,m.properties[(hprop,h)]);
        }
    }

    #[test]
    fn around_iterator() {
        let mut m = Mesh::new();
        let mut vvec = Vec::<Vertex>::new();
        for _ in 0..3 {
            vvec.push(m.add_vertex());
        }
        let f = m.add_face(&vvec).unwrap();

        let mut i = 0;
        for _ in m.topology.vertices_around(vvec[0]) {
            i += 1;
        }
        assert_eq!(i,2);

        let mut i = 0;
        for _ in m.topology.halfedges_around(vvec[0]) {
            i += 1;
        }
        assert_eq!(i,2);

        let mut i = 0;
        for _ in m.topology.faces_around(vvec[0]) {
            i += 1;
        }
        assert_eq!(i,1);

        let mut i = 0;
        for _ in m.topology.vertices_around(f) {
            i += 1;
        }
        assert_eq!(i,3);

        let mut i = 0;
        for _ in m.topology.halfedges_around(f) {
            i += 1;
        }
        assert_eq!(i,3);
    }

    #[test]
    fn empty_iterator() {
        let mut m = Mesh::new();
        let v = m.add_vertex();
        let mut i = 0;
        for _ in m.topology.vertices_around(v) {
            i += 1;
        }
        assert_eq!(i,0);
        let mut i = 0;
        for _ in m.topology.faces_around(v) {
            i += 1;
        }
        assert_eq!(i,0);
        let mut i = 0;
        for _ in m.topology.halfedges_around(v) {
            i += 1;
        }
        assert_eq!(i,0);
    }
}
