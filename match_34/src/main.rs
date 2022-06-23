fn main() {
    /*
        解构 &、ref、ref mut
        解引用 *
    */

    let num= &100;

    match num {
        &val=> println!("&val 是 {:?}",val),
    }

    match *num{
        val => println!("val 是 {:?}",val),
    }
    //ref他改变了赋值的行为，可以对具体值创建引用。
    let ref num3 = 66;

    //定义2个非引用变量，通过ref和ref mut仍然可以得到其引用。
    let num4 = 5;
    let mut mut_num4=7;

    match num4 {
        ref r => println!("num4 是 {:?}",*r),
    }

    match mut_num4{
        ref mut m =>{
            *m+=10;
            println!("mut_num4 是 {:?}",m);
        }
    }

    let s =Study{
        name:String::from("从0到Go语言微服务架构师"),
        target:String::from("全面掌握Go语言微服务落地，代码级一次性解决微服务和分布式系统。"),
        spend:3,
    };

    let Study{
        name:name,
        target:target,
        spend:spend
    }=s;

    println!("name={},target={},spend={}",name,target,spend);

    let s2 =Study{
        name:String::from("从0到Go语言微服务架构师"),
        target:String::from("全面掌握Go语言微服务落地，代码级一次性解决微服务和分布式系统。"),
        spend:5,
    };
    let Study{name,..}=s2;
    println!("name = {}",name);
}
struct Study{
    name:String,
    target:String,
    spend:u32,
}
