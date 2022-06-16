fn main() {
    /*
        enum 枚举名称{
            variant1,
            variant2,
            ...
        }

        使用枚举
        枚举名称::variant

        Option经常用于函数的返回值，可以有返回值Some(T)，也可以没有返回值None。
        enum Option<T>{
            Some(T), //用于返回一个值
            None     //用于返回null,用None来代替
        }

        match 判断枚举变量的值。

        带数据类型的枚举
        enum 枚举名称{
            variant1(数据类型1),
            variant2(数据类型2),
            ...
        }
    */

    let level3=RoadMap::从0到Go语言微服务架构师;
    println!("level3--{:?}",level3);

    let p=6;
    let result=get_discount(p);
    println!("{:?}",result);

    print_road_map(RoadMap::Go语言极简一本通);
    print_road_map(RoadMap::Go语言微服务架构核心22讲);
    print_road_map(RoadMap::从0到Go语言微服务架构师);

    let level3 = StudyRoadMap::Name(String::from("从0到Go语言微服务架构师"));
    match level3{
        StudyRoadMap::Name(val)=>{
            println!("{:?}",val);
        }
    }
}

enum StudyRoadMap{
    Name(String),
}

fn print_road_map(r:RoadMap){
    match r {
        RoadMap::Go语言极简一本通=>{
            println!("Go语言极简一本通")
        },
        RoadMap::Go语言微服务架构核心22讲=>{
            println!("Go语言微服务架构核心22讲")
        }
        RoadMap::从0到Go语言微服务架构师=>{
            println!("从0到Go语言微服务架构师")
        }
    }
}

fn get_discount(price:i32)->Option<bool>{
    if price>100{
        Some(true)
    }else{
        None
    }
}

#[derive(Debug)]
enum RoadMap{
    Go语言极简一本通,
    Go语言微服务架构核心22讲,
    从0到Go语言微服务架构师,
}
//这是Go语言学习的3个阶段
