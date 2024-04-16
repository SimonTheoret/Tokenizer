
use ahash::RandomState;
use std::hash::Hash;
use std::{
    cmp::{Eq, PartialEq},
    collections::HashMap,
};

/**
Toy implementation of the BPE. Implementation is from the
following paper: https://arxiv.org/pdf/1508.07909v5.pdf. It
implements two may methods: get_stats, which compute the
statistics of the vocabulary, and merge_vocab, which merge
together the most common token. Together, these two methods make
the BPE algorithm.
*/
#[derive(Default, Debug)]
pub struct ToyTokenizer {}

type Vocab<T> = HashMap<T, i32, RandomState>;
type Pairs<T> = HashMap<(T, T), i32, RandomState>;

impl ToyTokenizer {
    pub fn new() -> Self {
        Self {}
    }

    ///
    pub fn get_stats<T>(self, vocab: Vocab<T>) -> Pairs<String>
    where
        T: ToString + Eq + PartialEq + Hash,
    {
        let mut pairs: Pairs<String> = HashMap::default();
        vocab.iter().for_each(|(key, value)| {
            let key: String = (*key).to_string();
            let symbols: Vec<&str> = key.split(' ').collect(); // ' ' means its a char!
            for i in 0..symbols.len() - 1 {
                let pair: (String, String) = (symbols[i].to_string(), symbols[i + 1].to_string());
                let count = pairs.get_mut(&pair); // Must modify pair's value
                match count {
                    Some(count_value) => *count_value += value,
                    None => _ = pairs.insert(pair, *value),
                };
            }
        });
        pairs
    }

    pub fn merge_vocab<T>(self, pair: (T, T), vocab: Vocab<T>) -> Vocab<T>
    where
        T: ToString + Eq + PartialEq + Hash,
    {
        let new_vocab: Vocab<T> = HashMap::default();

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_stats_once() {
        let tt: ToyTokenizer = ToyTokenizer::new();
        let vocab = build_toy_vocab();
        let expected_stats: HashMap<(String, String), i32, RandomState> = build_expected_stats();
        let actual_stats = tt.get_stats(vocab);
        assert_eq!(actual_stats, expected_stats);
    }

    fn build_toy_vocab() -> HashMap<&'static str, i32, RandomState> {
        let mut vocab: HashMap<&str, i32, RandomState> = HashMap::default();
        vocab.insert("l o w</w>", 5);
        vocab.insert("l o w e r</w>", 2);
        vocab.insert("n e w e s t</w>", 6);
        vocab.insert("w i d e s t</w>", 3);
        vocab
    }
    fn build_expected_stats() -> HashMap<(String, String), i32, RandomState> {
        let tt = ToyTokenizer::new();
        let mut expected_stats: HashMap<(String, String), i32, RandomState> = HashMap::default();
        expected_stats.insert(("l".to_string(), "o".to_string()), 7);
        expected_stats.insert(("o".to_string(), "w</w>".to_string()), 5);
        expected_stats.insert(("o".to_string(), "w".to_string()), 2);
        expected_stats.insert(("w".to_string(), "e".to_string()), 8);
        expected_stats.insert(("e".to_string(), "r</w>".to_string()), 2);
        expected_stats.insert(("n".to_string(), "e".to_string()), 6);
        expected_stats.insert(("e".to_string(), "w".to_string()), 6);
        expected_stats.insert(("e".to_string(), "s".to_string()), 9);
        expected_stats.insert(("s".to_string(), "t</w>".to_string()), 9);
        expected_stats.insert(("w".to_string(), "i".to_string()), 3);
        expected_stats.insert(("i".to_string(), "d".to_string()), 3);
        expected_stats.insert(("d".to_string(), "e".to_string()), 3);
        expected_stats
    }
}
