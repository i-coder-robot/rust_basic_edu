use std::fmt::{Display, Formatter};

fn main() {
    let mut v:Vec<i32>=vec![1,2,3];
    // v.push("4");

    /*
        泛型结构体
        struct 结构体名称<T>{
            字段:T,
        }
    */

    let t:Data<i32>=Data{value:100};
    println!("值:{}",t.value);

    let t:Data<f64>=Data{value:66.00};
    println!("值:{}",t.value);

    /*
        特质Trait,对标其他语言的接口，都是行为的抽象。使用trait关键字用来定义。可以是具体方法，也可以是抽象的方法。
        trait some_trait{
            抽象方法
            fn method1(&self);

            具体实现的普通方法
            fn method2(&self){
                方法的具体代码
            }
        }

        impl for 为每个结构体实现某个特质。
    */
    let book=Book{
        id:1,
        name:String::from("Go语言极简一本通"),
        author:String::from("欢喜"),
    };
    book.Show();

    /*
        泛型函数，主要是参数类型是泛型，不要求所有参数都必须是泛型参数，可以是摸一个参数是泛型的。
        fn 函数名称<T[:特质名称]>(参数1:T,...){
            函数实现代码
        }
    */
    show2(book);
}

struct Book{
    name:String,
    id:u32,
    author:String,
}

fn show2<T:Display>(t:T){
    println!("{}",t);
}

impl Display for Book{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        println!("ID:{},Name:{},Author:{}",self.id,self.name,self.author);
        let r=Result::Ok(());
        return r;
    }
}

trait ShowBook{
    fn Show(&self);
}

impl ShowBook for Book{
    fn Show(&self) {
        println!("ID:{},Name:{},Author:{}",self.id,self.name,self.author);
    }
}


struct Data<T>{
    value:T,
}
