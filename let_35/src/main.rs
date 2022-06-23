fn main() {
    let s = Some("从0到Go语言微服务架构师");
    let s1:Option<i32>=None;
    let s2:Option<i32>=None;

    //如果let将s解构成Some(i),则执行语句块{}
    if let Some(i) = s{
        println!("已上车{:?}",i);
    }

    //如果解构失败，就执行else
    if let Some(i)=s1{
        println!("Matched {:?}",i)
    }else{
        println!("不匹配")
    }

    let flag=false;
    if let Some(i)=s2{
        println!("Matched{:?}",i);
    }else if flag{
        println!("不匹配s2")
    }else{
        println!("默认分支")
    }

    let mut num=Some(0);

    while let Some(i)=num{
        if i>9{
            println!("{},quit!",i);
            num=None;
        }else{
            println!("i is {:?} Try again",i);
            num=Some(i+1);
        }
    }
}
