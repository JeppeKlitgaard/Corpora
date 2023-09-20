use std::collections::HashMap;

use counter::Counter;
use indexmap::IndexMap;

pub type NGram = Counter<String>;
pub type NGrams = HashMap<u8, NGram>;

pub type SortedNGram = IndexMap<String, usize>;
pub type SortedNGrams = HashMap<u8, SortedNGram>;

pub type NGramsPython = HashMap<u8, IndexMap<String, usize>>;
