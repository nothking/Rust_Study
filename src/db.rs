use mysql::*;
use mysql::prelude::*;

pub fn db_test(){
    //设置连接字符串
    let url = "mysql://root:123456@localhost:3306/test";
    //创建连接池
    let pool = Pool::new(url).unwrap();
    //连接数据库
    let mut conn = pool.get_conn().unwrap();

    //数据库操作
    //1.查询user表
    //方式1 ： 流式查询 数据逐行读取，数据不会存储在内存中
    conn.query_iter("select id , name , age from user ").unwrap()
    .for_each(|row|{
        let r:(i32,String,i32) = from_row(row.unwrap());
        println!("id={},name={},age={}",r.0,r.1,r.2);
    });
    println!("--------------------------------------------");
    //方式2 ： 将数据集取出存储在Vec中
    let res : Vec<(i32,String,i32)> = conn.query("select id , name , age from user ").unwrap();
    for r in res {
        println!("id={},name={},age={}",r.0,r.1,r.2);
    }
    println!("--------------------------------------------");
    //方式3 ： 将数据转化成Struct 
    struct User {
        id : i32,
        name : String ,
        age : i32
    }

    let res = conn.query_map("select id , name , age from user ", |(id,name,age)|User {
        id,
        name,
        age
    }).expect("QUERY ERROR");

    for user in res {
        println!("id={},name={},age={}",user.id,user.name,user.age);
    }
    println!("{:?}",time::now());
}