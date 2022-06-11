fn main() {
    //&str 是在模块 std:str,字符串切片。
    let lesson="Go语言微服务架构核心22讲";

    /*
        字符串对象 在堆上分配，可以在运行时提供字符串值以及响应的操作方法。
        String::new()创建一个新的空字符串，他是静态的
        String::from() 从具体的字符串字面量创建字符串对象。
    */
    let s1= String::new();
    println!("s1:{},s1-len:{}",s1,s1.len());

    let s2  = String::from("面向加薪学习");
    println!("s2:{},s2-len:{}",s2,s2.len());

    let mut s3 = String::new();
    s3.push_str("Go语言极简一本通");
    println!("{}",s3);

    s3.push('O');
    s3.push('K');
    println!("{}",s3);

    let s4 = String::from("面向加薪学习");
    let result =s4.replace("面向加薪学习","www.go-edu.cn");
    println!("{}",result);

    let s5=String::from("面向加薪学习 www.go-edu.cn");
    println!("length is {}",s5.len());

    let s6="从0到Go语言微服务架构师".to_string();
    println!("{}",s6);

    let s7  = String::from("Go语言微服务架构核心22讲");
    show_name(s7.as_str());

    //去掉头尾空白符 ， 制表符\t 空格 回车\r 换行\n 回车换行\r\n
    let s8=" \tGo语言极简一本通\tGo语言微服务架构核心22讲 \r\n从0到Go语言微服务架构师\r\n     ";
    println!("length is {}",s8.len());
    println!("length is {}",s8.trim().len());
    println!("{}",s8);

    let s9 = "Go语言极简一本通、Go语言微服务架构核心22讲、从0到Go语言微服务架构师";
    for item in s9.split('、'){
        println!("充电科目：{}",item);
    }

    let s10 = "从0到Go语言微服务架构师";
    for c in s10.chars(){
        println!("字符:{}",c);
    }

    let s11 ="Go语言极简一本通".to_string();
    let s12 = " 欢喜".to_string();

    let result2= s11+&s12;
    println!("{}",result2);
}

fn show_name(name:&str){
    println!("充电科目:{}",name);
}
