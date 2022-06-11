use std::collections::HashMap;
use std::collections::HashSet;
use std::mem;

#[derive(Debug)]
struct MyNumber {
    num: i32,
}

impl From<i32> for MyNumber {
    fn from(item: i32) -> Self {
        MyNumber { num: item }
    }
}

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn main() {
    let s = String::from("从0到Go语言微服务架构师");
    // 这个闭包打印 `s`。它会立即借用（通过引用，`&`）`s` 并将该借用和
    // 闭包本身存储到 `c1` 变量中。`s` 会一直保持被借用状态直到
    // `c1` 离开作用域。
    // `println!` 只需传引用就能使用，而这个闭包捕获的也是变量的引用，因此无需
    // 进一步处理就可以使用 `println!`。
    let c1 = || println!("参与学习: {}",s);
    // 使用借用来调用闭包 `c1`。
    c1();

    // `c1` 可再次被不可变借用，因为闭包只持有一个指向 `c1` 的不可变引用。
    let c2 = &c1;
    c1();

    // 在最后使用 `c1` 之后，移动或重新借用都是允许的。
    let c3 = c1;
    c1();//这里也是可以使用的

    let mut sum = 0;
    // 这个闭包使 `sum` 值增加。需要得到 `&mut sum` 或者
    // `sum` 本身，但 `&mut count` 的要求没那么严格，所以可以采取这种方式。
    // 该闭包立即借用 `sum`。
    // `inc` 前面需要加上 `mut`，因为闭包里存储着一个 `&mut` 变量。调用闭包时，
    // 该变量的变化就意味着闭包内部发生了变化。因此闭包需要是可变的。
    let mut inc = || {
        sum += 1;
        println!("`sum`: {}", sum);
    };

    // 使用可变借用调用闭包
    inc();

    // 因为之后调用闭包，所以仍然可变借用 `sum`
    // 试图重新借用将导致错误
    // let _reborrow = &count;
    inc();

    // 闭包不再借用 `&mut sum`，因此可以正确地重新借用
    let _count_reborrowed = &mut sum;

    // 在竖线 | 之前使用 move 会强制闭包取得被捕获变量的所有权：
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));


    struct Study {
        name: String,
        target: String,
        spend: u32,
    }

    let s = Study {
        name: String::from("从0到Go语言微服务架构师"),
        target: String::from("全面掌握Go语言微服务落地，代码级一次性解决微服务和分布式系统。"),
        spend: 3,
    };

    let Study {
        name: name,
        target: target,
        spend: spend,
    } = s;

    println!("name = {}, target = {},  spend = {} ", name, target, spend);

    let s2 = Study {
        name: String::from("《Go语言极简一本通》"),
        target: String::from("学习Go语言，并且完成一个单体服务。"),
        spend: 5,
    };
    // 也可以忽略某些变量
    let Study { name, .. } = s2;
    println!("name = {}", name);

    let study = String::from("《Go语言极简一本通》"); //不可变变量
    let mut study2 = String::from("Go语言微服务架构核心22讲"); //可变变量

    println!("Before mutation: {}", study2);

    // 正确代码
    study2 = String::from("从0到Go语言微服务架构师");

    println!("修改后: {}", study2);

    // 错误！
    // study = String::from("从0到Go语言微服务架构师");

    let monday = Week::Monday;
    Do(monday);

    let work = Week::Work(String::from("从0到Go语言微服务架构师"));
    Do(work);

    let happy = Week::Happy {
        who: String::from("小红"),
        how: String::from("吃大餐"),
    };
    Do(happy);
}

enum Week {
    Sunday,    // 周天
    Monday,    // 周一
    Tuesday,   // 周二
    Wednesday, // 周三
    Thursday,  // 周四
    Friday,    // 周五
    Saturday,  // 周六
    // 或者一个元组结构体，
    Work(String),
    // 或者一个普通的结构体。
    Happy { who: String, how: String },
}

fn Do(w: Week) {
    match w {
        Week::Sunday => println!("Sunday"),
        Week::Monday => println!("Monday"),
        Week::Tuesday => println!("Tuesday"),
        Week::Wednesday => println!("Wednesday"),
        Week::Thursday => println!("Thursday"),
        Week::Friday => println!("Friday"),
        Week::Saturday => println!("Saturday"),
        Week::Work(s) => println!("我的工作是:{}", s),
        Week::Happy { who, how } => {
            println!("今天和{}去{}", who, how);
        }
    }
}
