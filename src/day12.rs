use itertools::Itertools;

use crate::utils::read_challenge_data;
use std::{
    collections::{HashMap, HashSet},
    fmt,
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum CaveType {
    Small,
    Big,
}

/// Gets the cave type based on name
fn get_cave_type(s: &str) -> CaveType {
    if s.chars().all(|c| c.is_uppercase()) {
        CaveType::Big
    } else {
        CaveType::Small
    }
}

/// Undirected graph based on adjacency list
struct UndirectedGraph<N> {
    node_list: HashMap<String, N>,
    adjacency_list: HashMap<String, HashSet<String>>,
}

impl<N> fmt::Debug for UndirectedGraph<N>
where
    N: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UndirectedGraph")
            .field("node_list", &self.node_list)
            .finish()
    }
}
impl<N> UndirectedGraph<N>
where
    N: Clone,
{
    fn new() -> Self {
        Self {
            node_list: HashMap::new(),
            adjacency_list: HashMap::new(),
        }
    }

    /// Adds node to graph.
    ///  Returns true if node was inserted or false if the node already exists
    fn add_node(&mut self, u: &str, data: N) -> bool {
        if self.node_list.contains_key(u) {
            return false;
        }
        self.node_list.insert(u.to_string(), data);
        self.adjacency_list.insert(u.to_string(), HashSet::new());
        true
    }

    /// Adds edge between node u and v. Returns an error if any of the nodes isn't found in the graph;
    fn add_edge(&mut self, u: &str, v: &str) -> Result<(), String> {
        self.node_exists(u)?;
        self.node_exists(v)?;
        let u_adj_list = self.adjacency_list.get_mut(u).unwrap();
        u_adj_list.insert(v.to_string());
        let v_adj_list = self.adjacency_list.get_mut(v).unwrap();
        v_adj_list.insert(u.to_string());
        Ok(())
    }
    /// Gets the nodes' adjacency list. Returns error if the node is not found
    fn neighbours(&self, u: &str) -> Result<HashSet<String>, String> {
        self.node_exists(u)?;
        // Checked for unwrap above
        let neighbours = self.adjacency_list.get(u).unwrap().clone();
        Ok(neighbours)
    }

    /// gets the nodes' data
    fn node_data(&self, u: &str) -> Result<N, String> {
        self.node_exists(u)?;
        let data = self.node_list.get(u).unwrap().clone();
        Ok(data)
    }

    /// gets a reference to  nodes' data
    #[allow(unused)]
    fn node_data_ref(&self, u: &str) -> Result<&N, String> {
        self.node_exists(u)?;
        let data = self.node_list.get(u).unwrap();
        Ok(data)
    }

    /// gets a mutable reference to  nodes' data
    #[allow(unused)]
    fn node_data_ref_mut(&mut self, u: &str) -> Result<&mut N, String> {
        self.node_exists(u)?;
        let data = self.node_list.get_mut(u).unwrap();
        Ok(data)
    }

    fn node_exists(&self, u: &str) -> Result<(), String> {
        if !self.node_list.contains_key(u) {
            let msg = format!("Node {} is not found in the graph.", u);
            return Err(msg);
        }
        Ok(())
    }
}

fn backtrack(
    current: &str,
    start: &str,
    end: &str,
    visited: &mut HashSet<String>,
    g: &UndirectedGraph<CaveType>,
    twice: bool,
) -> u64 {
    // If we reached the end increase the count by 1
    if current == end {
        return 1;
    }

    // Else we will get the increase the count recursively
    let mut c = 0;
    // Iterate through neighbours
    let neighbours = g.neighbours(current).unwrap(); // should panic if it doesnt exist
    for neighbour in neighbours {
        // If the neighbour wasn't visited yer
        if !visited.contains(&neighbour) {
            let neighbour_data = g.node_data(&neighbour).unwrap();
            // If the node is a small cave add it to the visited. Otherwise continue
            if neighbour_data == CaveType::Small {
                visited.insert(neighbour.clone());
            }
            c += backtrack(&neighbour, start, end, visited, g, twice);
            // Cleanup to prepare for the next iteration
            visited.remove(&neighbour);
        } else if !twice && neighbour != start {
            // If no node has been visited twice yet you can go to it but set twice to true;
            c += backtrack(&neighbour, start, end, visited, g, true);
        }
    }
    c
}

pub fn solve(run_example: bool, part1: bool) -> usize {
    let input = read_challenge_data(12, run_example);
    let mut g = UndirectedGraph::new();
    for line in input.lines() {
        let (from, to): (&str, &str) = line.split('-').tuples().next().unwrap();
        let from_type = get_cave_type(from);
        let to_type = get_cave_type(to);
        g.add_node(from, from_type);
        g.add_node(to, to_type);
        g.add_edge(from, to).unwrap(); // should panic if node is not in the list
    }
    // println!("{:?}", g);
    // println!("{:?}", g.adjacency_list);

    let start = "start";
    let end = "end";
    let mut visited: HashSet<String> = HashSet::new();
    visited.insert(start.to_string());

    let c = if part1 {
        backtrack(start, start, end, &mut visited, &g, true) // 4167
    } else {
        backtrack(start, start, end, &mut visited, &g, false) // 98441
    };
    println!("{:?}", c);
    c as usize
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_example() {
        assert_eq!(solve(true, true), 10);
        assert_eq!(solve(true, false), 36);
    }
    #[test]

    fn test_problem() {
        assert_eq!(solve(false, true), 4167);
        assert_eq!(solve(false, false), 98441);
    }
}
