pub trait SimpleVertex:
    crate::traits::Vertex +
    graph_matrix::MxElement +
    std::fmt::Display +
    std::hash::Hash +
    num::Unsigned +
    num::cast::AsPrimitive<usize> +
{
}



impl SimpleVertex for u8 {}
impl SimpleVertex for u16 {}
impl SimpleVertex for u32 {}
impl SimpleVertex for usize {}
impl SimpleVertex for u64 {}


pub struct SimpleEdge<V>
{
    pub src: V,
    pub dst: V,
}

impl<V> crate::traits::Edge<V> for SimpleEdge<V>
where
    V: SimpleVertex,
{
    fn src(&self) -> V {
        self.src
    }

    fn dst(&self) -> V {
        self.dst
    }
}

impl<V> crate::traits::WeightedEdge<V, u8> for SimpleEdge<V>
where
    V: SimpleVertex,
{
    fn weight(&self) -> u8 {
        1u8
    }
}

