use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn reverse() {
    let mut adjacent_list: Vec<Vec<usize>> = Vec::new();
    adjacent_list.push(vec![2, 3]);
    adjacent_list.push(vec![0]);
    adjacent_list.push(vec![1]);
    adjacent_list.push(vec![4]);
    adjacent_list.push(vec![]);
    let kosa = kosaraju(&adjacent_list);
    for (stuff, thing) in kosa {
        println!("{}, {}", stuff, thing);
    }
}

fn transpose(&Vec<Vec<usize>>) {

    let result: Vec<Vec<usize>> = Vec::new();
    
}
fn kosaraju(adjacent: &Vec<Vec<usize>>) -> HashMap<usize, usize> {
    let mut visited: HashSet<usize> = HashSet::new();
    let mut l: VecDeque<usize> = VecDeque::new();
    for u in 0..adjacent.len() {
        visit(u, adjacent, &mut visited, &mut l);
    }
    //let transposed_adjacent = vec![0; adjacent.len()];
    //  let adjacent = &transpose(adjacent);
    let mut assigned = HashMap::new();

    for u in l {
        assign(u, u, adjacent, &mut assigned);
    }
    assigned
}
fn visit(
    u: usize,
    adjacent: &Vec<Vec<usize>>,
    visited: &mut HashSet<usize>,
    l: &mut VecDeque<usize>,
) {
    if !visited.contains(&u) {
        visited.insert(u);
        for &v in &adjacent[u] {
            visit(v, adjacent, visited, l);
        }
        l.push_front(u);
    }
}

fn assign(u: usize, root: usize, adjacent: &Vec<Vec<usize>>, assigned: &mut HashMap<usize, usize>) {
    if !assigned.contains_key(&u) {
        assigned.insert(u, root);
        for &v in &adjacent[u] {
            assign(v, root, adjacent, assigned);
        }
    }
}
