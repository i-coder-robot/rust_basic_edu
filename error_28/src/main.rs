use std::fs::File;

fn main() {
    // panic!("出错了");
    // println!("Hello Rust");

    //panic 程序立即退出，退出的时候调用者抛出退出原因。

    let v = vec!["Go语言极简一本通","Go语言微服务架构核心22讲","从0到Go语言微服务架构师"];
    // v[5];

    // let f = File::open("abc.jpg");
    // println!("{:?}",f);

    //unwrap expect
    let result = is_even(6).unwrap();
    println!("结果{}",result);

    let result = is_even(11).unwrap();
    println!("结果{}",result);

    //unwrap是Result<T,E>的方法，实例上调用此方法时，如果是Ok,就返回Ok中的对象，
    // 如果是Err枚举，在运行时会panic

    //expect区别是 expect方法接收msg:&str作为参数，可以自定义报错信息。
}

fn is_even(no :i32)->Result<bool,String>{
    return  if no%2==0{
        Ok(true)
    }else{
        Err("输入值，不是偶数".to_string())
    }
}
