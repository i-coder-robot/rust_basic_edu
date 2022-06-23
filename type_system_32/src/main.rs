fn main() {
    let spend = 1;
    /*
        mismatched types [E0308] expected `f64`, found `i32`
    */
    // let cost:f64=spend;

    let cost = spend as f64;
    println!("转换 {}->{}",spend,cost);

    let x = 1u8;
    let y = 2u32;
    let z=3f32;

    let i=1;
    let f = 1.0;

    let study = String::from("从0到Go语言微服务架构师");

    let mut vec=Vec::new();
    vec.push(study);
    println!("{:?}",vec);

    //别名，要用驼峰命名法CamelCase,别名不是新类型，不能提供额外的类型安全。

    let myU64:MyU64=5 as ThirdU64;
    let otherU64:OtherU64 = 2 as ThirdU64;
    println!("{} MyU64 + {} OtherU64 = {}",myU64,otherU64,myU64+otherU64);
}

type MyU64 = u64;
type OtherU64 = u64;
type ThirdU64=u64;
