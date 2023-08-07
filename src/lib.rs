use std::collections::BTreeMap;
use std::iter::Peekable;

pub struct IndexedNumber {
    pub index: u64,
    pub value: u64,
}

// TODO docs
struct SortedTunnel<T: Ord> {
    sorted_data: BTreeMap<T, usize>,
}

impl<T: Ord> SortedTunnel<T> {
    pub fn new(tunnel: impl Iterator<Item = T>) -> SortedTunnel<T> {
        let tunnel = tunnel.enumerate().map(|(index, value)| (value, index));
        SortedTunnel {
            sorted_data: BTreeMap::from_iter(tunnel),
        }
    }

    fn remove_by_value(&mut self, target: usize) {
        for (key, value) in self.sorted_data.iter() {
            if *value == target {
                self.sorted_data.remove(key);
                return;
            }
        }
    }

    pub fn shift_right(&mut self, new_key: T) {
        self.remove_by_value(0);
        for (key, value) in self.sorted_data.iter_mut() {
            *value -= 1;
        }
        self.sorted_data.insert(new_key, self.sorted_data.len());
    }

    pub fn is_tunnel_safe(&self, number: T) -> bool {
        todo!()
    }
}

pub fn get_critical_number(numbers: impl Iterator<Item = u64>) -> Option<IndexedNumber> {
    let tunnel = numbers.take(100);
    let mut numbers = numbers.peekable();
    if numbers.peek() != None {
        return None;
    }

    todo!();
    let sorted_tunnel = SortedTunnel::new;
}
