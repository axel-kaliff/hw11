use std::io::{self, BufRead};
use std::usize;
type Subset = (usize, usize);

type WeightedEdge = (isize, usize, usize);
fn find(subsets: &mut Vec<Subset>, i: usize) -> usize {
    if subsets[i].1 != i {
        subsets[i].1 = find(subsets, subsets[i].1);
    }

    return subsets[i].1;
}

fn union(subsets: &mut Vec<Subset>, x: usize, y: usize) {
    let xroot = find(subsets, x);
    let yroot = find(subsets, y);

    if subsets[xroot].0 < subsets[yroot].0 {
        subsets[xroot].1 = yroot;
    } else if subsets[xroot].0 > subsets[yroot].0 {
        subsets[yroot].1 = xroot;
    } else {
        subsets[yroot].1 = xroot;
        subsets[xroot].0 += 1;
    }
}

fn kruskal(v: usize, edges: usize, sorted_edges: &Vec<WeightedEdge>) {
    let mut e = 0;
    let mut i = 0;
    let mut sum = 0;

    let mut result: Vec<WeightedEdge> = Vec::new();
    let mut subsets: Vec<Subset> = Vec::new();

    for _ in 0..v {
        subsets.push((0, 0));
    }
    for _ in 0..v {
        result.push((0, 0, 0));
    }

    for m in 0..v {
        subsets[m].1 = m;
        subsets[m].0 = 0;
    }

    while e < v && i < edges {
        let curr_edge = sorted_edges[i];
        i += 1;

        let x = find(&mut subsets, curr_edge.1);
        let y = find(&mut subsets, curr_edge.2);

        if x != y {
            result[e] = (curr_edge.0, curr_edge.1, curr_edge.2);
            e += 1;
            union(&mut subsets, x, y);
        }
    }
    for edge in 0..result.len() {
        sum += result[edge].0;
    }
    let mut sorted_result: Vec<(usize, usize)> = Vec::with_capacity(v - 1);

    for edge in &result {
        let s_edge = (edge.1, edge.2);
        if s_edge != (0, 0) {
            let index = sorted_result.binary_search(&s_edge).unwrap_or_else(|x| x);
            sorted_result.insert(index, s_edge);
        }
    }

    if sorted_result.len() == v - 1 {
        println!("{}", sum);
        for edge in 0..sorted_result.len() {
            if sorted_result[edge].0 < sorted_result[edge].1 {
                println!("{} {}", sorted_result[edge].0, sorted_result[edge].1);
            } else {
                println!("{} {}", sorted_result[edge].1, sorted_result[edge].0);
            }
        }
    } else {
        println!("Impossible");
    }
}
pub fn gogo() {
    let stdin = io::stdin();
    let mut number_nodes = 0;
    let mut number_edges = 0;
    let mut sorted_edges: Vec<WeightedEdge> = Vec::new();

    for (count, line) in stdin.lock().lines().map(|l| l.unwrap()).enumerate() {
        let nums: Vec<isize> = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        if nums.len() == 2 {
            if nums[0] == 0 && nums[1] == 0 {
                if number_edges == 0 {
                    println!("Impossible");
                } else {
                    kruskal(number_nodes, number_edges, &sorted_edges);
                }
                return;
            }
        }

        if count == 0 {
            number_edges = nums[1] as usize;
            number_nodes = nums[0] as usize;
        }

        if nums.len() == 2 {
            if count != 0 {
                if number_edges == 0 {
                    println!("Impossible");
                } else {
                    kruskal(number_nodes, number_edges, &sorted_edges);
                }
            }
            sorted_edges.clear();
            number_edges = nums[1] as usize;
            number_nodes = nums[0] as usize;
        } else if nums.len() == 3 {
            let edge: WeightedEdge = (nums[2], nums[0] as usize, nums[1] as usize);
            let index = sorted_edges.binary_search(&edge).unwrap_or_else(|x| x);
            sorted_edges.insert(index, edge);
        }
    }
}
