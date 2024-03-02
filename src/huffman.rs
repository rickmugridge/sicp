use std::cmp::Ordering;
use std::collections::HashSet;
use std::ops::Add;

#[derive(Debug, PartialEq, Clone)]
enum Huffman {
    Leaf(char, usize),
    NonLeaf(Box<Huffman>, Box<Huffman>, HashSet<char>, usize),
}

impl Huffman {
    fn new_leaf(ch: char, weight: usize) -> Self {
        Huffman::Leaf(ch, weight)
    }

    fn new_non_leaf(left: Huffman, right: Huffman, chars: HashSet<char>, weight: usize) -> Self {
        Huffman::NonLeaf(Box::new(left), Box::new(right),
                         chars, weight)
    }

    fn contains(&self, ch: char) -> bool {
        match self {
            Huffman::Leaf(c, _) => *c == ch,
            Huffman::NonLeaf(_, _, set, _) =>
                set.contains(&ch),
        }
    }

    fn encode(&self, s: &str) -> String {
        let mut result = String::new();
        s.chars().for_each(|ch| result.push_str(&self.encode_char(ch)));
        result
    }

    fn encode_char(&self, ch: char) -> String {
        match self {
            Huffman::Leaf(c, _) if *c == ch => "".to_string(),
            Huffman::Leaf(c, _) => panic!("Unable to match {c}"),
            Huffman::NonLeaf(left, right, _, _) => {
                if left.contains(ch) {
                    "0".to_string().add(&left.encode_char(ch))
                } else if right.contains(ch) {
                    "1".to_string().add(&right.encode_char(ch))
                } else {
                    panic!("Unable to match {ch}");
                }
            }
        }
    }

    fn decode(&self, s: &str) -> String {
        let bits: Vec<char> = s.chars().into_iter().collect();
        let mut remaining_bits: &[char] = &bits;
        let mut result = String::new();
        loop {
            let (ch, remaining) = self.decode_with(&remaining_bits);
            result.push(ch);
            if remaining.is_empty() { break; }
            remaining_bits = remaining;
        }
        result
    }

    fn decode_with<'a>(&self, bits: &'a [char]) -> (char, &'a [char]) {
        match self {
            Huffman::Leaf(c, _) => (*c, bits),
            Huffman::NonLeaf(left, right, _, _) => {
                if bits.len() > 0 {
                    if bits[0] == '0' {
                        left.decode_with(&bits[1..])
                    } else if bits[0] == '1' {
                        right.decode_with(&bits[1..])
                    } else {
                        panic!("Not able to decode with {:?}", bits);
                    }
                } else {
                    panic!("Not able to decode at NonLeaf with empty bits");
                }
            }
        }
    }

    fn make(leaves: Vec<(char, usize)>) -> Self {
        assert!(leaves.len() >= 2);
        let mut huffs: Vec<Huffman> = leaves.iter()
            .map(|(ch, weight)| Self::new_leaf(*ch, *weight))
            .collect();
        huffs.sort_by(|a, b| b.cmp(a)); // descending order
        println!(" huffs: {:?}", huffs);
        Self::build(huffs)
    }

    fn build(given_leaves: Vec<Huffman>) -> Self {
        let mut leaves = given_leaves;
        loop {
            let count = &leaves.len();
            let left = &leaves.remove(count - 1);
            let right = &leaves.remove(count - 2);
            let combined = Self::build_from_pair(left, right);
            if count == &2 {
                return combined;
            }
            leaves.push(combined);
            leaves.sort_by(|a, b| b.cmp(a)); // descending order
        }
    }

    fn build_from_pair(left: &Huffman, right: &Huffman) -> Huffman {
        let mut chars = (&left).chars();
        right.chars().iter().for_each(|ch| { chars.insert(*ch); });
        let weight = left.weight() + right.weight();
        Self::new_non_leaf(left.clone(), right.clone(), chars, weight)
    }

    fn weight(&self) -> usize {
        match self {
            Huffman::Leaf(_, weight) => *weight,
            Huffman::NonLeaf(_, _, _, weight) => *weight,
        }
    }

    fn chars(&self) -> HashSet<char> {
        match self {
            Huffman::Leaf(chars, _) => {
                let mut set = HashSet::new();
                set.insert(*chars);
                set
            }
            Huffman::NonLeaf(_, _, chars, _) => chars.clone(),
        }
    }
}

impl Eq for Huffman {}

impl PartialOrd<Self> for Huffman {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.weight().cmp(&other.weight()))
    }
}

impl Ord for Huffman {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight().cmp(&other.weight())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leaf() {
        let huff = Huffman::new_leaf('a', 3);
        assert_eq!(huff.contains('a'), true);
        assert_eq!(huff.contains('b'), false);
        let expected_chars: HashSet<char> = HashSet::from_iter(vec!['a']);
        assert_eq!(huff.chars(), expected_chars);
        assert_eq!(huff.weight(), 3);
        assert_eq!(huff.encode("a"), "".to_string());
        assert_eq!(huff.decode(""), "a");
    }

    #[test]
    fn non_leaf() {
        let huff1 = Huffman::new_leaf('a', 3);
        let huff2 = Huffman::new_leaf('b', 4);
        let huff = Huffman::build_from_pair(&huff1, &huff2);
        assert_eq!(huff.contains('a'), true);
        assert_eq!(huff.contains('b'), true);
        assert_eq!(huff.contains('c'), false);
        let expected_chars: HashSet<char> = HashSet::from_iter(vec!['a', 'b']);
        assert_eq!(huff.chars(), expected_chars);
        assert_eq!(huff.weight(), 7);
        assert_eq!(huff.encode("ab"), "01".to_string());
        assert_eq!(huff.decode("01"), "ab");
    }

    #[test]
    fn make_encode_decode() {
        let huff = Huffman::make(vec![
            ('a', 8), ('b', 3), ('c', 1), ('d', 1), ('e', 1), ('f', 1), ('g', 1), ]);
        assert_eq!(huff.contains('a'), true);
        let expected_chars: HashSet<char> = HashSet::from_iter(vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']);
        assert_eq!(huff.chars(), expected_chars);
        assert_eq!(huff.weight(), 16);

        assert_eq!(huff.encode("a"), "1".to_string());
        assert_eq!(huff.encode("b"), "00".to_string());
        assert_eq!(huff.encode("c"), "0110".to_string());
        assert_eq!(huff.encode("d"), "01111".to_string());
        assert_eq!(huff.encode("e"), "01110".to_string());
        assert_eq!(huff.encode("f"), "0101".to_string());
        assert_eq!(huff.encode("g"), "0100".to_string());

        assert_eq!(huff.encode("ab"), "100".to_string());
        assert_eq!(huff.encode("cd"), "011001111".to_string());
        assert_eq!(huff.encode("dc"), "011110110".to_string());
        assert_eq!(huff.encode("cabeg"), "0110100011100100".to_string());

        assert_eq!(huff.encode("abcdefg"), "1000110011110111001010100".to_string());
        assert_eq!(huff.decode("1000110011110111001010100"), "abcdefg".to_string());
    }
}