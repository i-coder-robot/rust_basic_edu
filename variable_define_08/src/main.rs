fn main() {
    /*
        定义变量的格式
        let 变量名=值; 不指定变量类型
        let 变量名:数据类型=值; 指定变量类型

        变量就是给某一个内存地址起名字。

        变量的命名规范
        1 可以是字母，数字，下划线。
        2 必须以字母或下划线开头。不能以数字开头。
        3 变量名区分大小写。

        可变变量
        mut关键字，mutable缩写。
        let mut 变量名=值;
        let mut 变量名:数据类型=值;
    */

    let Study = "";
    println!("{}",Study);

    let mut price=188;
    price=288;
    println!("{}",price);
}
