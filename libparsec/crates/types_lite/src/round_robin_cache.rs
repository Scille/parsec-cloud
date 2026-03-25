// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#[derive(Debug)]
pub struct RoundRobinCache<K: PartialEq, V, const L: usize> {
    items: [Option<(K, V)>; L],
    round_robin: usize,
}

impl<K: PartialEq, V, const L: usize> Default for RoundRobinCache<K, V, L> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: PartialEq, V, const L: usize> RoundRobinCache<K, V, L> {
    pub fn new() -> Self {
        Self {
            items: [const { None }; L],
            round_robin: 0,
        }
    }

    pub fn push(&mut self, key: K, value: V) {
        self.items[self.round_robin] = Some((key, value));
        self.round_robin = (self.round_robin + 1) % self.items.len();
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        // We iterate in reverse insertion order, considering the more recent
        // a block is, the more likely it is to be accessed again.
        self.items[..self.round_robin]
            .iter()
            .rev()
            .chain(self.items[self.round_robin..].iter().rev())
            .find_map(|item| {
                item.as_ref()
                    .filter(|(candidate_key, _)| candidate_key == key)
            })
            .map(|(_, value)| value)
    }
}
