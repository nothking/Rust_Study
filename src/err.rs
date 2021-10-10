
pub fn err_test() {
    let f = get_funcc();
    println!("{}",f(3));
    println!("where is the c");
}

fn get_funcc() -> fn(i32)-> i32 {
    let a = 1;
    fn inc(n:i32) -> i32 {
        // n + a      is err
        n + 1
    }
    inc
}