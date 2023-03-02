use std::{cmp, fmt::Debug, str::FromStr};

use crate::{etc::ErasedError, Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ThreeValuedOrdering {
    Less,
    Greater,
    Undecidable,
}

impl ThreeValuedOrdering {
    /// Returns `true` if the three valued ordering is [`Less`].
    ///
    /// [`Less`]: ThreeValuedOrdering::Less
    #[must_use]
    fn is_less(&self) -> bool {
        matches!(self, Self::Less)
    }

    /// Returns `true` if the three valued ordering is [`Greater`].
    ///
    /// [`Greater`]: ThreeValuedOrdering::Greater
    #[allow(unused)]
    #[must_use]
    fn is_greater(&self) -> bool {
        matches!(self, Self::Greater)
    }

    /// Returns `true` if the three valued ordering is [`Undecidable`].
    ///
    /// [`Undecidable`]: ThreeValuedOrdering::Undecidable
    #[allow(unused)]
    #[must_use]
    fn is_undecidable(&self) -> bool {
        matches!(self, Self::Undecidable)
    }
}

trait ThreeValuedOrd {
    fn three_cmp(&self, other: &Self) -> ThreeValuedOrdering;
}

impl ThreeValuedOrd for i64 {
    #[inline]
    fn three_cmp(&self, other: &Self) -> ThreeValuedOrdering {
        use ThreeValuedOrdering::*;
        match self.partial_cmp(other) {
            Some(ord) => match ord {
                cmp::Ordering::Less => Less,
                cmp::Ordering::Equal => Undecidable,
                cmp::Ordering::Greater => Greater,
            },
            None => Undecidable,
        }
    }
}

impl<T> ThreeValuedOrd for Vec<T>
where
    T: ThreeValuedOrd,
{
    fn three_cmp(&self, other: &Self) -> ThreeValuedOrdering {
        use ThreeValuedOrdering::*;
        let mut l_it = self.iter();
        let mut r_it = other.iter();
        loop {
            return match (l_it.next(), r_it.next()) {
                (None, None) => Undecidable,
                (None, Some(_)) => Less,
                (Some(_), None) => Greater,
                (Some(l_item), Some(r_item)) => match l_item.three_cmp(r_item) {
                    Less => Less,
                    Greater => Greater,
                    Undecidable => continue,
                },
            };
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Token {
    BrOpen,
    BrClose,
    Lit(i64),
}

/// Tokenizer using finite state machine logic to tokenize nested lists of integers.
/// e.g. s = [42, [69, 420]]
fn tokenize(s: &str) -> Result<Vec<Token>, ErasedError> {
    let char_err = |c: &char| format!("ERROR: Character {c} unknown");

    use Token::*;

    enum TokenizerState {
        ParsingLiteral,
        AwaitingToken,
    }
    use TokenizerState::*;

    let mut tokenizer_state = AwaitingToken;
    let mut token_stack = Vec::new();
    let mut literal_stack = String::new();

    for c in s.chars() {
        match tokenizer_state {
            AwaitingToken => match c {
                '[' => {
                    tokenizer_state = AwaitingToken;
                    token_stack.push(BrOpen);
                }
                ']' => {
                    tokenizer_state = AwaitingToken;
                    token_stack.push(BrClose);
                }
                ',' => tokenizer_state = AwaitingToken,
                d if d.is_numeric() => {
                    tokenizer_state = ParsingLiteral;
                    literal_stack.push(d);
                }
                _ => return Err(char_err(&c).into()),
            },
            ParsingLiteral => match c {
                '[' => {
                    tokenizer_state = AwaitingToken;
                    token_stack.push(Lit(literal_stack.parse()?));
                    literal_stack.clear();
                    token_stack.push(BrOpen)
                }
                ']' => {
                    tokenizer_state = AwaitingToken;
                    token_stack.push(Lit(literal_stack.parse()?));
                    literal_stack.clear();
                    token_stack.push(BrClose)
                }
                ',' => {
                    tokenizer_state = AwaitingToken;
                    token_stack.push(Lit(literal_stack.parse()?));
                    literal_stack.clear();
                }
                d if d.is_numeric() => {
                    literal_stack.push(d);
                }
                _ => return Err(char_err(&c).into()),
            },
        }
    }

    if !literal_stack.is_empty() {
        token_stack.push(Lit(literal_stack.parse()?));
        literal_stack.clear();
    }

    Ok(token_stack)
}

#[derive(Debug, Clone, PartialEq)]
enum Packet {
    Value(i64),
    List(Vec<Packet>),
}

impl FromStr for Packet {
    type Err = ErasedError;

    /// Iterative single pass parser. Begin with current_node as an empty list.
    /// Any time we encounter an opening bracket, we add a new list to the children of
    /// the current node and update our current_node to point to the end ot the just pushed list.
    /// If we encounter a closing bracket, we move up one level in the AST.
    /// Parents are tracked using a stack of raw pointers we push to and pop from. This ensures
    /// we aren't aliasing anything. Miri seems to be content.
    /// If we hit a Literal, we just add it to the children of current_node.
    /// e.g. s = "[[12,[]],1,23]"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Packet::*;
        use Token::*;

        let parser_error = |s| Err(format!("ERROR: Parsing of {s:?} unsuccessful.").into());

        let tokens = tokenize(s)?;
        if tokens.len() == 0 {
            return Err("ERROR: No tokens.".into());
        }
        if tokens.len() == 1 {
            match tokens[0] {
                Lit(i) => return Ok(Value(i)),
                _ => return parser_error(s),
            }
        }
        let mut root = match tokens[0] {
            Lit(i) => return Ok(Value(i)),
            BrOpen => List(Vec::new()),
            BrClose => return parser_error(s),
        };

        let mut current_node: *mut Packet = &mut root;
        let mut parents: Vec<*mut Packet> = vec![current_node.clone()];

        for token in &tokens[1..] {
            // SAFETY: We are not using parents to alias any nodes.
            // The only access occurs through `current_node = parents.pop().unwrap()`
            match unsafe { &mut *current_node } {
                Value(_) => unreachable!("ERROR: current_node points to a literal value."),
                List(v) => match token {
                    BrOpen => {
                        v.push(List(Vec::new()));
                        parents.push(current_node);
                        current_node = v.last_mut().expect("ERROR: Node should not be empty.");
                    }
                    BrClose => {
                        current_node = match parents.pop() {
                            Some(p) => p,
                            None => return Err("ERROR: Popped from empty parents stack.".into()),
                        };
                    }
                    Lit(i) => v.push(Value(*i)),
                },
            }
        }
        Ok(root)
    }
}

impl ThreeValuedOrd for Packet {
    fn three_cmp(&self, other: &Self) -> ThreeValuedOrdering {
        use Packet::*;
        match (self, other) {
            (Value(l_val), Value(r_val)) => l_val.three_cmp(r_val),
            (Value(l_val), List(r_vec)) => vec![Value(*l_val)].three_cmp(r_vec),
            (List(l_vec), Value(r_val)) => l_vec.three_cmp(&vec![Value(*r_val)]),
            (List(l_vec), List(r_vec)) => l_vec.three_cmp(r_vec),
        }
    }
}

impl PartialOrd for Packet {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        use cmp::Ordering::*;
        match self.three_cmp(other) {
            ThreeValuedOrdering::Less => Some(Less),
            ThreeValuedOrdering::Greater => Some(Greater),
            ThreeValuedOrdering::Undecidable => None,
        }
    }
}

fn read_data(s: &str) -> Result<Vec<(Packet, Packet)>, ErasedError> {
    let mut out = Vec::new();
    let parse_error = |s: &str| format!("ERROR: could not parse '{s}'.");
    for pair in s.split("\n\n") {
        let (fst, snd) = pair.split_once("\n").ok_or(parse_error(pair))?;
        out.push((fst.trim().parse()?, snd.trim().parse()?));
    }
    Ok(out)
}

pub fn solve() -> SolutionPair {
    const INPUT: &str = include_str!("../../input/day13.txt");
    let pairs = read_data(INPUT).unwrap();
    let mut idx_sum: u64 = 0;
    for (i, (left, right)) in pairs.iter().enumerate() {
        if left.three_cmp(right).is_less() {
            idx_sum += i as u64 + 1;
        }
    }

    let mut input_2 = INPUT.replace("\n\n", "\n").trim().to_string();
    input_2.push_str("\n[[2]]\n[[6]]");
    let mut packets: Vec<Packet> = input_2.lines().map(|l| l.parse().unwrap()).collect();
    packets.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let p2: Packet = "[[2]]".parse().unwrap();
    let p6: Packet = "[[6]]".parse().unwrap();

    let sol1: u64 = idx_sum;
    let sol2: u64 = packets
        .into_iter()
        .enumerate()
        .filter(|(_, p)| *p == p2 || *p == p6)
        .map(|(i, _)| i as u64 + 1)
        .product();

    (Solution::U64(sol1), Solution::U64(sol2))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tokenize() {
        let s = "[1,[2,[3,[4,[5,6,7]]]],8,99]";
        let tokens = tokenize(s).unwrap();
        assert_eq!(tokens[0], Token::BrOpen);
        assert_eq!(tokens[17], Token::Lit(99))
    }

    #[test]
    fn test_tokenize_literal() {
        let s = "99";
        let tokens = tokenize(s).unwrap();
        assert_eq!(tokens[0], Token::Lit(99));
    }

    #[test]
    fn test_parse_literal() {
        let s = "1";
        let packets: Packet = s.parse().unwrap();
        assert_eq!(packets, Packet::Value(1));
    }

    #[test]
    fn test_parse_empty() {
        let s = "[]";
        let packets: Packet = s.parse().unwrap();
        println!("{packets:?}");
    }

    #[test]
    fn test_parse_nested() {
        let s = "[[12,34],5,[]]";
        let packets: Packet = s.parse().unwrap();
        println!("{packets:?}");
    }

    #[test]
    fn test_three_valued_cmp_long() {
        use ThreeValuedOrdering::*;
        let left_data = "[1,[2,[3,[4,[5,6,7]]]],8,9]";
        let right_data = "[1,[2,[3,[4,[5,6,0]]]],8,9]";

        let left_packet: Packet = left_data.parse().unwrap();
        let right_packet: Packet = right_data.parse().unwrap();

        assert_eq!(left_packet.three_cmp(&right_packet), Greater);
    }

    #[test]
    fn test_three_valued_cmp_empty() {
        use ThreeValuedOrdering::*;
        let left_data = "[[[]]]";
        let right_data = "[[]]";

        let left_packet: Packet = left_data.parse().unwrap();
        let right_packet: Packet = right_data.parse().unwrap();

        assert_eq!(left_packet.three_cmp(&right_packet), Greater);
    }
}
