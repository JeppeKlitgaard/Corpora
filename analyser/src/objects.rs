use std::collections::HashMap;

use indexmap::IndexMap;

pub type NGrams = HashMap<u8, IndexMap<String, usize>>;
