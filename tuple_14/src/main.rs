fn main() {
   /*
        let tuple变量名称:(数据类型1，数据类型2,...) = (数据1,数据2,...)
         let tuple变量名称 = (数据1,数据2,...)
   */
    let t = ("Go语言极简一本通","掌握Go语言语法，并且可以完成单体服务应用。");
    println!("{:?}",t);
    //元组变量.索引数字
    println!("{}",t.0);
    println!("{}",t.1);
    show_tuple(t);
    println!("{:?}",t);

    let(book,target) = t;
    println!("{}",book);
    println!("{}",target);

    let arr1:[&str;3]=["Go语言极简一本通","Go语言微服务架构核心22讲","从0到Go语言微服务架构师"];
    for item in arr1{
        print!("充电科目 :{}\n",item);
    }

}
fn show_tuple(t:(&str,&str)){
    println!("{:?}",t);
}
