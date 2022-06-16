fn main() {
    let v = vec!["Go语言极简一本通","Go语言微服务架构核心22讲","从0到Go语言微服务架构师"];
    let mut it = v.iter();
    println!("{:?}",it.next());
    println!("{:?}",it.next());
    println!("{:?}",it.next());
    println!("{:?}",it.next());

    let iter=v.iter();
    for item in iter{
        println!("{}\n",item);
    }
    /*
        iter() 返回一个只读可冲入迭代器，迭代器元素的类型为&T,
        into_iter() 返回一个只读不可重入迭代器，迭代器元素的类型为T,
        iter_mut() 返回一个可修改重入迭代器，迭代器元素的类型为&mut T
    */
}
