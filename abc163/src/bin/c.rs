use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{
        n: i32,
        a: [i32; n]
    }

    for i in 1..n {
        let num = a.iter().filter( |x| x==i).count();
        println!("{}", num);
    }
}