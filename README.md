# Medium Exercism Rust

Esai ini dibuat untuk memenuhi tugas mata kuliah sistem operasi di semester 110.

# Pythagorean Triplet

Di program ini kita diminta untuk mengecek apakah angka-angka yang diberikan bisa menunjukkan bahwa angka itu adalah sisi yang dapat membuat segitiga pytagoras.

# Solusi

Untuk menyelesaikan problem tersebut, kita bisa pakai rumus:
`x**2 + y**2 = z**2` dimana `x < y < z`. 

Contoh nya seperti ini : `5**2 + 12**2 = 25 + 144 = 169 = 13**2`.

Kemudian Diberikan integer input N untuk menemukan segitiga Pythagoras dengan `a + b + c = N`. Misalnya, dengan N = 240, tepat ada satu segitiga Pythagoras yang `a + b + c = 240: {60, 80, 100}`.

# Step

1. Yang pertama kita buat suatu library Hashset yang tujuan nya untuk membuat suatu array yang berisi 3 sisi segitiga tersebut. Kemudian buat variabel nya.
2. Lalu kita tentukan sisi pertama dengan membuat for loop dimana sisi pertama adalah yang terkecil
