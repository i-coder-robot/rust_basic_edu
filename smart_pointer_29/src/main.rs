use std::ops::Deref;

fn main() {
    /*
        如果一个结构体，实现了deref和drop的Trait，那他们就不是普通结构体了。
        Rust提供了堆上存储数据的能力并把这个能力封装到了Box中。
        把栈上的数据搬到堆上的能力，就叫做装箱。
        Box可以把数据存储到堆上，而不是栈上，box  装箱，栈还是包含指向堆上数据的指针。

        星号 * 解引用符。
    */
    let a = 6;
    let b = Box::new(a);
    println!("b={}",b);

    let price1 = 158;
    let price2 = Box::new(price1);
    println!("{}",158==price1);
    println!("{}",158==*price2);

    let x = 666;
    let y =CustomBox::new(x);

    println!("666==x is {}",666==x);
    println!("666==*y is {}",666==*y);
    println!("x==*y is {}",x==*y);
}

struct CustomBox<T>{
    value:T
}

impl<T> CustomBox<T>{
    fn new(v:T)->CustomBox<T>{
        CustomBox{value:v}
    }
}
impl<T> Deref for CustomBox<T>{
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T> Drop for CustomBox<T>{
    fn drop(&mut self) {
        println!("drop CustomBox 对象!")
    }
}
