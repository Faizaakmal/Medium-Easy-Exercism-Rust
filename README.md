# Medium Exercism Rust

Esai ini dibuat untuk memenuhi tugas mata kuliah sistem operasi di semester 110.

# Pythagorean Triplet

Di program ini kita diminta untuk mengecek apakah angka-angka yang diberikan bisa menunjukkan bahwa angka itu adalah sisi yang dapat membuat segitiga pytagoras.

# Solution

Untuk menyelesaikan problem tersebut, kita bisa pakai rumus:
`x**2 + y**2 = z**2` dimana `x < y < z`. 

Contoh nya seperti ini : `5**2 + 12**2 = 25 + 144 = 169 = 13**2`.

Kemudian Diberikan integer input N untuk menemukan segitiga Pythagoras dengan `a + b + c = N`. Misalnya, dengan N = 240, tepat ada satu segitiga Pythagoras yang `a + b + c = 240: {60, 80, 100}`.

# Step

1. Yang pertama kita buat suatu library Hashset yang tujuan nya untuk membuat suatu array yang berisi 3 sisi segitiga tersebut. Kemudian buat variabel nya.
2. Lalu kita tentukan sisi pertama dengan membuat for loop dimana sisi pertama adalah yang terkecil. Caranya adalah buat suatu variabel (x)  dalam range 1 sampai sebelum (sum/3), kenapa dibagi 3 karena segitiga mempunyai 3 sisi & untuk mendapatkan sisi terkecil nya kita bagi 3
3. Di dalam for loop sisi pertama kita buat lagi for loop untuk sisi kedua dari segitiga, dengan cara buat lagi variabel nya (y) dalam range (x+1) sampai sebelum (sum/2).
4. Setelah itu kita buat variabel (z) yang sama dengan jumlahnya dikurangi 2 sisi sebelumnya
5. Buat if condition untuk menentukan segitiga pytagoras tersebut dengan menyamakan kuadrat dari sisi terbesar segitiga sama dengan kuadrat sisi pertama & kuadrat sisi kedua seperti solusi di atas.
6. jika benar maka buat pernyataan untuk memasukkan ketiga sisi segitiga tersebut ke dalam array.
7. Yang terakhir panggil array nya untuk menunjukkan sisi-sisi pytagoras

# Full Code

```rust
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
```
