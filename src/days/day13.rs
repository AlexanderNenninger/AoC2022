use std::{fmt::Debug, str::FromStr};

use crate::{etc::ErasedError, Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq)]
enum Token {
    BrOpen,
    BrClose,
    Lit(i64),
}

impl Token {
    /// Returns `true` if the token is [`BrOpen`].
    ///
    /// [`BrOpen`]: Token::BrOpen
    #[must_use]
    fn is_br_open(&self) -> bool {
        matches!(self, Self::BrOpen)
    }

    /// Returns `true` if the token is [`BrClose`].
    ///
    /// [`BrClose`]: Token::BrClose
    #[must_use]
    fn is_br_close(&self) -> bool {
        matches!(self, Self::BrClose)
    }

    /// Returns `true` if the token is [`Lit`].
    ///
    /// [`Lit`]: Token::Lit
    #[must_use]
    fn is_lit(&self) -> bool {
        matches!(self, Self::Lit(..))
    }
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
                Value(i) => unreachable!("ERROR: current_node points to a literal value."),
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

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1: u64 = 0;
    let sol2: u64 = 0;

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
}
