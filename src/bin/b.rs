#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        a: Chars,
        b: Chars,
        c: Chars,
    }
    let mut turn_a = 0;
    let mut turn_b = 0;
    let mut turn_c = 0;
    let mut next = 'a';
    let mut ans = 'a';
    for _ in 0..300 {
        match next {
            'a' => {
                if turn_a == a.len() {
                    ans = 'A';
                    break;
                }
                next = a[turn_a];
                turn_a += 1;
            }

            'b' => {
                if turn_b == b.len() {
                    ans = 'B';
                    break;
                }
                next = b[turn_b];
                turn_b += 1;
            }

            'c' => {
                if turn_c == c.len() {
                    ans = 'C';
                    break;
                }
                next = c[turn_c];
                turn_c += 1;
            }
            _ => {}
        }
    }
    println!("{}", ans);
}
