use std::io;

fn workPi(terms: f64) -> f64 {
    let mut sum: f64 = 0.0;
    let mut sign: f64 = 1.0;
    let mut i: f64 = 1.0;
    for _ in 0..terms as i64 {
        sum = sum + sign * 1.0 / (2.0 * i - 1.0);
        sign = -sign;
        i += 1.0;
    }
    sum * 4.0
}

fn main() {
    let mut terms_string = String::new();

    println!("欢迎使用圆周率计算器");
    
    println!("请输入迭代次数:");
    io::stdin().read_line(&mut terms_string).expect("错误");

    let terms: f64 = match terms_string.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("输入无效，请输入数字");
            return;
        }
    };

    let result: f64 = workPi(terms);
    println!("计算出的圆周率为:{}", result);
}
