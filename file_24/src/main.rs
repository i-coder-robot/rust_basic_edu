use std::fs;
use std::fs::OpenOptions;
use std::io::{Read, Write};

fn main() {
    // let file=std::fs::File::open("data.txt");
    // println!("文件打开\n{:?}",file);
    //
    // let file = std::fs::File::create("data2.txt").expect("创建失败");
    // println!("文件创建成功{:?}",file);

    // fs::remove_file("data.txt").expect("无法删除文件");
    // println!("文件已删除");

    // let mut file=OpenOptions::new().append(true).open("data2.txt").expect("失败");
    // file.write("\nwww.go-edu.cn".as_bytes()).expect("写入失败");
    // println!("\n数据追加成功");

    // file.write_all("Rust".as_bytes()).expect("失败");
    // file.write_all("\nRust".as_bytes()).expect("失败");
    // println!("\n数据写入成功");
    //write_all并不会在写入后自动写入换行\n

    let mut file = std::fs::File::open("data2.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}",contents)
}
