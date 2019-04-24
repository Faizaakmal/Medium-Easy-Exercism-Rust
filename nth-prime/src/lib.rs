pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2
    }
    let mut prima = 2;
    let mut sum = 0;

    while sum != n {
        prima += 1;
        if prime(prima) {
            sum += 1;
        }
    }
    prima
}
fn prime(angka: u32) -> bool {
    let mut bilprima = true;
	for pembagi in 2..angka {
		if angka % pembagi == 0 {
			return false;
		}
	}
	bilprima
}