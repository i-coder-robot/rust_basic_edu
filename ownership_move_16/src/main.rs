fn main() {
   let name="从0到Go语言微服务架构师";

    //move
    // let a=b
    // foo(a)
    /*
        栈 后进先出。类型大小是固定的。如i32.
        堆 编译时大小未知或不确定大小。用户自己管理，增加内存溢出风险。
    */

    let a=88;
    let b=a;
    println!("a {}, and b {}", a ,b);

    // let v1=vec!["Go语言极简一本通","Go语言微服务架构核心22讲","从0到Go语言微服务架构师"];
    // let v2= v1;
    // println!("{:?}",v1);
    //
    // let studyList=vec!["Go语言极简一本通","Go语言微服务架构核心22讲","从0到Go语言微服务架构师"];
    // let studyList2 = studyList;
    // show(studyList2);
    // println!("studyList2 {:?}",studyList2);

    let studyList3=vec!["Go语言极简一本通","Go语言微服务架构核心22讲","从0到Go语言微服务架构师"];
    let studyList4=studyList3;
    let result = show2(studyList4);
    println!("result:{:?}",result);
    println!("result:{:?}",studyList4);

    //赋值并不是唯一涉及移动所有权的操作，值在作为参数传递或从函数返回时也会被移动。

}

fn show2(v:Vec<&str>) -> Vec<&str>{
    println!("v {:?}",v);
    return v;
}

fn show(v:Vec<&str>){
    println!("v {:?}",v)
}
