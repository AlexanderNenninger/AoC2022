#![allow(unused)]

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
    iter::once,
    rc::{Rc, Weak},
};

use derive_more;
use lazy_static::lazy_static;
// use mimalloc::MiMalloc;
use regex::Regex;

//#[global_allocator]
// static GLOBAL: MiMalloc = MiMalloc;

use crate::{
    etc::{ErasedError, HeapArray},
    Solution, SolutionPair,
};

#[derive(Debug, Default, Clone, derive_more::Deref, derive_more::DerefMut)]
struct CaveSystem(HashMap<String, Cave>);

impl TryFrom<&str> for CaveSystem {
    type Error = ErasedError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut cave_map: HashMap<String, Cave> = HashMap::new();
        for line in value.lines() {
            let cave: Cave = line.try_into()?;
            cave_map.insert(cave.name.clone(), cave);
        }

        Ok(CaveSystem(cave_map))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Cave {
    name: String,
    data: i32,
    neighbors: Vec<String>,
}

impl Cave {
    /// Create Cave from referenced data. Allocates.
    fn new(name: &str, data: i32, neighbors: &[&str]) -> Cave {
        Cave {
            name: name.to_owned(),
            data: data,
            neighbors: neighbors.iter().map(|&s| s.to_owned()).collect(),
        }
    }
}

impl TryFrom<&str> for Cave {
    type Error = ErasedError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref RE_NODE: Regex =
                Regex::new(r"^Valve (?P<name>[A-Z]{2}) has flow rate=(?P<data>[0-9]+); tunnels? leads? to valves? (?P<neighbors>([A-Z]{2}(?:,\s*|$))+)").unwrap();
        }

        let caps = RE_NODE
            .captures(value)
            .ok_or_else(|| format!("ERROR: '{value}' did not match spec."))?;

        let name: &str = caps.name("name").unwrap().as_str();
        let data: i32 = caps["data"].parse()?;
        let neighbors: Vec<&str> = caps
            .name("neighbors")
            .unwrap()
            .as_str()
            .trim()
            .split(", ")
            .collect();

        Ok(Cave::new(name, data, &neighbors))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ActionData {
    Root(String),
    MoveTo(String),
    OpenValve(String),
}

impl ActionData {
    fn inner(&self) -> &str {
        match self {
            ActionData::Root(s) => &s,
            ActionData::MoveTo(s) => &s,
            ActionData::OpenValve(s) => &s,
        }
    }

    fn next_actions<'a>(
        &self,
        cave_system: &'a CaveSystem,
    ) -> impl Iterator<Item = ActionData> + 'a {
        use ActionData::*;

        let base_cave = cave_system.get(self.inner()).expect(&format!(
            "ERROR: ActionData {self:?} does not reference a cave."
        ));
        base_cave
            .neighbors
            .iter()
            .map(|id| MoveTo(id.clone()))
            .chain(once(OpenValve(self.inner().to_owned())))
    }
}

type Refrc<T> = Rc<RefCell<T>>;

struct ActionNode<const N: usize> {
    parent: Option<Refrc<ActionNode<N>>>,
    children: HeapArray<N, Refrc<ActionNode<N>>>,
    depth: i32,
    data: ActionData,
}

impl<const N: usize> ActionNode<N> {
    /// SAFETY: Self can never be moved.
    /// Returns self, child
    fn new_child(mut self, data: ActionData) -> (Refrc<ActionNode<N>>, Refrc<ActionNode<N>>) {
        let self_ref = Refrc::new(RefCell::new(self));
        let child = ActionNode {
            parent: Some(self_ref),
            children: HeapArray::<N, Refrc<ActionNode<N>>>::new(),
            depth: self.depth + 1,
            data,
        };

        self.children.push(Refrc::new(RefCell::new(child)));
        let n = self.children.len() - 1;
        (self_ref, self.children[n])
    }
}

impl<const N: usize> Debug for ActionNode<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ActionNode")
            .field(
                "parent",
                &match self.parent {
                    Some(parent) => parent.borrow().data.inner(),
                    None => "None",
                },
            )
            .field("children", &self.children)
            .field("depth", &self.depth)
            .field("data", &self.data)
            .finish()
    }
}

impl<const N: usize> Drop for ActionNode<N> {
    fn drop(&mut self) {
        self.parent = None;
    }
}

impl<const N: usize> ActionNode<N> {
    /// SAFETY: None of the ActionNodes get ever reallocated.
    fn from_cave_system_bfs(cave_system: &CaveSystem, start: &str, max_depth: i32) -> Self {
        let root_data = ActionData::Root(start.into());
        let mut root_node = ActionNode {
            parent: None,
            children: HeapArray::<N, _>::new(),
            depth: -1,
            data: root_data,
        };

        let mut done: HashSet<ActionData> = HashSet::new();
        let mut action_queue: VecDeque<Refrc<ActionNode<N>>> = VecDeque::new();
        action_queue.push_front(Rc::new(RefCell::new(root_node)));

        while let Some(action_node) = action_queue.pop_back() {
            unsafe {
                if action_node.borrow().depth <= max_depth {
                    for action in action_node.borrow().data.next_actions(cave_system) {
                        if !done.contains(&action) {
                            if matches!(action, ActionData::OpenValve(_)) {
                                done.insert(action.clone());
                            }
                            let (_, child) = action_node.borrow_mut().new_child(action);
                            action_queue.push_front(child)
                        }
                    }
                }
            }
        }
        root_node
    }

    fn total_flow(&self, cave_system: &CaveSystem, max_depth: i32) -> i32 {
        let key = self.data.inner();
        let flow_rate = match self.data {
            ActionData::OpenValve(_) => cave_system[key].data,
            _ => 0,
        };
        flow_rate * (max_depth - self.depth)
    }

    fn score(&self, cave_system: &CaveSystem, max_depth: i32) -> i32 {
        let mut current_node = self;
        let mut acc = current_node.total_flow(cave_system, max_depth);

        while let Some(parent) = current_node.parent {
            current_node = &*(*parent).borrow();
            acc += current_node.total_flow(cave_system, max_depth);
        }
        acc
    }

    fn is_leaf(&self) -> bool {
        self.children.len() == 0
    }
}

const INPUT: &str = include_str!("../../input/day16.txt");

pub fn solve() -> SolutionPair {
    (Solution::I64(0), Solution::I64(0))
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day16_test.txt");

    #[test]
    fn try_from_str() -> Result<(), ErasedError> {
        let cave_system: CaveSystem = TEST_INPUT.try_into()?;
        dbg!(&cave_system);
        assert!(cave_system.iter().all(|(k, v)| *k == v.name));
        Ok(())
    }

    #[test]
    fn gen_action_tree() -> Result<(), ErasedError> {
        let cave_system: CaveSystem = TEST_INPUT.try_into()?;
        let _ = ActionNode::<4>::from_cave_system_bfs(&cave_system, "AA", 3);
        Ok(())
    }

    #[test]
    fn find_leafs() -> Result<(), ErasedError> {
        const MAX_DEGREE: usize = 4;
        let max_depth = 3;
        let cave_system: CaveSystem = TEST_INPUT.try_into()?;
        let action_tree =
            ActionNode::<MAX_DEGREE>::from_cave_system_bfs(&cave_system, "AA", max_depth);
        let leafs = action_tree.leafs();
        assert!(leafs.len() <= MAX_DEGREE.pow(max_depth as u32 + 1));
        Ok(())
    }

    #[test]
    fn total_flow() -> Result<(), ErasedError> {
        const MAX_DEGREE: usize = 4;
        let max_depth: i32 = 3;
        let cave_system: CaveSystem = TEST_INPUT.try_into()?;
        let action_tree =
            ActionNode::<MAX_DEGREE>::from_cave_system_bfs(&cave_system, "AA", max_depth);
        let leafs = action_tree.leafs();

        let flows: Vec<_> = leafs
            .into_iter()
            .map(|leaf| leaf.total_flow(&cave_system, max_depth))
            .collect();
        assert!(flows.into_iter().all(|flow| flow == 0));

        Ok(())
    }

    #[test]
    fn score() -> Result<(), ErasedError> {
        const MAX_DEGREE: usize = 10;
        let max_depth: i32 = 3;
        let cave_system: CaveSystem = TEST_INPUT.try_into()?;
        let action_tree =
            ActionNode::<MAX_DEGREE>::from_cave_system_bfs(&cave_system, "AA", max_depth);
        let leafs = action_tree.leafs();

        let best = leafs
            .into_iter()
            .max_by_key(|leaf| leaf.score(&cave_system, max_depth))
            .ok_or("ERROR: No leafs.")?;
        dbg!(best, best.score(&cave_system, max_depth));
        Ok(())
    }
}
