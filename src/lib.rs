use std::fmt;
mod traits;

pub trait SimpleVertex: graph_matrix::MxElement + fmt::Display + traits::Vertex {}

impl traits::Vertex for u8 {}
impl traits::Vertex for u16 {}
impl traits::Vertex for u32 {}
impl traits::Vertex for usize {}
impl traits::Vertex for u64 {}

impl SimpleVertex for u8 {}
impl SimpleVertex for u16 {}
impl SimpleVertex for u32 {}
impl SimpleVertex for usize {}
impl SimpleVertex for u64 {}

pub struct Vertices<V> {
    start: V,
    end: V,
}

impl<V> Iterator for Vertices<V>
where
    V: SimpleVertex,
{
    type Item = V;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }
        self.start += V::one();
        Some(self.start)
    }
}

pub struct SimpleEdge<V>
where
    V: graph_matrix::MxElement + graph_matrix::MxElement,
{
    src: V,
    dst: V,
}

impl<V> traits::Edge<V> for SimpleEdge<V>
where
    V: traits::Vertex + graph_matrix::MxElement,
{
    fn src(&self) -> V {
        self.src
    }

    fn dst(&self) -> V {
        self.dst
    }
}
pub struct StaticGraph<V>
where
    V: traits::Vertex + graph_matrix::MxElement,
{
    fadj: graph_matrix::GraphMatrix<V>,
    badj: graph_matrix::GraphMatrix<V>,
}

impl<V> traits::Graph<V, SimpleEdge<V>> for StaticGraph<V>
where
    V: SimpleVertex + traits::Vertex + graph_matrix::MxElement,
{
    type VIT = Vertices<V>;
    fn nv(&self) -> usize {
        usize::from(self.fadj.dim())
    }

    fn ne(&self) -> usize {
        self.fadj.n()
    }

    fn vertices(&self) -> Vertices<V> {
        Vertices {
            start: V::zero(),
            end: V::from(self.nv()).expect("vertex out of range"),
        }
    }
}
