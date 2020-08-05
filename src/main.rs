use rustgraphs::traits::Graph;
use rustgraphs::{StaticDiGraph, StaticGraph, triangles, threaded_triangles, threaded_triangles_csr, bfs, dijkstra};
use std::env;
use std::path::Path;
use std::time::Instant;
pub const NRUNS: usize = 50;

fn weights(_: u32, _: u32) -> f32 { 1f32 }
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let src = &args[2];
    let src: u32 = src.parse().expect("invalid source");

    let op = &args[3];

    let mut avg: f64 = 0.0;

    if op == "bfs" {
        let now = Instant::now();
        let h: StaticDiGraph<u32> = StaticDiGraph::from_edge_file(Path::new(filename));
        println!("Load took {}ms", now.elapsed().as_micros() as f64 / 1000.0);
        println!("h = {}", h);

        let newb = StaticDiGraph::from_edge_file(Path::new(filename));
        let _bfslevels = bfs(h, src);
        for _ in 0..NRUNS {
            let now = Instant::now();
            let _levels = bfs(h, src);
            let elp = now.elapsed().as_micros() as f64 / 1000.0;
            avg += elp;
            // println!("BFS took {}ms", elp);
            // println!(
            //     "max level = {}",
            //     levels
            //         .into_iter()
            //         .filter(|&x| { x < std::u32::MAX })
            //         .max()
            //         .unwrap()
            // );
            print!(".");
        }
        println!();
        println!(
            "bfs unstable sort: average over {} runs: {:.3}ms",
            NRUNS,
            avg / NRUNS as f64
        );
    }
    if op == "dijkstra" {
        let now = Instant::now();
        let h: StaticDiGraph<u32> = StaticDiGraph::from_edge_file(Path::new(filename));
        println!("Load took {}ms", now.elapsed().as_micros() as f64 / 1000.0);
        println!("h = {}", h);

        println!("starting first dijkstra");
        let _dists = dijkstra(h, src, weights);
        println!("starting looped dijkstra");

        for _ in 0..NRUNS {
            let now = Instant::now();
            let _dists = dijkstra(h, src, weights);
            let elp = now.elapsed().as_micros() as f64 / 1000.0;
            avg += elp;
            print!(".");
        }
        println!();
        println!(
            "dijkstra: average over {} runs: {:.3}ms",
            NRUNS,
            avg / NRUNS as f64
        );
    }
    if op == "triangle" {
        let now = Instant::now();
        let h: StaticGraph<u32> = StaticGraph::from_edge_file(Path::new(filename));
        println!("Load took {}ms", now.elapsed().as_micros() as f64 / 1000.0);
        println!("h = {}", h);
        println!("starting first triangle count");
        let ntri = triangles(&h);
        println!("{} triangles", ntri);
        println!("starting looped triangle count");
        for _ in 0..NRUNS {
            let now = Instant::now();
            let _ntri = triangles(&h);
            let elp = now.elapsed().as_micros() as f64 / 1000.0;
            avg += elp;
            print!(".");
        }
        println!();
        println!(
            "triangle count: average over {} runs: {:.3}ms",
            NRUNS,
            avg / NRUNS as f64
        );
    }

    if op == "threaded_triangles" {
        let now = Instant::now();
        let h: StaticGraph<u32> = StaticGraph::from_edge_file(Path::new(filename));
        println!("Load took {}ms", now.elapsed().as_micros() as f64 / 1000.0);
        println!("h = {}", h);
        println!("starting first threaded_triangle count");
        let ntri = threaded_triangles(&h);
        println!("{} triangles", ntri);
        println!("starting looped threaded_triangle count");
        for _ in 0..NRUNS {
            let now = Instant::now();
            let _ntri = threaded_triangles(&h);
            let elp = now.elapsed().as_micros() as f64 / 1000.0;
            avg += elp;
            print!(".");
        }
        println!();
        println!(
            "itertriangle count: average over {} runs: {:.3}ms",
            NRUNS,
            avg / NRUNS as f64
        );
    }
    if op == "threaded_triangles_csr" {
        let now = Instant::now();
        let h: StaticGraph<u32> = StaticGraph::from_edge_file(Path::new(filename));
        println!("Load took {}ms", now.elapsed().as_micros() as f64 / 1000.0);
        println!("h = {}", h);
        println!("starting first threaded_triangles_csr count");
        let ntri = threaded_triangles_csr(&h);
        println!("{} triangles", ntri);
        println!("starting looped threaded_triangles_csr count");
        for _ in 0..NRUNS {
            let now = Instant::now();
            let _ntri = threaded_triangles_csr(&h);
            let elp = now.elapsed().as_micros() as f64 / 1000.0;
            avg += elp;
            print!(".");
        }
        println!();
        println!(
            "threaded_triangles_csr count: average over {} runs: {:.3}ms",
            NRUNS,
            avg / NRUNS as f64
        );
    }
}

