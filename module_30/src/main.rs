use mylib::add_salary::study;

fn main() {
    /*
        私有模块
        mod module_name{
            fn function_name(){

            }
        }

        pub关键字

        pub mod public_module{
            pub fn function_name(){
                //公开方法
            }

            fn function_name2(){
                //私有方法
            }
        }

        使用模块
        use 公开模块::函数名称；

        pub mod mod1{
            pub mod mod2{
                pub mod mod3{
                    pub fn function_name(参数){
                        代码逻辑
                    }
                }
            }
        }

        use mod1::mod2::mod3::function_name;
    */

    study("从0到Go语言微服务架构师".to_string());
}
