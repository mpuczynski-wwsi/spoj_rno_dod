/**
https://pl.spoj.com/problems/RNO_DOD/

RNO_DOD - Proste dodawanie
Twoim zadaniem jest dodać wszystkie liczby całkowite podane na wejściu.

Wejście
W pierwszym wierszu znajduje się liczba t testów (0 < t < 100) Każdy test opisany jest w następujący sposób. 
W pierwszym wierszu dana jest liczba n - liczba liczb do zsumowania. Następnie podanych jest n liczb pooddzielanych spacją.

Przykład
Input:
2
5
1 2 3 4 5
2
-100 100

Output:
15
0
*/
 
use std::io::BufRead;


fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let n:u8 = lines.next().unwrap().unwrap().parse().unwrap();
    for i in 0..n {
        let op_num:u16 = lines.next().unwrap().unwrap().parse().unwrap();
        let l = lines.next().unwrap().unwrap();
        let s:i32 = l.split(' ').into_iter().map(|x| -> i32 {x.parse().unwrap()}).sum();

        println!("{:?}", s);

    }
}
