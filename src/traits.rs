use std::path::Path;
pub trait Vertex {}

pub trait Edge<V>
where
    V: Vertex,
{
    fn src(&self) -> V;
    fn dst(&self) -> V;
}

pub trait Graph<V, E>
where
    V: Vertex,
    E: Edge<V>,
{
    type VIT: Iterator<Item = V>;
    // type EIT: Iterator<Item = E>;

    fn nv(&self) -> usize;
    fn ne(&self) -> usize;
    fn vertices(&self) -> Self::VIT;
    fn outdegree(&self, v: V) -> V;
    fn indegree(&self, v: V) -> V;
    fn outneighbors(&self, v: V) -> &[V];
    fn has_edge(&self, e: E) -> bool;
    fn from_edge_file(fname: &Path) -> Self;
    fn bfs(&self, v: V) -> Vec<V>;
    fn dijkstra<W>(&self, v: V, weights: fn(V, V) -> W) -> Vec<W>
    where
        W: num::Float;
}
