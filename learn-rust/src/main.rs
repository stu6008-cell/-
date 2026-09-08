fn main() {
    let mut s = String::from("hi");
    let w1 = &mut s;
    let w2 = &mut s;      // ❌ 두 번째 쓰기 빌림
    println!("{} {}", w1, w2);
}
