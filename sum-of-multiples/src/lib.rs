use std::collections::BTreeSet;

pub fn sum_of_multiples(limit: u64, factors: &[u64]) -> u64 {
    let mut factor_list: BTreeSet<u64> = BTreeSet::new();
    for &i in factors {
        let mut multiply = 2;
        let mut factor: u64 = i;
        while factor < limit {
            factor_list.insert(factor);
            factor = i * multiply;
            multiply += 1;
        }
    }
    return factor_list.iter().sum();
}
// https://exercism.io/my/solutions/6c5d98d067324c4d8f91c7c821fbf145
