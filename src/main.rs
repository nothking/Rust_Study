// mod err;
// mod start;
// mod db;
// mod iotext;
mod postgre_test;

// use std::convert::From;
// use std::convert::Into;

fn main() {
    
    postgre_test::db_test();
    // iotext::io_text();

    // db::db_test();

    /*
    //test git 
    start::bd_test();
    start::some_value();
    println!("--start--------------------",);
    err::err_test();
    println!("---err------------------");
    as_type();
    println!();
    from_into();
    int_string();
    println!("");
    let func: IncType = inc;
    println!("3 + 1 = {}", func(3));
    println!("3 + 1 = {}", inc(3));
    println!("3 + 1 = {}", process(3, inc));
    println!("3 - 1 = {}", process(3, dec));
    println!("---self-------------------");
    let a = [1, 2, 3, 4, 5, 6, 7];
    let mut b = Vec::<i32>::new();

    for i in &a {
        println!("{}", *i);
        println!("func : \t{}", get_func(*i)(*i)); // first is n ,second is used by behind fn
        b.push(get_func(*i)(*i))
    }
    println!("{:?}", b);

    println!("{}", get_func(99999)(6));

    */
}
