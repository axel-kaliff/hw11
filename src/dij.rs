use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};
use std::usize;

struct Grid<T> {
    nodes: Vec<Node<T>>,
}

struct Node<T> {
    data: T,
    edges: Vec<(usize, usize)>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    node: usize,
    cost: usize,
}

// Manually implement Ord so we get a min-heap instead of a max-heap
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type WeightedEdge = (usize, usize, usize);

impl<T> Grid<T> {
    fn new() -> Self {
        Grid { nodes: Vec::new() }
    }

    fn add_node(&mut self, data: T) -> usize {
        let node = Node {
            edges: Vec::new(),
            data: data,
        };
        self.nodes.push(node);
        self.nodes.len() - 1
    }

    fn create_edges<'a, I>(&mut self, iterator: I)
    where
        I: IntoIterator<Item = &'a WeightedEdge>,
    {
        for &(start, end, weight) in iterator.into_iter() {
            self.nodes[start].edges.push((end, weight));
            //            self.nodes[end].edges.push((start, weight));
        }
    }

    fn find_path(&self, start: usize, end: usize) -> Option<(Vec<usize>, usize)> {
        let mut dist = vec![(usize::MAX, None); self.nodes.len()];

        let mut heap = BinaryHeap::new();
        dist[start] = (0, None);
        heap.push(State {
            node: start,
            cost: 0,
        });

        while let Some(State { node, cost }) = heap.pop() {
            if node == end {
                let mut path = Vec::with_capacity(dist.len() / 2);
                let mut current_dist = dist[end];
                path.push(end);
                while let Some(prev) = current_dist.1 {
                    path.push(prev);
                    current_dist = dist[prev];
                }
                path.reverse();
                return Some((path, cost));
            }

            if cost > dist[node].0 {
                continue;
            }
            for edge in &self.nodes[node].edges {
                let next = State {
                    node: edge.0,
                    cost: cost + edge.1,
                };
                if next.cost < dist[next.node].0 {
                    dist[next.node] = (next.cost, Some(node));
                    heap.push(next);
                }
            }
        }
        None
    }
}

pub fn dij() {
    let stdin = io::stdin();
    let mut start = 0;
    let mut grid = Grid::new();

    for (count, line) in stdin.lock().lines().map(|l| l.unwrap()).enumerate() {
        let nums: Vec<usize> = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        if nums.len() == 4 {
            if nums[0] == 0 && nums[1] == 0 && nums[2] == 0 && nums[3] == 0 {
                return;
            }
        }

        if nums.len() == 4 {
            println!("");
            grid = Grid::new();
            for i in 0..nums[0] + 1 {
                grid.add_node(i);
            }
            start = nums[3];
        } else if nums.len() == 3 {
            grid.create_edges(&[(nums[0], nums[1], nums[2])]);
        } else if nums.len() == 1 {
            if grid.find_path(start, nums[0]).is_some() {
                let (path, cost) = grid.find_path(start, nums[0]).unwrap();
                println!("{}", cost);
            } else {
                println!("Impossible");
            }
        }
    }

    //print!("{}", grid.nodes[path[0]].data);
    //for i in path.iter().skip(1) {
    //   print!(" -> {}", grid.nodes[*i].data);
    //}
    //println!("\nCost: {}", cost);
}
