use std::collections::HashMap;
use std::hash::Hash;
use std::io::Read;
use std::str::FromStr;

use crate::etc::ErasedError;
use crate::{Solution, SolutionPair};
use lazy_static::lazy_static;
use regex::Regex;

const INPUT: &str = include_str!("../../input/day16.txt");

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node<'a> {
    name: &'a str,
    data: u32,
    neighbors: Vec<&'a str>,
}

impl<'a> Node<'a> {
    const fn new(name: &'a str, data: u32, neighbors: Vec<&'a str>) -> Self {
        Node {
            name,
            data,
            neighbors,
        }
    }
}

impl<'a> TryFrom<&'a str> for Node<'a> {
    type Error = ErasedError;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref RE_NODE: Regex =
                Regex::new(r"^Valve (?P<name>[A-Z]{2}) has flow rate=(?P<data>[0-9]+); tunnels? leads? to valves? (?P<neighbors>([A-Z]{2}(?:,\s*|$))+)").unwrap();
        }

        let caps = RE_NODE
            .captures(s)
            .ok_or_else(|| format!("ERROR: '{s}' did not match spec."))?;

        let name: &str = caps.name("name").unwrap().as_str();
        let data: u32 = caps["data"].parse()?;
        let neighbors: Vec<&str> = caps
            .name("neighbors")
            .unwrap()
            .as_str()
            .trim()
            .split(", ")
            .collect();

        Ok(Node::new(name, data, neighbors))
    }
}

struct Graph<'a> {
    nodes: HashMap<&'a str, Node<'a>>,
}

impl<'a> TryFrom<&'a str> for Graph<'a> {
    type Error = ErasedError;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let mut nodes = HashMap::new();
        for line in s.lines() {
            let node: Node = line.try_into()?;
            nodes.insert(node.name, node);
        }
        Ok(Graph { nodes })
    }
}

enum Action {
    OPEN,
    MOVE,
}

struct PathNode<'a> {
    action: Action,
    node: &'a Node<'a>,
    parent: Option<usize>,
}

struct ActionTree<'a> {
    nodes: Vec<PathNode<'a>>,
}

pub fn solve() -> SolutionPair {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day16_test.txt");

    #[test]
    fn test_node_try_from() {
        let data = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB";
        let node: Node = data.try_into().unwrap();
        assert_eq!(node.name, "AA");
        assert_eq!(node.data, 0);
        assert_eq!(node.neighbors, vec!["DD", "II", "BB"]);
    }

    #[test]
    fn test_graph_try_from() {
        let graph: Graph = TEST_INPUT.try_into().unwrap();
        assert_eq!(graph.nodes.len(), 10);
    }
}
