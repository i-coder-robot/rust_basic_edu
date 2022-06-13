fn main() {
    /*
        let 切片变量=&变量[起始位置..结束位置] 左闭右开的一个区间
    */

    let mut v = Vec::new();
    v.push("Go语言极简一本通");
    v.push("Go语言微服务架构核心22讲");
    v.push("从0到Go语言微服务架构师");
    println!("len:{}",v.len());
    let s1=&v[..2];
    //[..]获取全部元素
    //[起始位置..] 获取 起始位置 到 整个容器的全部元素
    //[..结束位置] 获取 从0到结束位置的元素
    println!("s1:{:?}",s1);
    show_slice(s1);
    println!("s1:{:?}",s1);

    let mut v2 = Vec::new();
    v2.push("Go语言极简一本通");
    v2.push("Go语言微服务架构核心22讲");
    v2.push("从0到Go语言微服务架构师");
    println!("modify_slice 之前 v2:{:?}",v2);
    modify_slice(&mut v2[1..3]);
    println!("modify_slice 之后 v2:{:?}",v2);
}

fn show_slice(s:&[&str]){
    println!("show_slice函数内:{:?}",s);
}

fn modify_slice(s:&mut [&str]){
    s[0]="这个阶段学习完毕";
    println!("modify_slice:{:?}",s);
}
