use num::cast::NumCast;
use crate::traits::Graph;
use crate::simplegraphs::{SimpleVertex, SimpleEdge};
use std::fmt;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
pub mod traits;
pub mod traversals;
pub mod simplegraphs;

const VOOR: &str = "vertex out of range";

pub struct StaticDiGraph<V>
{
    fadj: graph_matrix::GraphMatrix<V>,
    badj: graph_matrix::GraphMatrix<V>,
}

impl<V> traits::Graph<V, SimpleEdge<V>> for StaticDiGraph<V>
// we need V to be a SimpleVertex, and we also need Range<V> to return an iterator over V.
where
    V: SimpleVertex,
    std::ops::Range<V>: Iterator<Item=V>
{
    type VIT = std::ops::Range<V>;
    fn nv(&self) -> V::T {
        V::T::from(self.fadj.dim()).unwrap()
    }

    fn ne(&self) -> usize {
        self.fadj.n()
    }

    fn vertices(&self) -> std::ops::Range<V> {
        std::ops::Range {
            start: V::zero(),
            end: V::from(self.nv()).expect(VOOR),
        }
    }

    fn out_degree(&self, v: V) -> V {
        // println!("out_degree: v = {:?}", v);
        self.fadj.row_len(v.to_usize().expect(VOOR))
    }

    fn in_degree(&self, v: V) -> V {
        self.badj.row_len(v.to_usize().expect(VOOR))
    }

    fn has_edge(&self, e: SimpleEdge<V>) -> bool {
        let s = e.src;
        let d = e.dst;

        let d1 = self.out_degree(s);
        let d2 = self.out_degree(d);
        if d1 < d2 {
            // dst out_degree is larger.
            self.fadj.has_index(s, d)
        } else {
            self.badj.has_index(d, s)
        }
    }

    fn in_neighbors(&self, v: V) -> &[V] {
        self.badj.row(v)
    }
    fn out_neighbors(&self, v: V) -> &[V] {
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
            let src = V::from(src128).expect(VOOR);
            let dst = V::from(dst128).expect(VOOR);
            edgelist.push((src, dst));
        }
        let bedges = edgelist.clone().iter().map(|x| (x.1, x.0)).collect();
        let fadj = graph_matrix::GraphMatrix::from_edges(edgelist);
        let badj = graph_matrix::GraphMatrix::from_edges(bedges);
        StaticDiGraph { fadj, badj }
    }
}

impl<V> fmt::Display for StaticDiGraph<V>
where
    V: SimpleVertex,
    std::ops::Range<V>: Iterator<Item=V>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}) Graph", self.nv(), self.ne())
    }
}

pub struct StaticGraph<V>
{
    adj: graph_matrix::GraphMatrix<V>,
}

impl<V> traits::Graph<V, SimpleEdge<V>> for StaticGraph<V>
where
    V: SimpleVertex,
    std::ops::Range<V>: Iterator<Item=V>
{
    type VIT = std::ops::Range<V>;
    fn nv(&self) -> usize {
        usize::from(self.adj.dim())
    }

    fn ne(&self) -> usize {
        self.adj.n() / 2
    }

    fn vertices(&self) -> std::ops::Range<V> {
        std::ops::Range {
            start: V::zero(),
            end: V::from(self.nv()).expect(VOOR),
        }
    }

    fn out_degree(&self, v: V) -> V {
        // println!("out_degree: v = {:?}", v);
        self.adj.row_len(v.to_usize().expect(VOOR))
    }

    fn in_degree(&self, v: V) -> V { self.out_degree(v) }

    fn has_edge(&self, e: SimpleEdge<V>) -> bool {
        let s = e.src;
        let d = e.dst;

        let d1 = self.out_degree(s);
        let d2 = self.out_degree(d);
        if d1 < d2 {
            // dst out_degree is larger.
            self.adj.has_index(s, d)
        } else {
            self.adj.has_index(d, s)
        }
    }

    fn out_neighbors(&self, v: V) -> &[V] {
        self.adj.row(v)
    }

    fn in_neighbors(&self, v: V) -> &[V] {
        self.out_neighbors(v)
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
            let src = V::from(src128).expect(VOOR);
            let dst = V::from(dst128).expect(VOOR);
            edgelist.push((src, dst));
            edgelist.push((dst, src));
        }
        let adj = graph_matrix::GraphMatrix::from_edges(edgelist);
        StaticGraph { adj }
    }
}

impl<V> fmt::Display for StaticGraph<V>
where
    V: SimpleVertex,
    std::ops::Range<V>: Iterator<Item=V>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}) Graph", self.nv(), self.ne())
    }
}

