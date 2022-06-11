fn main() {
   /*
    const 常量名称:数据类型=值;
    常量名称通常是大写字母。
    常量可以在任何地方定义，常量只是一个符号，编译时替换为具体的值。

    static 具有'static声明周期的，可以是可变的变量（static mut关键字）
   */

    const PI:f64=3.1415926;
    println!("{}",PI);


    //变量的隐藏
    let name="《Go语言极简一本通》";
    let name = "《从0到Go语言微服务架构师》";
    println!("{}",name);

    //隐藏变量并且改变数据类型
    let price = 199;
    let price="299";
    println!("{}",price);

    //常量不能被隐藏，也不能被重复定义
    // const DISCOUNT:f64 = 0.8;
    const DISCOUNT:f64 = 0.6;


    static BOOK:&'static str = "《Go语言极简一本通》";
    println!("{}",BOOK);

}
