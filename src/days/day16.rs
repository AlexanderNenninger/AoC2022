#![allow(unused)]

use crate::{etc::ErasedError, Solution, SolutionPair};
use bit_set::BitSet;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::VecDeque,
    fmt::{write, Display},
    io::Write,
    rc::Rc,
    task,
};

type NodeId = usize;
type FlowRate = i64;
type NodeName = Rc<str>;

#[derive(Debug)]
struct Graph {
    node_names: Vec<NodeName>,
    adjacency: Vec<i64>,
    flow_rates: Vec<i64>,
}

impl Graph {
    fn len(&self) -> usize {
        self.node_names.len()
    }

    fn to_dot<W>(&self, mut writer: W) -> Result<(), ErasedError>
    where
        W: Write,
    {
        let n = self.node_names.len();
        writeln!(writer, "graph {{")?;
        for (id, name) in self.node_names.iter().enumerate() {
            writeln!(
                writer,
                "{} [label = \"{}\\n{}\"]",
                id, *name, self.flow_rates[id]
            )?;
        }
        for i in 0..n {
            for j in 0..n {
                let dist = index_ij_owned(&self.adjacency, i, j, n);
                if i < j {
                    if dist == 1 {
                        writeln!(writer, "{i} -- {j} [style=\"bold\"];")?;
                    } else {
                        writeln!(
                            writer,
                            "{i} -- {j} [color=\"red\", constraint=false, label=\"d={dist}\", style=\"dashed\"];"
                        )?;
                    }
                }
            }
        }
        writeln!(writer, "}}")?;
        Ok(())
    }

    // Find id of node by name. WARNING: Linear Search
    fn get_id(&self, node_name: &str) -> Option<NodeId> {
        return self
            .node_names
            .iter()
            .enumerate()
            .find(|(_, name)| **name.clone() == *node_name)
            .map(|(node_id, _)| node_id);
    }

    // Find dist of node by name. WARNING: Linear Search
    fn get_dist(&self, a_name: &str, b_name: &str) -> Option<i64> {
        let a_id = self.get_id(a_name)?;
        let b_id = self.get_id(b_name)?;
        Some(*index_ij(&self.adjacency, a_id, b_id, self.len()))
    }

    // Find dist of node by name. WARNING: Linear Search
    fn get_flowrate(&self, node_name: &str) -> Option<i64> {
        let node_id = self.get_id(node_name)?;
        self.flow_rates.get(node_id).map(|&node_id| node_id)
    }
}

#[inline]
fn index_ij<T>(buf: &[T], i: usize, j: usize, n: usize) -> &T {
    &buf[i * n + j]
}

#[inline]
fn index_ij_owned<T: Copy>(buf: &[T], i: usize, j: usize, n: usize) -> T {
    buf[i * n + j]
}

#[inline]
fn index_mut_ij<T>(buf: &mut [T], i: usize, j: usize, n: usize) -> &mut T {
    &mut buf[i * n + j]
}

/// Parse input
/// (names, adjacency, flow_rates)
fn read_input(s: &str) -> Result<Graph, ErasedError> {
    lazy_static! {
        static ref RE_NODE: Regex =
            Regex::new(r"^Valve (?P<name>[A-Z]{2}) has flow rate=(?P<data>[0-9]+); tunnels? leads? to valves? (?P<neighbors>([A-Z]{2}(?:,\s*|$))+)").unwrap();
    }

    let mut node_names = Vec::new();
    let mut flow_rates = Vec::new();

    // First pass: Define all nodes
    for line in s.lines() {
        let caps = RE_NODE
            .captures(line)
            .ok_or_else(|| format!("ERROR: '{line}' did not match spec."))?;

        let node_name: NodeName = caps.name("name").unwrap().as_str().into();
        let flow_rate: FlowRate = caps["data"].parse()?;

        node_names.push(node_name);
        flow_rates.push(flow_rate);
    }
    let n: usize = node_names.len();
    // Adjacency Matrix
    let mut adjacency: Vec<i64> = vec![i64::MAX / 2; n * n];

    // distance to oneself equals 0.
    for i in 0..node_names.len() {
        *index_mut_ij(&mut adjacency, i, i, n) = 0;
    }

    // Second pass: Add neighbors
    for (node_id, line) in s.lines().enumerate() {
        let caps = RE_NODE
            .captures(line)
            .ok_or_else(|| format!("ERROR: '{line}' did not match spec."))?;

        caps.name("neighbors")
            .unwrap()
            .as_str()
            .trim()
            .split(", ")
            .map(|neighbor| {
                // Lookup node_id in node_names. This is O(N), but the graph is tiny.
                let (neighbor_id, _) = node_names
                    .iter()
                    .enumerate()
                    .find(|(_, name)| neighbor == name.as_ref())
                    .expect("ERROR: Neighbor not in node_names.");

                // Set distances to neighbors
                *index_mut_ij(&mut adjacency, node_id, neighbor_id, n) = 1;
                *index_mut_ij(&mut adjacency, neighbor_id, node_id, n) = 1;
            })
            .count();
    }
    floyd_warshall_prefilled(&mut adjacency, n);
    Ok(Graph {
        node_names,
        adjacency,
        flow_rates,
    })
}

/// Apply the Floyd-Warshall algorithm to an already prefilled adjacency matrix.
/// We assume the diagonal has been zeroed and distances to direct neighbors have been
fn floyd_warshall_prefilled(adjacency: &mut [i64], num_nodes: usize) {
    for k in 0..num_nodes {
        for i in 0..num_nodes {
            for j in 0..num_nodes {
                let d_ik = *index_ij(adjacency, i, k, num_nodes);
                let d_kj = *index_ij(adjacency, k, j, num_nodes);
                let d_ij = index_mut_ij(adjacency, i, j, num_nodes);
                *d_ij = (*d_ij).min(d_ik + d_kj);
            }
        }
    }
}

/// We can ignore nodes with 0 flow in our problem.
fn reduce_zero_flow(graph: &Graph, keep: &str) -> Graph {
    let mut node_names = vec![];
    let mut flow_rates = vec![];
    let mut idxs = vec![];
    for (idx, (node_name, flow_rate)) in graph
        .node_names
        .iter()
        .zip(graph.flow_rates.iter())
        .enumerate()
    {
        if *flow_rate > 0 || **node_name == *keep {
            node_names.push(node_name.clone());
            flow_rates.push(*flow_rate);
            idxs.push(idx);
        }
    }
    let n = node_names.len();
    let old_n = graph.node_names.len();
    let mut adjacency = vec![i64::MAX / 2; n * n];
    for i in 0..n {
        for j in 0..n {
            let (old_i, old_j) = (idxs[i], idxs[j]);
            *index_mut_ij(&mut adjacency, i, j, n) =
                *index_ij(&graph.adjacency, old_i, old_j, old_n);
        }
    }

    Graph {
        node_names,
        adjacency,
        flow_rates,
    }
}

fn start_node(graph: &Graph, node_name: &str) -> Option<NodeId> {
    Some(
        graph
            .node_names
            .iter()
            .enumerate()
            .find(|(_, name)| ***name == *node_name)?
            .0,
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ActionType {
    MoveTo,
    Open,
    Root,
}

impl Display for ActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionType::MoveTo => write!(f, "move to"),
            ActionType::Open => write!(f, "open"),
            ActionType::Root => write!(f, "start at"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Action {
    node: NodeId,
    variant: ActionType,
    cost: i64,
}

fn next_actions(graph: &Graph, node: NodeId, time_remaining: i64, opened: &BitSet) -> Vec<Action> {
    use ActionType::*;
    let n = graph.node_names.len();
    let mut actions = Vec::with_capacity(n);

    if !opened.contains(node) && time_remaining >= 1 {
        actions.push(Action {
            node,
            variant: Open,
            cost: 1,
        })
    }

    for next_node in 0..n {
        if next_node != node {
            let travel_time = index_ij_owned(&graph.adjacency, node, next_node, graph.len());

            if time_remaining >= travel_time
                && graph.flow_rates[next_node] > 0
                && !opened.contains(next_node)
            {
                actions.push(Action {
                    node: next_node,
                    variant: MoveTo,
                    cost: travel_time,
                })
            }
        }
    }
    actions
}

fn total_flow_rate(graph: &Graph, opened: &BitSet) -> i64 {
    opened.iter().map(|node| graph.flow_rates[node]).sum()
}

/// The maximal relievable pressure in each state is calculated as
///
/// current_flow_rate * time_remaining + sum_i flow_rate(unopened[i])*(time_remaining - 2i + 1)
///
fn max_relievable_pressure(graph: &Graph, opened: &BitSet, time_remaining: i64) -> i64 {
    let mut relievable_pressure = total_flow_rate(graph, opened) * time_remaining;

    let mut to_open = BitSet::from_iter(0..graph.len());
    to_open.difference_with(opened);

    let mut time_remaining = time_remaining;

    let flow_rates = to_open
        .iter()
        .map(|node_id| graph.flow_rates[node_id])
        .sorted()
        .rev();

    for flow_rate in flow_rates {
        time_remaining -= 1;
        relievable_pressure += flow_rate * time_remaining;
        time_remaining -= 1;
    }
    relievable_pressure
}

fn pressure_bfs(
    graph: &Graph,
    time_remaining: i64,
    start_node: NodeId,
) -> (i64, Vec<(Action, i64, BitSet)>) {
    let mut max_pressure_relieved = 0;

    // Setup Visited Bitset
    let opened: BitSet = BitSet::new();
    let mut best_path = vec![];

    // Setup BFS queue
    let action = Action {
        node: start_node,
        variant: ActionType::Root,
        cost: 0,
    };

    // action, time remaining, pressure relieved, nodes already opened
    let start = (action, time_remaining, 0, opened, vec![]);
    let mut queue = VecDeque::new();
    queue.push_front(start);

    // BFS Loop
    while let Some((action, time_remaining, pressure_reliefed, mut opened, mut path)) =
        queue.pop_back()
    {
        let tf = total_flow_rate(graph, &opened);

        path.push((action.clone(), tf, opened.clone()));

        let pressure_reliefed = pressure_reliefed + tf * action.cost;

        if matches!(action.variant, ActionType::Open) {
            opened.insert(action.node);
        };
        let time_remaining = time_remaining - action.cost;

        if pressure_reliefed > max_pressure_relieved {
            max_pressure_relieved = pressure_reliefed;
            best_path = path.clone();
        }

        // If we can never relieve more pressure than the best we already found, we
        // prune the branch.
        if pressure_reliefed + max_relievable_pressure(graph, &opened, time_remaining)
            < max_pressure_relieved
        {
            continue;
        }

        for action in next_actions(graph, action.node, time_remaining, &opened) {
            let next_args = (
                action,
                time_remaining,
                pressure_reliefed,
                opened.clone(),
                path.clone(),
            );
            queue.push_front(next_args);
        }
    }
    (max_pressure_relieved, best_path)
}

pub fn solve() -> SolutionPair {
    const INPUT: &str = include_str!("../../input/day16.txt");
    let graph = read_input(INPUT).expect("ERROR: Invalid input day 16.");
    let start_node = start_node(&graph, "AA").expect("ERROR: Start node 'AA' not in graph.");
    let sol1: u64 = pressure_bfs(&graph, 20, start_node).0 as u64;
    let sol2: u64 = 0;

    (Solution::U64(sol1), Solution::U64(sol2))
}

mod tests {

    #[allow(unused)]
    use super::*;

    #[allow(unused)]
    const TEST_INPUT: &str = include_str!("../../input/day16_test.txt");

    #[test]
    fn test_read_input() -> Result<(), ErasedError> {
        let g = read_input(TEST_INPUT)?;
        let f = std::io::BufWriter::new(std::fs::File::create("day16.dot")?);
        g.to_dot(f)?;
        Ok(())
    }

    #[test]
    fn test_floyd_warshall() -> Result<(), ErasedError> {
        let Graph {
            node_names,
            mut adjacency,
            flow_rates: _,
        } = read_input(TEST_INPUT)?;
        let num_nodes = node_names.len();
        floyd_warshall_prefilled(&mut adjacency, num_nodes);
        Ok(())
    }

    #[test]
    fn test_reduce_zero_flow() -> Result<(), ErasedError> {
        // Check whether graph reduction works.
        let graph = read_input(TEST_INPUT)?;
        let graph_reduced = reduce_zero_flow(&graph, "AA");

        for a_name in graph_reduced.node_names.iter() {
            for b_name in graph_reduced.node_names.iter() {
                assert_eq!(
                    graph.get_dist(&a_name, &b_name),
                    graph_reduced.get_dist(&a_name, &b_name)
                )
            }
        }

        for node_name in graph_reduced.node_names.iter() {
            assert_eq!(
                graph_reduced.get_flowrate(node_name),
                graph.get_flowrate(node_name)
            )
        }

        assert_eq!(
            graph_reduced
                .flow_rates
                .iter()
                .filter(|&flow_rate| *flow_rate > 0)
                .count(),
            graph_reduced.len() - 1
        );

        assert_eq!(
            graph
                .flow_rates
                .iter()
                .filter(|&flow_rate| *flow_rate > 0)
                .count(),
            graph_reduced.len() - 1
        );

        let f = std::io::BufWriter::new(std::fs::File::create("day16_reduced.dot")?);
        graph_reduced.to_dot(f)?;
        Ok(())
    }

    #[test]
    fn test_max_reliveable_pressure() -> Result<(), ErasedError> {
        let graph = read_input(TEST_INPUT)?;
        let graph_reduced = reduce_zero_flow(&graph, "AA");
        let opened = BitSet::new();
        let p = max_relievable_pressure(&graph_reduced, &opened, 30);
        assert_eq!(p, 2105);

        let mut opened = BitSet::new();
        opened.insert(graph_reduced.get_id(&"CC").ok_or("ERROR: Not node CC.")?);
        let p = max_relievable_pressure(&graph_reduced, &opened, 30);
        assert_eq!(p, 2127);
        Ok(())
    }

    #[test]
    fn test_pressure_bfs() -> Result<(), ErasedError> {
        let g = read_input(TEST_INPUT)?;
        let g = reduce_zero_flow(&g, "AA");

        let start_node_id = start_node(&g, "AA").expect("ERROR: AA not in graph.");
        let (score, path) = pressure_bfs(&g, 30, start_node_id);

        let mut time_passed = 0;
        for (minute, (action, flowrate, opened)) in path.iter().enumerate() {
            let node_name = g.node_names[action.node].clone();
            let action_type = action.variant;
            time_passed += action.cost;
            println!(
                "== Minute {time_passed} ==\n Releasing {flowrate} pressure.\n You {action_type} valve {node_name}.\n"
            );
        }

        assert_eq!(score, 1651);
        Ok(())
    }
}
