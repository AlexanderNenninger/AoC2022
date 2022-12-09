use std::{str::FromStr, vec};

use crate::{etc::ErasedError, Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
struct Directory {
    children: Vec<usize>,
}

impl Directory {
    fn new(children: Vec<usize>) -> Self {
        Directory { children }
    }

    fn size(&self, buf: &Vec<Node>) -> u64 {
        let mut acc = 0;
        for child_idx in &self.children {
            let child = &buf[*child_idx];
            acc += child.size(buf);
        }
        acc
    }
}

impl FromStr for Directory {
    type Err = ErasedError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (prefix, _name) = s
            .split_once(" ")
            .ok_or("ERROR: Directory input needs to be of the form 'dir *name*'")?;
        if prefix == "dir" {
            return Ok(Directory { children: vec![] });
        }
        Err("ERROR: Not a Directory.".into())
    }
}

#[derive(Debug, Clone, PartialEq)]
struct File {
    size: u64,
}

impl File {
    fn _new(size: u64) -> Self {
        File { size }
    }

    fn size(&self, _buf: &Vec<Node>) -> u64 {
        return self.size;
    }
}

impl FromStr for File {
    type Err = ErasedError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (size, _name) = s
            .split_once(" ")
            .ok_or("ERROR: File input needs to be of the Form '*size* *name*'")?;
        let size: u64 = size.parse()?;
        Ok(File { size })
    }
}

#[derive(Debug, Clone, PartialEq)]
enum NodeType {
    File(File),
    Dir(Directory),
}

impl NodeType {
    fn size(&self, buf: &Vec<Node>) -> u64 {
        match self {
            NodeType::File(f) => f.size(buf),
            NodeType::Dir(d) => d.size(buf),
        }
    }
}

impl FromStr for NodeType {
    type Err = ErasedError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (prefix, _) = s.split_once(" ").ok_or("ERROR: Not a Directory or File.")?;
        Ok(match prefix {
            "dir" => Self::Dir(Directory::from_str(s)?),
            _ if prefix.as_bytes().iter().all(|b| b.is_ascii_digit()) => {
                Self::File(File::from_str(s)?)
            }
            _ => Err("ERROR: Unknown prefix.".to_string())?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Node {
    name: String,
    type_: NodeType,
    parent: Option<usize>,
}

impl Node {
    fn size(&self, buf: &Vec<Node>) -> u64 {
        self.type_.size(buf)
    }
}

impl FromStr for Node {
    type Err = ErasedError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, name) = s.split_once(" ").ok_or("ERROR: Invalid node input.")?;
        let type_: NodeType = s.parse()?;
        return Ok(Node {
            name: name.into(),
            type_,
            parent: None,
        });
    }
}

#[derive(Debug, Clone)]
enum Command {
    MoveIn(String),
    MoveOut,
    Ls,
}

impl FromStr for Command {
    type Err = ErasedError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(("$", cmd)) = s.split_once(" ") {
            if let Some(("cd", dirname)) = cmd.split_once(" ") {
                if dirname == ".." {
                    return Ok(Self::MoveOut);
                } else {
                    return Ok(Self::MoveIn(dirname.into()));
                }
            } else if cmd == "ls" {
                return Ok(Self::Ls);
            }
            Err("ERROR: Unknown command.".into())
        } else {
            Err("ERROR: Not a command.".into())
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<Node>, ErasedError> {
    let mut node_buffer: Vec<Node> = vec![];
    enum ParserState {
        Command,
        Node,
    }
    let mut parser_state = ParserState::Command;
    let mut current_directory_idx = 0usize;

    let base = Node {
        name: "".into(),
        type_: NodeType::Dir(Directory::new(vec![1])),
        parent: None,
    };

    node_buffer.push(base);

    let root = Node {
        name: "/".into(),
        type_: NodeType::Dir(Directory::new(vec![])),
        parent: Some(current_directory_idx),
    };

    node_buffer.push(root);

    for line in input.trim().lines() {
        if line.starts_with("$") {
            parser_state = ParserState::Command
        };

        if let ParserState::Command = parser_state {
            let cmd: Command = line.parse()?;
            match cmd {
                Command::MoveIn(dir) => {
                    let current_node = &node_buffer[current_directory_idx];
                    if let NodeType::Dir(ref cur_dir) = current_node.type_ {
                        let move_target = *cur_dir
                            .children
                            .iter()
                            .find(|&child| node_buffer[*child].name == dir)
                            .ok_or(
                                "ERROR: Tried moving into non-existant directory.".to_string(),
                            )?;
                        current_directory_idx = move_target;
                    }
                }
                Command::Ls => parser_state = ParserState::Node,
                Command::MoveOut => {
                    current_directory_idx = node_buffer[current_directory_idx]
                        .parent
                        .ok_or("ERROR: Tried to 'cd ..' without parent.".to_string())?
                }
            }
        } else if let ParserState::Node = parser_state {
            let n = node_buffer.len();
            if let NodeType::Dir(ref mut dir) = node_buffer[current_directory_idx].type_ {
                dir.children.push(n);

                let mut node: Node = line.parse()?;
                node.parent = Some(current_directory_idx);
                node_buffer.push(node);
            } else {
                Err("ERROR: current_directory_idx points to file.".to_string())?;
            }
        }
    }
    Ok(node_buffer)
}

fn part_1(node_buffer: &Vec<Node>) -> u64 {
    let max_val = 100000;
    node_buffer
        .iter()
        .filter(|&node| matches!(node.type_, NodeType::Dir(_)))
        .map(|dir| dir.size(node_buffer))
        .filter(|s| *s <= max_val)
        .sum()
}

fn part_2(node_buffer: &Vec<Node>) -> u64 {
    let memory_size = 70000000;
    let max_space_used = memory_size - 30000000;
    let total_space_used = node_buffer[0].size(node_buffer);
    let mut node_size;

    let mut max_space_after_deletion = 0;
    let mut ret_val = u64::MAX;
    for node in node_buffer.iter() {
        node_size = node.size(node_buffer);
        let space_after_deletion = total_space_used - node_size;
        if space_after_deletion <= max_space_used && space_after_deletion > max_space_after_deletion
        {
            max_space_after_deletion = space_after_deletion;
            ret_val = node_size
        }
    }
    ret_val
}

pub fn solve() -> SolutionPair {
    let input = include_str!("../../input/day07.txt");
    let node_buffer = parse_input(input).expect("ERROR: Could not parse input.");
    let sol1: u64 = part_1(&node_buffer);
    let sol2: u64 = part_2(&node_buffer);

    (Solution::U64(sol1), Solution::U64(sol2))
}

#[test]
fn test_parse_input() {
    let input = include_str!("../../input/day07_test.txt");
    let node_buffer = parse_input(input).expect("ERROR: Could not parse input.");
    assert_eq!(node_buffer[1].size(&node_buffer), 48381165);
    assert_eq!(part_1(&node_buffer), 95437)
}
