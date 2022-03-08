fn main() {
    println!("Hello, world!");

    let a = 1;
    let mut b = 1;
    // a = 2; mut 가 없으면 변경 불가능 !!
    b = 2;
    const TEST_C: i32 = 1;
    //const 는 타입 지정이 필요함!!
    //그나저나 어떻게 타입이 i32...
    // c = 2; const 도 변경 불가능

    println!();
    println!("{}", a);
    println!("{}", b);
    println!("{}", TEST_C);
}
