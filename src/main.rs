use clap::Parser;

#[derive(Debug,Parser)]
struct CLI {
    terms: i64,
}



fn work_pi(terms :i64) -> f64 {
    let mut sum: f64 = 0.0;
    let mut sign: f64 = 1.0;
    let mut i: f64 = 1.0;
    for _ in 0..terms  {
        sum = sum + sign * 1.0 / (2.0 * i - 1.0);
        sign = -sign;
        i += 1.0;

        

    }
    sum * 4.0
}
fn main() {
    let cli = CLI::parse();

    let result: f64 = work_pi(cli.terms);
    println!("计算出的圆周率为:{}", result);
}