use std::cmp::Ordering;
use std::ops::{AddAssign, Mul};

use counter::Counter;
use indexmap::map::Entry;
use indexmap::map::Keys;
use indexmap::IndexMap;
use num_traits::NumCast;
use serde::{Deserialize, Serialize};

use delegate::delegate;
use smartstring::{LazyCompact, SmartString};

pub trait NGramLikeT = AddAssign + Send + Copy + Default;

pub type Countable = SmartString<LazyCompact>;
pub trait OccuranceT =
    num_traits::Zero + num_traits::One + PartialOrd + Default + AddAssign + Send + Copy;

pub type OccuranceCounter = Counter<Countable, usize>;
/// This is simply used to efficiently do the counting within a thread

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Occurances<T: OccuranceT>(IndexMap<Countable, T>);
impl<T: OccuranceT> Occurances<T> {
    pub fn new() -> Occurances<T> {
        let counter: IndexMap<Countable, T> = IndexMap::new();
        Self(counter)
    }

    delegate! {
        to self.0 {
            pub fn into_iter(self) -> impl Iterator<Item = (Countable, T)>;
            pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Countable, &mut T)>;
            pub fn keys(&self) -> Keys<Countable, T>;
            pub fn entry(&mut self, key: Countable) -> Entry<Countable, T>;
            pub fn par_sort_by<F>(&mut self, cmp: F)
                where F: Fn(&Countable, &T, &Countable, &T) -> Ordering + Sync;
            }
    }
}

impl Occurances<f64> {
    pub fn normalize(&mut self) {
        let total: f64 = self.0.values().sum();
        self.0.iter_mut().for_each(|(_, count)| *count /= total);
    }
}

impl From<Counter<Countable>> for Occurances<usize> {
    fn from(counter: Counter<Countable>) -> Self {
        let inner: IndexMap<Countable, usize> = counter.into_iter().collect();
        Self(inner)
    }
}

impl<T> From<Occurances<T>> for IndexMap<Countable, T>
where
    T: OccuranceT,
{
    fn from(occurances: Occurances<T>) -> Self {
        occurances.0
    }
}

impl<T> FromIterator<(Countable, T)> for Occurances<T>
where
    T: OccuranceT,
{
    fn from_iter<I: IntoIterator<Item = (Countable, T)>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<T> AddAssign for Occurances<T>
where
    T: OccuranceT,
{
    fn add_assign(&mut self, other: Self) {
        for (key2, value2) in other.into_iter() {
            self.entry(key2.clone())
                .and_modify(|e| *e += value2)
                .or_insert(value2);
        }
    }
}

pub type NOccurances<T> = IndexMap<usize, Occurances<T>>;
/// A map of occurances indexed by the ordinate of the occurances.
/// Can be ngram1, ngram2, skipgram1, skipgram2, etc

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct OccuranceAnalysis<T>
where
    T: OccuranceT,
{
    pub ngrams: NOccurances<T>,
    pub skipgrams: NOccurances<T>,
    pub words: Occurances<T>,
}

impl<T> OccuranceAnalysis<T>
where
    T: OccuranceT,
{
    pub fn sort(&mut self) {
        let occurances = self.ngrams.values_mut().chain(self.skipgrams.values_mut());
        occurances.for_each(|occurance| {
            occurance.par_sort_by(|_, v1, _, v2| v2.partial_cmp(v1).unwrap());
        });
    }
}

impl OccuranceAnalysis<f64> {
    pub fn normalize(&mut self) {
        // Normalise occurances of ngrams and skipgrams
        self.ngrams
            .values_mut()
            .chain(self.skipgrams.values_mut())
            .for_each(|occurances| occurances.normalize());
    }
}

impl<T> AddAssign for OccuranceAnalysis<T>
where
    T: OccuranceT,
{
    fn add_assign(&mut self, other: Self) {
        for (n, occurance) in other.ngrams {
            let entry = self.ngrams.entry(n).or_default();
            *entry += occurance;
        }

        for (n, occurance) in other.skipgrams {
            let entry = self.skipgrams.entry(n).or_default();
            *entry += occurance;
        }

        self.words += other.words;
    }
}

impl<T> Mul<f64> for OccuranceAnalysis<T>
where
    T: OccuranceT + NumCast,
{
    type Output = OccuranceAnalysis<f64>;
    fn mul(self, rhs: f64) -> Self::Output {
        let ngrams = self.ngrams.into_iter().map(|(n, counter)| {
            let new_counter = counter
                .into_iter()
                .map(|(s, count)| (s, NumCast::from(count).unwrap_or(0.0) * rhs))
                .collect::<Occurances<f64>>();
            (n, new_counter)
        });

        let skipgrams = self.skipgrams.into_iter().map(|(n, counter)| {
            let new_counter = counter
                .into_iter()
                .map(|(s, count)| (s, NumCast::from(count).unwrap_or(0.0) * rhs))
                .collect::<Occurances<f64>>();
            (n, new_counter)
        });

        let words = self
            .words
            .into_iter()
            .map(|(s, count)| (s, NumCast::from(count).unwrap_or(0.0) * rhs));

        let ngrams: NOccurances<f64> = ngrams.collect();
        let skipgrams: NOccurances<f64> = skipgrams.collect();
        let words: Occurances<f64> = words.collect();

        OccuranceAnalysis { ngrams, skipgrams, words }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_into_iterator_to_occurances() {
        let src: Vec<Countable> = vec!["aa".into(), "bb".into(), "aa".into()];
        let occurances: OccuranceCounter = src.into_iter().collect();

        assert_eq!(occurances.get("aa".into()), Some(&2)); // "aa" occurs twice
        assert_eq!(occurances.get("bb".into()), Some(&1)); // "bb" occurs once
    }
}
