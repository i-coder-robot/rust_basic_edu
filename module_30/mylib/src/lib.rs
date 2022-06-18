#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub mod add_salary{
    pub fn study(name:String){
        println!("面向加薪学习 {}",name);
    }
}
