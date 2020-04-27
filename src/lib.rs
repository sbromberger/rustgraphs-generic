use crate::traits::Graph;
use bitvec::prelude as bv;
use std::fmt;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::mem;
use std::path::Path;
pub mod traits;

const VOOR: &str = "vertex out of range";

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
        self.start += V::one();
        if self.start >= self.end {
        self.start += V::one();
            return None;
        }
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
pub struct StaticDiGraph<V>
where
    V: traits::Vertex + graph_matrix::MxElement,
{
    fadj: graph_matrix::GraphMatrix<V>,
    badj: graph_matrix::GraphMatrix<V>,
}

impl<V> traits::Graph<V, SimpleEdge<V>> for StaticDiGraph<V>
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
            end: V::from(self.nv()).expect(VOOR),
        }
    }

    fn outdegree(&self, v: V) -> V {
        // println!("outdegree: v = {:?}", v);
        self.fadj.row_len(v.to_usize().expect(VOOR))
    }

    fn indegree(&self, v: V) -> V {
        self.badj.row_len(v.to_usize().expect(VOOR))
    }

    fn has_edge(&self, e: SimpleEdge<V>) -> bool {
        let s = e.src;
        let d = e.dst;

        let d1 = self.outdegree(s);
        let d2 = self.outdegree(d);
        if d1 < d2 {
            // dst outdegree is larger.
            self.fadj.has_index(s, d)
        } else {
            self.badj.has_index(d, s)
        }
    }

    fn outneighbors(&self, v: V) -> &[V] {
        self.fadj.row(v)
    }

    fn from_edge_file(fname: &Path) -> Self {
        let f = File::open(fname).expect("Cannot open file");
        let file = BufReader::new(&f);
        let mut edgelist: Vec<(V, V)> = vec![];
        for line in file.lines() {
            let l = line.expect("error reading file"); // produces a std::string::String
            let l = l.trim(); // changes to &str
            if l.starts_with("#") {
                continue;
            }
            let mut eit = l.split_whitespace();
            let s1 = eit.next().expect("Invalid line (first field)");
            let s2 = eit.next().expect("Invalid line (second field)");
            if eit.next().is_some() {
                panic!("Invalid line (extra fields)");
            }
            let src128: u128 = s1.parse().unwrap();
            let dst128: u128 = s2.parse().unwrap();
            let src = V::from(src128).expect("vertex out of range");
            let dst = V::from(dst128).expect("vertex out of range");
            edgelist.push((src, dst));
        }
        let bedges = edgelist.clone().iter().map(|x| (x.1, x.0)).collect();
        let fadj = graph_matrix::GraphMatrix::from_edges(edgelist);
        let badj = graph_matrix::GraphMatrix::from_edges(bedges);
        StaticDiGraph { fadj, badj }
    }
    fn bfs(&self, src: V) -> Vec<V> {
        let n = self.nv();
        let maxdeg = self
            .vertices()
            .map(|v| self.outdegree(v))
            .max()
            .expect("Invalid degree found")
            .to_usize()
            .unwrap();
        let mut visited: bv::BitVec<bv::Lsb0, u64> = bv::BitVec::repeat(false, n);

        let mut levels: Vec<V> = vec![V::max_value(); n];
        let mut cur_level: Vec<V> = Vec::new();
        cur_level.reserve(maxdeg);

        let mut next_level: Vec<V> = Vec::new();
        next_level.reserve(maxdeg);

        let s = src.to_usize().expect("Invalid vertex");
        visited.set(s, true);
        cur_level.push(src);
        levels[s] = V::zero();

        let mut n_level = V::one();

        // println!("cur_level = {:?}", cur_level);
        while !cur_level.is_empty() {
            for v in cur_level.iter() {
                for i in self.outneighbors(*v) {
                    // println!("neighbor {:?}", i);
                    let ui = i.to_usize().expect("Invalid vertex");
                    if unsafe { !*visited.get_unchecked(ui) } {
                        // println!("{:?} -> {}", v, ui);
                        next_level.push(*i);
                        unsafe {
                            visited.set_unchecked(ui, true);
                            *levels.get_unchecked_mut(ui) = n_level;
                        }
                    }
                }
            }
            n_level = n_level + V::one();
            // println!("next_level = {:?}", next_level);
            cur_level.clear();

            mem::swap(&mut cur_level, &mut next_level);
            cur_level.sort_unstable();
        }
        levels
    }
}

impl<V> fmt::Display for StaticDiGraph<V>
where
    V: SimpleVertex,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}) Graph", self.nv(), self.ne())
    }
}
