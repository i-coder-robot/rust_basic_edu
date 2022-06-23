fn main() {
    /*
        变量是有作用域的，也就是在一个代码块中生存。代码块 {}，也允许变量遮蔽。
    */

    //main函数中
    let spend = 1;
    {
        //只存在本代码块中
        let target = "面向加薪学习";
        println!("内部 {}",target);
        //遮蔽了外面的spend
        let spend=2.0;
        println!("内部 {}",spend);
    }
    //target在此作用域是不存在的
    // println!("外部 {}",target);
    println!("外部 {}",spend);
    //遮蔽了spend
    let spend=String::from("学习时间1小时");
    println!("外部 {}",spend);

    let spend2;
    {
        let x=2;
        spend2= x*x;
    }
    println!("spend2:{}",spend2);

    let spend3;
    // println!("spend3:{}",spend3);//报错，使用了未初始化的绑定。
    spend3=1;
    println!("another binding spend3:{}",spend3);

    let mut spend = 1;
    {
        //被不可变的spend遮蔽
        let spend = spend;
        //Cannot assign twice to immutable variable [E0384]
        //spend在本作用域被冻结
        // spend=50;
    }
    //spend离开作用域
    spend=3;
}
