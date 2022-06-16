use std::collections::{HashMap, HashSet};

fn main() {
    /*
        向量vector
        特点
        1 相同类型的元素集合。
        2 长度可变，运行时增加减少都可以。
        3 使用索引查找。
        4 添加元素到队尾。
        5 向量内存在堆上，长度可动态变化。

        创建向量
        Vec::new();
        vec![val1,val2,...]
    */

    let mut v = Vec::new();
    v.push("Go语言极简一本通");
    v.push("Go语言微服务架构核心22讲");
    v.push("从0到Go语言微服务架构师");
    println!("{:?}",v);
    println!("len:{}",v.len());

    let mut v2=vec!["Go语言极简一本通","Go语言微服务架构核心22讲","从0到Go语言微服务架构师"];
    println!("{:?}",v2);

    let x = v2.remove(0);
    println!("{}",x);
    println!("{:?}",v2);

    if v.contains(&"从0到Go语言微服务架构师"){
        println!("找到了");
    }
    let y = v[0];
    println!("{}",y);

    for item in v{
        println!("充电项目:{}",item);
    }

    /*
        HashMap 键值对的集合，键不能重复的，值可以重复。 显示导入std::collections

        let mut 变量名称=HashMap::new();
    */

    let mut proccess = HashMap::new();

    proccess.insert("Go语言极简一本通",1);
    proccess.insert("Go语言微服务架构核心22讲",2);
    proccess.insert("从0到Go语言微服务架构师",3);

    println!("{:?}",proccess);
    println!("len {}",proccess.len());

    match proccess.get(&"从0到Go语言微服务架构师"){
        Some(v)=>{
            println!("HashMap v:{}",v)
        }
        None=>{
            println!("找不到");
        }
    }

    for (k,v) in proccess.iter(){
        println!("k:{} v:{}",k,v);
    }

    if proccess.contains_key(&"Go语言极简一本通"){
        println!("找到了");
    }

    let x = proccess.remove(&"Go语言极简一本通");
    println!("{:?}",x);
    println!("{:?}",proccess);

    /*
        HashSet 相同数据类型的集合，没有重复的值。如果存在相同的值，插入会失败。
        let mut 变量名称=HashSet::new();
    */

    let mut studySet = HashSet::new();
    studySet.insert("Go语言极简一本通");
    studySet.insert("Go语言微服务架构核心22讲");
    studySet.insert("从0到Go语言微服务架构师");

    println!("{:?}",studySet);

    studySet.insert("从0到Go语言微服务架构师");
    println!("{:?}",studySet);
    println!("len: {}",studySet.len());

    for item in studySet.iter(){
        println!("hashSet-充电项目:{}",item);
    }

    match studySet.get("从0到Go语言微服务架构师"){
        None=>{
            println!("找不到")
        }
        Some(data)=>{
            println!("studySet中找到:{}",data);
        }
    }

    if studySet.contains("Go语言微服务架构核心22讲"){
        println!("包含 Go语言微服务架构核心22讲");
    }

    studySet.remove("Go语言极简一本通");
    println!("{:?}",studySet);

    studySet.remove("golang");
    println!("{:?}",studySet);

}
