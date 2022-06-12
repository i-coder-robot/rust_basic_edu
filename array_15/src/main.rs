fn main() {
    //let 变量名称:[数据类型;数组长度]=[值1,值2,值3,...];
    let arr1:[&str;3]=["Go语言极简一本通","Go语言微服务架构核心22讲","从0到Go语言微服务架构师"];
    //let 变量名称=[值1,值2,值3,...];
    let mut arr2=["Go语言极简一本通","Go语言微服务架构核心22讲","从0到Go语言微服务架构师"];
    //let 变量名称:[数据类型;数组长度]=[默认值;数组长度];
    let arr3:[&str;3]=["";3];

    println!("{}",arr1.len());

    for item in arr1{
        println!("充电科目:{}\n",item);
    }

    for item in arr1.iter(){
        println!("已参加的充电科目:{}\n",item);
    }

    // arr2[0]="";
    println!("{:?}",arr2);
    show_arr(arr2);
    println!("{:?}",arr2);

    let mut arr3=["Go语言极简一本通","Go语言微服务架构核心22讲","从0到Go语言微服务架构师"];
    println!("{:?}",arr3);
    modify_arr(&mut arr3);
    println!("{:?}",arr3);
}

fn modify_arr(arr: &mut[&str;3]){
    let l = arr.len();
    for i in 0..l{
       arr[i]="";
    }
}

fn show_arr(mut arr:[&str;3]){
    let l = arr.len();
    for i in 0..l{
        if i==0{
            arr[0]="";
        }
        println!("充电科目:{}",arr[i])
    }
}
