fn main() {
    /*
        if 条件表达式{
            //条件表达式 为 真，就执行这个代码。
        }
    */
    let total:f32 = 666.00;
    if total>500.00{
        println!("打8折,{}",total*0.8);
    }
    /*
        if 条件表达式{
            //条件表达式 为 真，就执行这个代码。
        }else{
            //条件表达式 为 假，就执行else中的这个代码。
        }
    */
    let total:f32 = 166.00;
    if total>500.00{
        println!("打8折,{}",total*0.8);
    }else{
        println!("无优惠折扣{}",total);
    }
    /*
        if...else if ... else
        if 条件表达式1{
            条件表达式1 为真，执行此业务逻辑
        }
        else if 条件表达式2{
            条件表达式2 为真，执行此业务逻辑
        }
        else{
            条件表达式1和条件表达式2读为假，执行此业务逻辑
        }
    */

    let total:f32=366.00;
    if total>200.00&&total<500.00{
        println!("打9折,{}",total*0.9);
    }else if total>500.00{
        println!("打8折,{}",total*0.8);
    }else{
        println!("无优惠折扣{}",total);
    }

    let code="10010";
    let choose = match code{
        "10010"=>"联通",
        "10086"=>"移动",
        _=>"Unknown"
    };
    println!("选择{}",choose);

    let code="80010";
    let choose = match code{
        "10010"=>"联通",
        "10086"=>"移动",
        _=>"Unknown"
    };
    println!("选择{}",choose);
}
