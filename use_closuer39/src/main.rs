fn main() {
    let add = |x, y| x + y;
    let result = add(3, 4);
    println!("{}", result);

    receives_closure(add);

    let y = 2;
    receives_closure2(|x| x+y);

    let y =3;
    receives_closure2(|x| x+y);

    let closure = returns_closure();
    println!("返回闭包=>{}",closure(1));

    let result = do1(add,5);
    println!("result(1)=>{}",result(1));

    let result = do2(add,5);
    println!("result(2)=>{}",result(2));
}

fn do2<F,X,Y,Z>(f:F,x:X)->impl Fn(Y)->Z
    where
        F:Fn(X,Y)->Z,
        X:Copy{
    move |y| f(x,y)
}

fn do1<F>(f:F,x:i32)->impl Fn(i32)->i32
    where
        F:Fn(i32,i32)->i32
{
    move |y| f(x,y)
}

fn returns_closure() -> impl Fn(i32)->i32{
    |x| x+6
}

fn receives_closure<F>(closure: F)
    where
        F: Fn(i32, i32) -> i32
{
    let result = closure(3, 5);
    println!("闭包作为参数执行结果=>{}", result)
}


fn receives_closure2<F>(closure: F)
    where
        F: Fn(i32) -> i32 {
    let result = closure(1);
    println!("closure(1)=>{}",result);
}
