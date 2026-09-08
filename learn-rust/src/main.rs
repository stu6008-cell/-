fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("0으로 나눌 수 없음"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide(10, 2) {
        Ok(n)  => println!("결과: {}", n),
        Err(e) => println!("에러: {}", e),
    }

    match divide(10, 0) {
        Ok(n)  => println!("결과: {}", n),
        Err(e) => println!("에러: {}", e),
    }
}
