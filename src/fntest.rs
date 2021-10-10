fn get_func(n: i32) -> fn(i32) -> i32 {
    fn inc(n: i32) -> i32 {
        n + 1
    }

    fn dec(n: i32) -> i32 {
        n - 1
    }
    if n & 1 == 0 {
        inc
    } else {
        dec
    }
}

fn inc(n: i32) -> i32 {
    // 函数定义
    n + 1
}

fn dec(n: i32) -> i32 {
    n - 1
}

fn process(n: i32, func: IncType) -> i32 {
    func(n)
}

type IncType = fn(i32) -> i32; // 函数类型

fn int_string() {
    let my_str = "235".to_string();
    // let my_int = my_str.parse::<i32>().unwrap(); // 1
    let my_int: i32 = my_str.parse().unwrap(); // 2
    print!("{}", my_int * 2 + 1); //out
}

fn from_into() {
    println!("{}", i32::from(127i8));
    let i: i32 = 127i8.into();
    println!("{}", i);
}

// Error
// fn try_from_try_into () {
//     println!("{}", i8::try_from(32i32).unwrap());
//     // println!("{}",);
// }

fn as_type() {
    println!("{}", 127i32 as i8);
    println!("{}", 128i32 as i8);
    println!("{}", -100.533f64 as i32);
    for i in 1..10 {
        print!("{}", i / 2);
    }
    println!();

    for i in 1..10 {
        print!("{}", i as f64 / 2i32 as f64);
    }
    println!();

    for i in 1..10 {
        print!("{}", i as f64 / 2.0);
    }
}
