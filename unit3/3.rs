fn main() {
    let a = 5;
    let b = 7;
    let operator = '*';
    match operator {
        '+' => {
            println!("{}", a+b);
        },
        '-' => {
            println!("{}", a-b);
        },
        '*' => {
            println!("{}", a*b);
        },
        '/' => {
            if b == 0 {
                println!("Division by 0 is undefined");
            }
            else {
                println!("{}", a/b);
            }
        },
        '%' => {
            println!("{}", a%b);
        }
        _ => {
            println!("invalid operator");
        }
    }
}
