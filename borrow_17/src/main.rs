fn main() {
    //借用  &变量名
    let studyList=vec!["Go语言极简一本通","Go语言微服务架构核心22讲","从0到Go语言微服务架构师"];
    let studyList2 = studyList;
    show(&studyList2);
    println!("{:?}",studyList2);

    let mut studyList3=vec!["Go语言极简一本通","Go语言微服务架构核心22讲","从0到Go语言微服务架构师"];
    println!("{:?}",studyList3);
    show2(&mut studyList3);
    println!("{:?}",studyList3);
    //借用borrow  从一个函数中的变量传递给另外一个函数作为参数暂时使用。函数离开后将所有权返回给当初传递给他的变量。
    //可变借用， 定义时候和使用的时候 都要用 &mut
}

fn show(v:&Vec<&str>){
    println!("{:?}",v);
}

fn show2(v:&mut Vec<&str>){
    v[0]="第一个充电项目已完成";
    println!("{:?}",v)
}
