use rustgraphs::traits::Graph;
use rustgraphs::StaticDiGraph;
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

    let now = Instant::now();
    let h: StaticDiGraph<u32> = StaticDiGraph::from_edge_file(Path::new(filename));
    println!("Load took {}ms", now.elapsed().as_micros() as f64 / 1000.0);
    println!("h = {}", h);
    let mut avg: f64 = 0.0;

    let _bfslevels = h.bfs(src);
    for _ in 0..NRUNS {
        let now = Instant::now();
        let _levels = h.bfs(src);
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

    avg = 0.0;

    println!("starting first dijkstra");
    let _dists = h.dijkstra(src, weights);
    println!("starting looped dijkstra");

    for _ in 0..NRUNS {
        let now = Instant::now();
        let _dists = h.dijkstra(src, weights);
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

