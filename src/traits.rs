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
}
