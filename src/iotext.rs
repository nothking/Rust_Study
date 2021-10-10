use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;

// #[test]
pub fn io_text() {
    // let file = std::fs::File::create("data.txt").expect("create failed");
    // println!("文件创建成功:{:?}", file);

    // let file = std::fs::File::open("data.txt").unwrap();
    // println!("文件打开成功：{:?}",file);

    // let mut f = File::open("data.txt").unwrap();
    // let mut buf = vec![0; 8];
    // let n = f.read(&mut buf[..]).unwrap();
    // println!("{:?}", &buf[..n]);   //[104, 101, 108, 108, 111, 32, 119, 111]

    // let mut f = File::open("data.txt").unwrap();
    // let mut buf = String::new();
    // f.read_to_string(&mut buf).unwrap();
    // println!("{}", buf);    //hello world
    //                         //hello world !!!!!

    let f = File::open("data.txt").unwrap();
    let reader = BufReader::new(f);
    for line in reader.lines() {
        // line 是 std::result::Result<std::string::String, std::io::Error> 类型
        // line 不包含换行符
        let line = line.unwrap();
        println!("{}", line);   //hello world
                                //hello world !!!!!
    }

    //############ write #################
    // let mut f = File::create("test.text").unwrap();
    // f.write("Hello\n".as_bytes()).unwrap();
    // f.write("12233445556666\n".as_bytes()).unwrap();
    // ######### write append   ################
    // let mut f = OpenOptions::new()
    //     .create(true)
    //     .append(true)
    //     .open("test.text")
    //     .unwrap();
    // f.write("\n使用中文对很对非常对输入的\n".as_bytes()).unwrap();
    // f.write("345667777\n".as_bytes()).unwrap();
}
