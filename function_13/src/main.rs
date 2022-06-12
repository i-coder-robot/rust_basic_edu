fn main() {
   /*
        fn function缩写
        fn 函数名称([参数:参数类型]) -> 返回值{
            //执行逻辑代码
        }

        无明确返回值时候，就会返回一个单元类型 ()
   */

    hello();
    println!("{}",get_name());
    println!("{}",get_name2());

    let mut price=99;
    double_price(price);
    println!("外部price是{}",price);

    let mut price=88;
    double_price2(&mut price);
    println!("外部price是{}",price);

    let name = String::from("从0到Go语言微服务架构师");
    show_name(name);
    println!("调用show_name函数后{}",name);
}

fn hello(){
    println!("Hello,rust!");
}

fn get_name()->String{
    return String::from("Go语言微服务架构核心22讲");
}

//没有return关键字，函数会默认使用最后一条语句的执行结果作为返回值，并且数据类型要保持一致。
fn get_name2()->String{
     String::from("从0到Go语言微服务架构师")
}

//值传递，函数内部和外部各自保存了相同的值，互不影响，因此内外结果不一致。
fn double_price(mut price:i32){
    price=price*2;
    println!("内部的price是{}",price);
}
//引用传递，把当前变量的内存地址传递给函数。传递的变量和函数参数共同指向了同一个内存位置。引用传递在参数类型的前面加上&符号。
//* 解引用符，用于获取访问变量price指向的内存地址上存储的变量的值。
fn double_price2(price:&mut i32){
    *price = *price *2;
    println!("内部的price是{}",price);
}

fn show_name(name:String){
    println!("充电科目:{}",name);
}
