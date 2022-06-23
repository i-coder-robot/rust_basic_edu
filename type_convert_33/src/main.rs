fn main() {
    let s1 = "从0到Go语言微服务架构师";
    let s2=String::from(s1);

    let my_number = MyNumber::from(1);
    println!("{:?}",my_number);

    let spend=3;
    let my_spend:MyNumber = spend.into();
    println!("{:?}",my_spend);

    let cost:i32 = "5".parse().unwrap();
    println!("{}",cost);
}
#[derive(Debug)]
struct MyNumber{
    num:i32,
}

impl From<i32> for MyNumber{
    fn from(item: i32) -> Self {
        MyNumber{ num:item}
    }
}
