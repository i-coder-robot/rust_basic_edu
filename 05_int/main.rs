fn main () {

    //有符号类型signed可以存储正数或负数, 无符号类型 usigned只能存储正数。
    //按存储控件来说，划分1字节，2字节，4字节，8字节，16字节， 1字节=8位
    let price =100;
    let price2:u32=200;
    let price3:i32=-300;
    let price4:isize=400;
    let price5:usize=500;

    println!("price is {}",price);
    println!("price2 is {} and price3 is {}",price2,price3);
    println!("price4 is {} and price5 is {}",price4,price5);

    // let price6:i32=66.66;
    let price7:i8=192;
}