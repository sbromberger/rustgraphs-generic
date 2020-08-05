use std::ops::Range;
use rayon::prelude::*;

use crate::{SimpleVertex, StaticGraph};

pub fn triangles<V>(g: &StaticGraph<V>) -> usize where V: SimpleVertex {
    let mut dodg: Vec<Vec<V>> = Vec::with_capacity(g.nv());

    let mut degrees = vec![V::zero(); g.nv()];
    let mut ntri = 0usize;

    // let z:Vec<V> = g.vertices().collect();
    // println!("minvert = {:?}, maxvert = {:?}", z.iter().min(), z.iter().max());
    for u in g.vertices() {
        let degu = g.out_degree(u);
        degrees[u.as_()] = degu;
        let vvec = g.out_neighbors(u).into_iter().filter(|v| {
            let degv = g.out_degree(**v);
            degv > degu || (degv == degu && **v > u)
        }).cloned().collect();
        dodg.push(vvec);
    }

    // println!("len(dodg) = {}", dodg.len());
    for u in g.vertices() {
        let uvec = &dodg[u.as_()];
        let ulen = uvec.len();
        for i in 0..ulen {
            let v = uvec[i];
            let vvec = &dodg[v.as_()];
            for j in (i+1)..ulen {
                let w = uvec[j];
                let wvec = &dodg[w.as_()];
                let w_to_v = degrees[v.as_()] > degrees[w.as_()] || (degrees[v.as_()] == degrees[w.as_()] && v > w);
                if (w_to_v && wvec.binary_search(&v).is_ok()) || (!w_to_v && vvec.binary_search(&w).is_ok()) {
                     ntri += 1;
                }
            }
        }
    }
    ntri
}

fn optimal_contiguous_partition(weights: Vec<usize>, n_partitions: usize) -> Vec<Range<usize>>
{

    let mut up_bound = weights.iter().sum();
    let mut low_bound = (up_bound-1) / n_partitions;
    let n_items = weights.len();

    while up_bound > low_bound + 1 {
        let search_for = (up_bound + low_bound) / 2;
        let mut sum_part = 0usize;
        let mut remain_part = n_partitions;
        let mut possible = true;

        for w in weights.iter() {
            sum_part += w;
            if sum_part > search_for {
                sum_part = *w;
                remain_part -= 1;
                if remain_part == 0 {
                    possible = false;
                    break;
                }
            }
        }
        if possible {
            up_bound = search_for;
        } else {
            low_bound = search_for;
        }
    }
    let best_balance = up_bound;
    let mut partitions: Vec<Range<usize>> = Vec::with_capacity(n_partitions);
    let mut sum_part = 0;
    let mut left = 0;
    weights.iter().enumerate().for_each(|(i, w)| {
        sum_part += w;
        if sum_part > best_balance {
            partitions.push(left..i);
            sum_part = *w;
            left = i;
        }
    });
    partitions.push(left..n_items);

    partitions
}



pub fn threaded_triangles<V>(g: &StaticGraph<V>) -> usize where V: SimpleVertex + std::marker::Sync + std::marker::Send {
    // let z:Vec<V> = g.vertices().collect();
    // println!("minvert = {:?}, maxvert = {:?}", z.iter().min(), z.iter().max());
    let mut bigvec: Vec<(V, Vec<V>)> = Vec::with_capacity(g.nv());
    (0..g.nv()).into_par_iter().map(|u| {
        let uu = V::from(u).unwrap();
        let degu = g.out_degree(uu);
        let vvec: Vec<V> = g.out_neighbors(uu).into_iter().filter(|v| {
            let degv = g.out_degree(**v);
            degv > degu || (degv == degu && **v > uu)
        }).cloned().collect();
        // if u < 5 {
        //     println!("out_degree = {}, len(vvec) = {}", degu, vvec.len());
        // }
        (degu, vvec)
    }).collect_into_vec(&mut bigvec);

    // let s: Vec<usize> = bigvec.iter().map(|(_, v)| v.len()).collect();
    // let ss: usize = s.into_iter().sum();
    // println!("total sum from bigvec = {}", ss);

    let (degrees, dodg) = {
        let mut degs: Vec<V> = Vec::with_capacity(bigvec.len());
        let mut indptr: Vec<usize> = Vec::with_capacity(bigvec.len() + 1);
        let mut indices: Vec<V> = Vec::new();
        // let mut acc = 0usize;
        indptr.push(0);
        bigvec.into_iter().for_each(|mut v| {
            degs.push(v.0);
            indices.append(&mut v.1);
            indptr.push(indices.len());
        });
    // let isum: usize = indptr.iter().sum();
    // println!("isum = {}", isum);
    // println!("indices = {:?}", &indices[0..10]);
    let gm = graph_matrix::GraphMatrix::new(indptr, indices);
    // println!("gm = {}", gm);
    (degs, gm)
    };

    // let (degrees, dodg): (Vec<V>, Vec<Vec<V>>) = bigvec.unzip();
    // let foo: usize = (0..dodg.dim()).map(|r| dodg.row_len(r).as_()).sum();
    // println!("foo = {}", foo);
    // println!("dodg = {}", dodg);
    let weights: Vec<usize> = (0..dodg.dim()).map(|r| dodg.row_len(r).pow(2).as_()).collect();
    // println!("{:?}", weights);
    let partitions = optimal_contiguous_partition(weights, 12);
    // println!("{:?}", partitions);
    partitions.into_par_iter().map(|p| {
        let mut ntri = 0usize;
        for u in p {
            let uvec = dodg.row(V::from(u).unwrap());
            let ulen = uvec.len();
            for i in 0..ulen {
                let v = uvec[i];
                let vvec = dodg.row(v);
                for j in (i+1)..ulen {
                    // ntri += 1;
                    let w = uvec[j];
                    let wvec = dodg.row(w);
                    let w_to_v = degrees[v.as_()] > degrees[w.as_()] || (degrees[v.as_()] == degrees[w.as_()] && v > w);
                    if (w_to_v && wvec.binary_search(&v).is_ok()) || (!w_to_v && vvec.binary_search(&w).is_ok()) {
                         ntri += 1;
                    }
                }
            }
        }
       ntri
    }).sum()
}

pub fn threaded_triangles_csr<V>(g: &StaticGraph<V>) -> usize where V: SimpleVertex + std::marker::Sync + std::marker::Send {
    // let z:Vec<V> = g.vertices().collect();
    // println!("minvert = {:?}, maxvert = {:?}", z.iter().min(), z.iter().max());
    let bigvec = (0..g.nv()).into_par_iter().map(|u| {
        let uu = V::from(u).unwrap();
        let degu = g.out_degree(uu);
        let vvec: Vec<V> = g.out_neighbors(uu).into_iter().filter(|v| {
            let degv = g.out_degree(**v);
            degv > degu || (degv == degu && **v > uu)
        }).cloned().collect();
        // if u < 5 {
        //     println!("out_degree = {}, len(vvec) = {}", degu, vvec.len());
        // }
        (degu, vvec)
    });

    // let s: Vec<usize> = bigvec.iter().map(|(_, v)| v.len()).collect();
    // let ss: usize = s.into_iter().sum();
    // println!("total sum from bigvec = {}", ss);

    let (degrees, dodg): (Vec<V>, Vec<Vec<V>>) = bigvec.unzip();

    // let (degrees, dodg): (Vec<V>, Vec<Vec<V>>) = bigvec.unzip();
    // let foo: usize = (0..dodg.dim()).map(|r| dodg.row_len(r).as_()).sum();
    // println!("foo = {}", foo);
    // println!("dodg = {}", dodg);
    let weights: Vec<usize> = (0..dodg.len()).map(|r| dodg[r].len()).collect();
    // println!("{:?}", weights);
    let partitions = optimal_contiguous_partition(weights, 12);
    // println!("{:?}", partitions);
    partitions.into_par_iter().map(|p| {
        let mut ntri = 0usize;
        for u in p {
            let uvec = &dodg[u];
            let ulen = uvec.len();
            for i in 0..ulen {
                let v = uvec[i];
                let vvec = &dodg[v.as_()];
                for j in (i+1)..ulen {
                    // ntri += 1;
                    let w = uvec[j];
                    let wvec = &dodg[w.as_()];
                    let w_to_v = degrees[v.as_()] > degrees[w.as_()] || (degrees[v.as_()] == degrees[w.as_()] && v > w);
                    if (w_to_v && wvec.binary_search(&v).is_ok()) || (!w_to_v && vvec.binary_search(&w).is_ok()) {
                         ntri += 1;
                    }
                }
            }
        }
       ntri
    }).sum()
}
