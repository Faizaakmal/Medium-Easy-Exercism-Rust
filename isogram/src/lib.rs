pub fn check(candidate: &str) -> bool {
    let mut list = Vec::new();
    let mut isogram = true;
    let mut not_isogram = false;
    let mut length = candidate.chars().count(); //sama aja candidate.len() fungsinya buat ngitung panjang string
    let mut word = candidate.to_lowercase();
    let mut words = word.chars();
    if length == 0 {
        isogram;
    }
    for letter in words {
        if list.contains(&letter) { //kalo dalam kata tersebut terdapat huruf yang sama berarti bukan isogram
            return not_isogram;
        }
        if letter.is_alphabetic() {
            list.push(letter);
        }
    }
    isogram
}
// https://exercism.io/my/solutions/f90beaad449c4e868757a40e4b3cb671
