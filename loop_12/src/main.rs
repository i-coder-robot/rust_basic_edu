fn main() {

    /*
        for 临时变量 in 左区间..右区间{
            //执行业务逻辑
        }
        左闭右开区间
    */
    for num in 1..5{
        println!("num is {}",num);
    }

    for num in 1..=5{
        println!("num is {}",num);
    }
    let studyList = vec![
        "《Go语言极简一本通》",
        "Go语言微服务架构核心22讲",
        "从0到Go语言微服务架构师",
    ];
    //iter()每次迭代是借用集合中的一个元素。元素本身不会改变，循环之后还可以使用。
    for name in studyList.iter(){
        match name {
            &"从0到Go语言微服务架构师"=>println!("恭喜你进阶到第三阶段-{}",name),
            _=>println!("上车:{}",name),
        }
    }
    let studyList2 = vec![
        "《Go语言极简一本通》",
        "Go语言微服务架构核心22讲",
        "从0到Go语言微服务架构师",
    ];
    //into_iter 会消耗集合，每次迭代，集合中的数据本身被提供，一旦集合被消耗完了。之后无法再使用，因为它已经在循环中被move了
    for name in studyList2.into_iter(){
        match name {
            "从0到Go语言微服务架构师"=>println!("恭喜你进阶到第三阶段-{}",name),
            _=>println!("上车:{}",name),
        }
    }

    let mut studyList3 = vec![
        "《Go语言极简一本通》",
        "Go语言微服务架构核心22讲",
        "从0到Go语言微服务架构师",
    ];
    for name in studyList3.iter_mut(){
        *name = match name {
            &mut "从0到Go语言微服务架构师"=>{"恭喜你进阶到第三阶段--从0到Go语言微服务架构师"},
            _=>*name,
        }
    }
    println!("studyList3:{:?}",studyList3);
    /*
        while (条件表达式) {
            执行业务代码
        }
    */
    let mut num = 1;
    while num<20{
        println!("num is {}",num);
        num=num*2;
    }

    let mut num=1;
    loop{
        if num>20{
            break;
        }
        println!("num is {}",num);
        num=num*3;
    }
}
