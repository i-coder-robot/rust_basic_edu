fn main() {
    /*
        元组结构体
        struct Pair(String,i32);

        C语言风格结构体
        struct 结构体名称{
            ...
        }

        单元结构体,不带字段，在泛型中很有用。
        type Unit


        struct 结构体名称{
            字段1:数据类型
            字段2:数据类型
            ...
        }


        let 实例名称 = 结构体名称{
            字段1:数据1
            字段2:数据2
            ...
        }
    */

    let s = Study{
        name:String::from("从0到Go语言微服务架构师"),
        target:String::from("全面掌握Go语言微服务落地，代码级一次性解决微服务和分布式系统。"),
        spend: 3,
    };
    println!("{:?}",s);

    println!("{}",s.name);

    let s3 = get_instance(
        String::from("Go语言极简一本通"),
        String::from("学习Go语言语法，并且完成一个单体服务"),5);
    println!("{:?}",s3);

    // show(s3);

    /*
        impl 结构体{
            fn 方法名(&self,参数列表) 返回值{
                //方法体
            }
        }

        函数 可以直接调用，同一个程序不能出现2个相同的函数签名的函数，因为函数不归属任何owner.
        方法 归属某一个owner,不同的owner可以有相同的方法签名。

        impl 是 implement的缩写。 "实现"的意思。

        self "自己"的意思，  &self表示当前结构体的实例。也是结构体普通方法固定的第一个参数，其他参数可选。

        结构体静态方法
        fn 方法名(参数:数据类型,...) -> 返回类型{
            //方法体
        }
        调用方式  结构体名称::方法名(参数列表)

    */

    println!("spend {}",s3.get_spend());
    let s4 = Study::get_instance_another(
        String::from("Go语言极简一本通"),
        String::from("学习Go语言语法，并且完成一个单体服务"),5);
    println!("s4 {:?}",s4);

    /*
        单元结构体
        是一个类型，有且只有一个值()
    */

    //元组结构体
    let pair=(String::from("从0到Go语言微服务架构师"),1);

    println!("pair 包含{:?} and {:?}",pair.0,pair.1);

    //解构元组结构体
    let (study,spend)  = pair;
    println!("pair 包含{:?} and {:?}",study,spend);
}

fn show(s:Study){
    println!("name is {} target is {} spend is {}",s.name,s.target,s.spend);
}

fn get_instance(name:String,target :String,spend:i32) -> Study{
    return Study{
        name,
        target,
        spend,
    }
}

#[derive(Debug)]
struct Study {
    name:String,
    target:String,
    spend:i32
}

impl Study{
    fn get_spend(&self)->i32{
        return self.spend;
    }

    fn get_instance_another(name:String,target:String,spend:i32)->Study{
        return Study{
            name,
            target,
            spend
        };
    }
}
