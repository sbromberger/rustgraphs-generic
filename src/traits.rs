use std::path::Path;
use num;


pub trait Vertex: Clone {
    type T: num::Bounded + num::cast::NumCast + std::cmp::Ord + num::Zero + num::One;  // this is a type that will be used for methods that return an integer type (out_degree, etc.)
    fn index(&self) -> Self::T;
    fn from_index(x:Self::T) -> Self;
    fn sentinel() -> Self;
}

// Anything that can cast to primitive can use to_usize.
impl<T: Clone + num::Bounded + num::cast::NumCast + std::cmp::Ord + num::Zero + num::One> Vertex for T {
    type T = T;
    fn index(&self) -> T { *self }
    fn from_index(x:T) -> Self { x as Self }
    fn sentinel() -> Self { Self::max_value() }
}

pub trait Edge<V>
{
    fn src(&self) -> V;
    fn dst(&self) -> V;
}

pub trait WeightedEdge<V, W> : Edge<V>
{
    fn weight(&self) -> W;
}

pub trait Graph<V, E>
where
    V: Vertex,
    E: Edge<V>,
{
    type VIT: Iterator<Item = V>;

    fn nv(&self) -> V::T;
    fn ne(&self) -> usize;
    fn vertices(&self) -> Self::VIT;
    fn in_degree(&self, v: V) -> V::T;
    fn out_degree(&self, v: V) -> V::T;
    fn in_neighbors(&self, v: V) -> &[V];
    fn out_neighbors(&self, v: V) -> &[V];
    fn has_edge(&self, e: E) -> bool;
    fn from_edge_file(fname: &Path) -> Self;
}

