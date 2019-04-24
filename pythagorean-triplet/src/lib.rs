use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut list = HashSet::new();
    for x in 1..sum/3 {
        for y in x+1..sum/2 {
            let z = sum-x-y;
            if z.pow(2) == x.pow(2) + y.pow(2) {
                list.insert([x, y, z]);
            }
        }
    }
    return list
}
// https://exercism.io/my/solutions/e9dbe2604f7a462db1baed004822b0ad
