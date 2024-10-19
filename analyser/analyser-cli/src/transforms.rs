use crate::occurance::{Countable, OccuranceAnalysis, OccuranceT, Occurances};

impl<T: OccuranceT> Occurances<T> {
    pub fn strip(&mut self, replace: bool, check: impl Fn(&Countable) -> (bool, Countable)) {
        let keys: Vec<Countable> = self.keys().cloned().collect();
        for key in keys {
            let (strip, new_key) = check(&key);
            if strip {
                let value = self.swap_remove(&key).unwrap();
                if replace {
                    self.entry(new_key)
                        .and_modify(|v| *v += value)
                        .or_insert(value);
                }
            }
        }
    }
}

impl<T: OccuranceT> OccuranceAnalysis<T> {
    pub fn strip(&mut self, check: impl Fn(&Countable) -> (bool, Countable)) {
        // For ngrams and skipgrams we just strip invalid entries
        self.ngrams
            .values_mut()
            .chain(self.skipgrams.values_mut())
            .for_each(|occurance| {
                occurance.strip(false, &check);
            });

        // For words we strip invalid entries and replace them with the stripped version
        self.words.strip(true, &check);
    }

    pub fn transform(&mut self, spec: &TransformSpecification) {
        if spec.strip_whitespace {
            self.strip(check_whitespace);
        }

        if spec.strip_punctuation {
            self.strip(check_punctuation);
        }

        if spec.strip_numbers {
            self.strip(check_numeric);
        }

        if spec.strip_nonlatin {
            self.strip(check_nonlatin);
        }
    }
}

pub fn check_whitespace(x: &Countable) -> (bool, Countable) {
    let stripped: Countable = x.chars().filter(|c| !c.is_whitespace()).collect();
    (*x != stripped, stripped.into())
}

pub fn check_punctuation(x: &Countable) -> (bool, Countable) {
    let stripped: Countable = x.chars().filter(|c| !c.is_ascii_punctuation()).collect();
    (*x != stripped, stripped.into())
}

pub fn check_numeric(x: &Countable) -> (bool, Countable) {
    let stripped: Countable = x.chars().filter(|c| !c.is_numeric()).collect();
    (*x != stripped, stripped.into())
}

pub fn check_nonlatin(x: &Countable) -> (bool, Countable) {
    let stripped: Countable = x.chars().filter(|c| c.is_ascii_alphanumeric()).collect();
    (*x != stripped, stripped.into())
}

pub struct TransformSpecification {
    pub strip_whitespace: bool,
    pub strip_punctuation: bool,
    pub strip_numbers: bool,
    pub strip_nonlatin: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::occurance::Occurances;

    #[test]
    fn test_strip() {
        let mut occurances_replace: Occurances<usize> = vec![("a ".into(), 2), ("a".into(), 1)]
            .into_iter()
            .collect();
        let mut occurances_strip: Occurances<usize> = vec![("a ".into(), 2), ("a".into(), 1)]
            .into_iter()
            .collect();

        occurances_replace.strip(true, &check_whitespace);
        occurances_strip.strip(false, &check_whitespace);

        assert_eq!(occurances_replace.get(&"a".into()), Some(&3));
        assert_eq!(occurances_strip.get(&"a".into()), Some(&1));
    }
}
