use std::ops::Add;
use tunnel_utils::SortedTunnel;

pub struct IndexedStep<T>
where
    T: Ord + Add<Output = T>,
{
    pub index: usize,
    pub step: T,
}

pub fn get_critical_number<T: Ord + Add<Output = T> + Copy>(
    steps: &mut impl Iterator<Item = T>,
    tunnel_len: usize,
) -> Option<IndexedStep<T>> {
    let tunnel = steps.by_ref().take(tunnel_len).collect();
    let mut steps = steps.peekable();

    if steps.peek() == None {
        return None;
    }

    let mut sorted_tunnel = SortedTunnel::new(tunnel);

    for (index, step) in steps.enumerate() {
        if !sorted_tunnel.is_tunnel_safe(step) {
            return Some(IndexedStep {
                index: index + tunnel_len,
                step,
            });
        }
        sorted_tunnel.shift_right(step);
    }

    None
}

// TODO docs
mod tunnel_utils {
    use std::cmp::Ordering;
    use std::collections::{BTreeMap, VecDeque};
    use std::ops::Add;

    trait AddDuplicate<T>
    where
        T: Ord + Add<Output = T> + Copy,
    {
        fn add_duplicate(&mut self, step: T, age: usize);
    }

    type TunnelMap<T> = BTreeMap<T, VecDeque<usize>>;

    impl<T> AddDuplicate<T> for TunnelMap<T>
    where
        T: Ord + Add<Output = T> + Copy,
    {
        fn add_duplicate(&mut self, step: T, age: usize) {
            self.entry(step)
                .and_modify(|ages| ages.push_back(age))
                .or_insert(VecDeque::from([age]));
        }
    }

    pub struct SortedTunnel<T>
    where
        T: Ord + Add<Output = T> + Copy,
    {
        tunnel_map: TunnelMap<T>,
        tunnel_length: usize,
    }

    impl<T: Ord + Add<Output = T> + Copy> SortedTunnel<T> {
        pub fn new(tunnel: Vec<T>) -> SortedTunnel<T> {
            let mut tunnel_map: TunnelMap<T> = BTreeMap::new();
            for (age, step) in tunnel.iter().enumerate() {
                tunnel_map.add_duplicate(*step, age);
            }
            SortedTunnel {
                tunnel_map,
                tunnel_length: tunnel.len() - 1,
            }
        }

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
            panic!("There was no oldest key in SortedTunnel before removal");
        }

        pub fn shift_right(&mut self, new_key: T) {
            self.remove_oldest_step();
            self.tunnel_map.add_duplicate(new_key, self.tunnel_length);
        }

        pub fn is_tunnel_safe(&self, step: T) -> bool {
            'outer: for (i, (candidate_a, _)) in self.tunnel_map.iter().enumerate() {
                if candidate_a >= &step {
                    return false;
                }

                for (candidate_b, _) in self.tunnel_map.iter().skip(i) {
                    match (*candidate_a + *candidate_b).cmp(&step) {
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
