pub fn bd_test() {
    let a1 = 5;
    let a2: i32 = 5;
    assert_eq!(a1, a2);
    let b1: u32 = 5;
    // assert_eq!(a1,b1);
    //上面的会报错   类型不匹配
    //    error[E0308]: mismatched types
}

pub fn some_value() {
    //boolean type
    let t = true;
    let f: bool = false;
    println!("{},{}",t,f);
    //char type
    let c = 'c';
    println!("{}",c);
    //numeric types
    let x = 42;
    let y: u32 = 123_456;
    let z: f64 = 1.23e+2;
    println!("{},{},{}",x,y,z);
    let zero:f64 = z.abs_sub(123.4);
    let bin = 0b1111_0000;
    let oct = 0o7320_1546;
    let hex: i64 = 0xf23a_b049;
    println!("{},{},{},{}",zero,bin,oct,hex);
    //string types
    let str = "Hello world!";
    let mut string = str.to_string();
    println!("{},{}",str,string);
    //arrays and slices
    let a = [1, 3, 4, 56, 6, 2];
    let middle = &a[1..4];
    let mut ten_zero: [i64; 10] = [0; 10];

    //tuples
    let tuple: (i32, &str) = (50, "hello");
    let (fifty, _) = tuple;
    let hello = tuple.1;

    //raw pointers
    let x = 5;
    let raw = &x ;// as *const i32;
    let points_at = unsafe { *raw };
    unsafe {
        println!("{}，{}，{}",x,* raw,points_at);
    }
    
    //functions
    fn foo(x: i32) -> i32 {
        x * x
    }
    let bar: fn(i32) -> i32 = foo;  
    println!("{}",bar(5));

    //explicit conversion 
    let decimal = 65.4321_f32;
    let integer = decimal as u8;
    let character = integer as char ;

    //type aliases
    type NanoSecond = u64;
    type Point = (u8,u8);


}
