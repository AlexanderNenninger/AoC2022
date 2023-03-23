#![allow(unused)]

use std::{
    cmp::Reverse,
    ops::{Deref, DerefMut},
};

use priority_queue::PriorityQueue;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node {
    pub parent: Option<usize>,
    pub neighbors: Vec<usize>,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            parent: None,
            neighbors: Vec::new(),
        }
    }
}

impl Node {
    fn new(parent: Option<usize>) -> Node {
        Node {
            parent,
            neighbors: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Graph {
    pub start: Vec<usize>,
    pub destination: usize,
    pub nodes: Vec<Node>,
}

impl Graph {
    pub fn djikstra(&mut self) {
        // Init algorithm
        let mut pq = PriorityQueue::new();
        for node_idx in 0..self.nodes.len() {
            let priority = if self.start.contains(&node_idx) {
                Reverse(0)
            } else {
                Reverse(u64::MAX)
            };
            pq.push(node_idx, priority);
        }
        // Run algorithm
        loop {
            if let Some((u_idx, u_dist)) = pq.pop() {
                let u = self.nodes[u_idx].clone();

                for v_idx in &u.neighbors {
                    if let Some((&v_idx, v_dist)) = pq.get(v_idx) {
                        let [u, v] = self
                            .nodes
                            .get_many_mut::<2>([u_idx, v_idx])
                            .expect("ERROR: Node wit self loop.");

                        let alternative = u_dist.0.saturating_add(1);
                        if alternative < v_dist.0 {
                            pq.change_priority(&v_idx, Reverse(alternative));
                            v.parent = Some(u_idx);
                        }
                    }
                }
            } else {
                break;
            }
        }
    }

    #[allow(unused)]
    pub fn shortest_path(&self) -> Vec<usize> {
        let mut out = Vec::new();
        let mut current_node = self.destination;
        while let Some(parent) = self.nodes[current_node].parent {
            out.push(parent);
            current_node = parent;
        }
        out
    }

    pub fn shortest_path_len(&self) -> u64 {
        let mut out = 0;
        let mut current_node = self.destination;
        while let Some(parent) = self.nodes[current_node].parent {
            out += 1;
            current_node = parent;
        }
        out
    }
}

impl Deref for Graph {
    type Target = [Node];

    fn deref(&self) -> &Self::Target {
        &self.nodes
    }
}

impl DerefMut for Graph {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.nodes
    }
}
