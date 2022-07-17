use std::future::Future;
use std::time::Duration;
use async_std::task::{block_on, sleep, spawn};

#[async_std::main]
async fn main() {
    // do3();
    // do4();

    // let do3_span=spawn(do3);
    // let do4_span=spawn(do4);
    //
    // do3_span.join().unwrap();
    // do4_span.join().unwrap();

    // let do3_async = spawn(do3());
    // do4().await;
    // do3_async.await;

    let result = block_on(go_study());
    println!("{}",result);
}

async fn lesson() ->String{
    String::from("从0到Go语言微服务架构师")
}

fn study1() -> impl Future<Output = String>{
    async {
        let x= lesson().await;
        x+" 学习目标"
    }
}

fn go_study() -> impl Future<Output=String>{
    let r = |x:String| async move {
        let y:String=study1().await;
        y+&*x
    };
    r(String::from(":全面掌握Go语言微服务落地，代码级一次性解决微服务和分布式系统。"))
}

//
// fn do3(){
//     for i in 1..=5{
//         println!("do3 {}",i);
//         sleep(Duration::from_millis(500));
//     }
// }
//
// fn do4(){
//     for i in 1..=5{
//         println!("do4 {}",i);
//         sleep(Duration::from_millis(1000));
//     }
// }


async fn do3(){
    for i in 1..=5{
        println!("do3 {}",i);
        sleep(Duration::from_millis(500)).await;
    }
}

async fn do4(){
    for i in 1..=5{
        println!("do4 {}",i);
        sleep(Duration::from_millis(1000)).await;
    }
}
