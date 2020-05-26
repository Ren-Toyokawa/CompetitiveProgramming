use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{
        n: i32, m: i32,
        a: [i32; m]
    }

    let days_to_do_homework: i32 = a.iter().sum();
    let days_to_play: i32 = n - days_to_do_homework;

    if days_to_play < 0 {
        println!("-1");
    }else{
        println!("{}", days_to_play);
    }
}
