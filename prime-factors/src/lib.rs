pub fn factors(n: u64) -> Vec<u64> {
    let mut kosong = vec![];

    if n <= 1 {
        return kosong;
    }
    for divider in 2..n+1{
        if prima(divider) && n % divider == 0{
			kosong.push(divider);
			let sisa = n / divider;
			let mut factors: Vec<u64> = factors(sisa);
			kosong.append(&mut factors);
			break;
		}
    }
    return kosong;
}

fn prima(n: u64) -> bool {
	for i in 2..n {
		if n % i == 0 {
			return false;
		}
	}
	return n != 1;
}
// https://exercism.io/my/solutions/fa93c65f92c740d89c342ab6031fe577
