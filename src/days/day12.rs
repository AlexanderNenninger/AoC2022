use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use crate::{
    etc::{ErasedError, Graph, Matrix, Node},
    Solution, SolutionPair,
};

///////////////////////////////////////////////////////////////////////////////
const INPUT: &str = include_str!("../../input/day12.txt");

/////////////
#[derive(Debug, Clone)]
struct PathProblem<const M: usize, const N: usize>
where
    [u8; M * N]:,
{
    map: Matrix<M, N, u8>,
    start_index: [usize; 2],
    end_index: [usize; 2],
}

impl<const M: usize, const N: usize> PathProblem<M, N>
where
    [u8; M * N]:,
{
    fn neighbors(&self, idx: [usize; 2]) -> impl Iterator<Item = [usize; 2]> {
        let [i, j] = idx;
        let mut row_indices = [None; 4];
        let mut col_indices = [None; 4];
        if i > 0 && i < M {
            row_indices[0] = Some(i - 1);
            col_indices[0] = Some(j);
        }
        if j > 0 && j < N {
            row_indices[1] = Some(i);
            col_indices[1] = Some(j - 1);
        }

        if j < N - 1 {
            row_indices[2] = Some(i);
            col_indices[2] = Some(j + 1);
        }

        if i < M - 1 {
            row_indices[3] = Some(i + 1);
            col_indices[3] = Some(j)
        }
        row_indices
            .into_iter()
            .zip(col_indices.into_iter())
            .filter_map(|(idx, idy)| Some([idx?, idy?]))
    }

    fn moves(&self, idx: [usize; 2]) -> impl Iterator<Item = [usize; 2]> + '_ {
        self.neighbors(idx)
            .filter(move |&nidx| self.map[nidx] <= self.map[idx] + 1)
    }

    fn to_graph(&self) -> Graph {
        let mut nodes = Vec::with_capacity(M * N);
        // Generate all Nodes
        for _ in 0..M * N {
            nodes.push(Node::default())
        }

        // Add Edges to Graph
        for idx in self.map.each_index() {
            let node = &mut nodes[N * idx[0] + idx[1]];
            for nidx in self.moves(idx) {
                node.neighbors.push(N * nidx[0] + nidx[1])
            }
        }

        Graph {
            start: vec![N * self.start_index[0] + self.start_index[1]],
            destination: N * self.end_index[0] + self.end_index[1],
            nodes: nodes,
        }
    }

    fn to_graph_target_level(&self, target_level: u8) -> Graph {
        let mut nodes = Vec::with_capacity(M * N);
        let mut starts = Vec::with_capacity(M * N);

        for _ in 0..M * N {
            nodes.push(Node::default())
        }

        // Add Edges to Graph
        for idx in self.map.each_index() {
            let node = &mut nodes[N * idx[0] + idx[1]];
            for nidx in self.moves(idx) {
                node.neighbors.push(N * nidx[0] + nidx[1])
            }
            if self.map[idx] == target_level {
                starts.push(N * idx[0] + idx[1]);
            }
        }

        Graph {
            start: starts,
            destination: N * self.end_index[0] + self.end_index[1],
            nodes: nodes,
        }
    }
}

impl<const M: usize, const N: usize> FromStr for PathProblem<M, N>
where
    [u8; M * N]:,
{
    type Err = ErasedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const START_CHAR: u8 = 'S' as u8;
        const END_CHAR: u8 = 'E' as u8;

        const MSG: &str = "ERROR: Parsing PathProblem failed.";
        let s = s.trim();
        if !s.trim().len() == M * N + N - 1 {
            return Err(MSG.into());
        }
        let mut map = Matrix::new(0);

        let mut start_index = None;
        let mut end_index = None;
        for (idx, mut height) in s
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c as u8)
            .enumerate()
        {
            if height == START_CHAR {
                height = 'a' as u8;
                start_index = Some([idx / N, idx % N])
            } else if height == END_CHAR {
                height = 'z' as u8;
                end_index = Some([idx / N, idx % N])
            }
            map[idx] = height;
        }
        if let Some(start_index) = start_index {
            if let Some(end_index) = end_index {
                return Ok(PathProblem {
                    map: map,
                    start_index,
                    end_index,
                });
            }
        }
        Err(MSG.into())
    }
}

impl<const M: usize, const N: usize> Display for PathProblem<M, N>
where
    [(); M * N]:,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, h) in self.map.iter().enumerate() {
            let h = *h as char;
            if i % N == 0 {
                writeln!(f)?
            }
            if i / N == self.start_index[0] && i % N == self.start_index[1] {
                write!(f, "\x1b[36;1m {h:3}\x1b[0m")?
            } else if i / N == self.end_index[0] && i % N == self.end_index[1] {
                write!(f, "\x1b[35;1m {h:3}\x1b[0m")?
            } else {
                write!(f, " {h:3}")?
            }
        }
        writeln!(f)
    }
}

pub fn solve() -> SolutionPair {
    let pp: PathProblem<41, 162> = INPUT.parse().unwrap();

    let mut g1 = pp.to_graph();
    g1.djikstra();
    let sol1: u64 = g1.shortest_path_len();

    let mut g2 = pp.to_graph_target_level('a' as u8);
    g2.djikstra();
    let sol2: u64 = g2.shortest_path_len();

    (Solution::U64(sol1), Solution::U64(sol2))
}

#[cfg(test)]
mod tests {

    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day12_test.txt");

    #[test]
    fn from_str() {
        let pp: PathProblem<5, 8> = TEST_INPUT.parse().unwrap();
        println!("{pp}");

        let pp: PathProblem<41, 162> = INPUT.parse().unwrap();
        println!("{:?}", &pp.start_index);
        println!("{:?}", &pp.end_index);
        println!("{pp}");
    }

    #[test]
    fn neighbors() {
        const M: usize = 5;
        const N: usize = 8;
        let pp: PathProblem<M, N> = TEST_INPUT.parse().unwrap();

        let n_ul: Vec<[usize; 2]> = pp.neighbors([0, 0]).collect();
        assert_eq!(n_ul, vec![[0, 1], [1, 0]]);

        let n_lr_out_of_bounds: Vec<[usize; 2]> = pp.neighbors([5, 8]).collect();
        let v: Vec<[usize; 2]> = vec![];
        assert_eq!(n_lr_out_of_bounds, v);
    }

    #[test]
    fn to_graph() {
        let pp: PathProblem<5, 8> = TEST_INPUT.parse().unwrap();
        let mut graph = pp.to_graph();
        graph.djikstra();
        assert_eq!(graph.nodes[0].parent, None);
        for i in 1..graph.len() {
            assert!(graph.nodes[i].parent.is_some())
        }
    }
    #[test]
    fn shortest_path() {
        let pp: PathProblem<5, 8> = TEST_INPUT.parse().unwrap();
        let mut graph = pp.to_graph();
        graph.djikstra();
        let path = graph.shortest_path();
        assert_eq!(path.len(), 31);

        assert_eq!(graph.shortest_path_len(), 31);
    }

    #[test]
    fn part_1() {
        let pp: PathProblem<41, 162> = INPUT.parse().unwrap();
        let mut g = pp.to_graph();
        g.djikstra();
        println!("{:?}", g.shortest_path_len());
    }
}
