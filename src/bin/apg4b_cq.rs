use proconio::input;

fn main() {
    input! {
        a: i32,
        op: String,
        b: i32,
    }

    let result = match op.as_str() {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        "/" => {
            if b == 0 { Err("error") } else { Ok(a / b) }
        },
        _ => Err("error"),
    };
    match result {
        Ok(result) => println!("{}", result),
        Err(err) => println!("{}", err),
    }
}
