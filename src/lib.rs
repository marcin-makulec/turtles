//! # turtles
//! This library is a set of utilities created for Number Associative Mining and Trading Inc
//! for finding the point where the tunnel may collapse on their turtles 🐢

use std::{
    fmt::Debug,
    ops::{Add, Mul},
};
use tunnel_utils::SortedTunnel;

/// Holds information on what value would (`step`) cause the tunnel to collapse
/// and at which step/on which line (`index`) it would happen
#[derive(Debug, PartialEq)]
pub struct IndexedStep<T>
where
    T: Ord + Add<Output = T>,
{
    pub step: T,
    pub index: usize,
}

/// Consumes `steps` until it finds a step at which tunnel would collapse.
/// Tunnel collapses if the next step cannot be represented as a sum of 2 from `tunnel_len` preceding elements.
/// Returns `None` if the tunnel is safe (including the case when `tunnel_len` is bigger or equal to `steps` length).
///
/// # Examples
///
/// ```
/// use turtles::get_critical_number;
/// use turtles::IndexedStep;
///
/// let steps = vec![5, 4, 7, 9, 14].into_iter();
/// let tunnel_len = 3;
/// let answer = get_critical_number(steps, tunnel_len);
///
/// assert_eq!(answer, Some(IndexedStep {step: 14, index: 4}));
///
///
/// let steps = vec![5, 4, 9].into_iter();
/// let tunnel_len = 2;
/// let answer = get_critical_number(steps, tunnel_len);
///
/// assert_eq!(answer, None);
///
///
/// let steps = vec![5, 4, 18].into_iter();
/// let tunnel_len = 3;
/// let answer = get_critical_number(steps, tunnel_len);
///
/// assert_eq!(answer, None);
/// ```
pub fn get_critical_number<T: Ord + Add<Output = T> + Copy + Debug + Mul<u128, Output = T>>(
    mut steps_in_tunnel: impl Iterator<Item = T>,
    tunnel_len: usize,
) -> Option<IndexedStep<T>> {
    // first tunnel_len steps are removed from the iterator
    let tunnel = steps_in_tunnel.by_ref().take(tunnel_len).collect();
    let mut steps_in_tunnel = steps_in_tunnel.peekable();

    // if the iterator is empty, the tunnel is safe
    if steps_in_tunnel.peek() == None {
        return None;
    }

    let mut sorted_tunnel = SortedTunnel::new(tunnel);

    for (index, step) in steps_in_tunnel.enumerate() {
        if !sorted_tunnel.is_tunnel_safe(step) {
            // we need to add tunnel_len, as the for loop starts from this offset
            return Some(IndexedStep {
                index: index + tunnel_len,
                step,
            });
        }
        sorted_tunnel.shift_right(step);
    }

    None
}

/// Low level utilities for examining tunnels for turtles
mod tunnel_utils {
    use std::cmp::Ordering;
    use std::collections::{BTreeMap, VecDeque};
    use std::fmt::Debug;
    use std::ops::{Add, Mul};

    /// This trait allows us to handle inserting duplicate keys to the BTreeMap.
    trait AddDuplicate<T>
    where
        T: Ord + Add<Output = T> + Copy,
    {
        fn add_duplicate(&mut self, step: T, age: usize);
    }

    /// Holds information about preceding fragment of the tunnel.
    ///
    /// `BTreeMap` is used because it's sorted and because of its fast lookup times.
    /// It cannot store duplicate keys, so the underlying queue represents how many keys are present.
    /// `usize` values represent their order in the preceding tunnel fragment.
    ///
    /// This is efficient because checking sums of elements which are smaller than our target is the majority
    /// of operations conducted in the process. Also, we will only remove elements from the start of the queue
    /// and append at its end, so `VecDeque` is also appropriate.
    pub struct SortedTunnel<T>
    where
        T: Ord + Add<Output = T> + Copy,
    {
        tunnel_map: BTreeMap<T, VecDeque<usize>>,
        tunnel_length: usize,
    }

    impl<T> AddDuplicate<T> for SortedTunnel<T>
    where
        T: Ord + Add<Output = T> + Copy,
    {
        fn add_duplicate(&mut self, step: T, age: usize) {
            self.tunnel_map
                .entry(step)
                .and_modify(|ages| ages.push_back(age))
                .or_insert(VecDeque::from([age]));
        }
    }

    impl<T: Ord + Add<Output = T> + Copy + Debug + Mul<u128, Output = T>> SortedTunnel<T> {
        pub fn new(tunnel: Vec<T>) -> SortedTunnel<T> {
            let mut sorted_tunnel = SortedTunnel {
                tunnel_map: BTreeMap::new(),
                tunnel_length: tunnel.len() - 1,
            };
            for (age, step) in tunnel.iter().enumerate() {
                sorted_tunnel.add_duplicate(*step, age);
            }
            sorted_tunnel
        }

        /// Removes the oldest step in preceding fragment.
        fn remove_oldest_step(&mut self) {
            let mut target_step: Option<T> = None;
            for (step, ages) in self.tunnel_map.iter() {
                if ages.contains(&0) {
                    target_step = Some(*step);
                    break;
                }
            }
            if let Some(step) = target_step {
                let ages_ref = self.tunnel_map.get_mut(&step).unwrap();
                ages_ref.pop_front();
                if ages_ref.len() == 0 {
                    self.tunnel_map.remove(&step);
                }

                self.tunnel_map
                    .values_mut()
                    .for_each(|ages| ages.iter_mut().for_each(|age| *age -= 1));
                return;
            }
            panic!("There was no oldest step in SortedTunnel before removal");
        }

        /// Replaces the oldest step from preceding tunnel fragment with a new step.
        pub fn shift_right(&mut self, new_step: T) {
            self.remove_oldest_step();
            self.add_duplicate(new_step, self.tunnel_length);
        }

        /// Checks if the tunnel won't collapse after the next step.
        pub fn is_tunnel_safe(&self, new_step: T) -> bool {
            'outer: for (i, (candidate_a, ages_a)) in self.tunnel_map.iter().enumerate() {
                if candidate_a >= &new_step {
                    return false;
                }

                if ages_a.len() > 1 && *candidate_a * 2 == new_step {
                    return true;
                }

                for (candidate_b, _) in self.tunnel_map.iter().skip(i + 1) {
                    match (*candidate_a + *candidate_b).cmp(&new_step) {
                        Ordering::Equal => return true,
                        Ordering::Greater => continue 'outer,
                        _ => continue,
                    }
                }
            }
            false
        }
    }
}
