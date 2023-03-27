mod front_of_house;

mod serving {
    fn tske_order(){}
    fn server_order(){}
    fn take_payment(){}
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();          //使用绝对路径调用
    //同级引用可以去掉根
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_house::Breakfast::summer("Rye");        //传入Rye，构建breakfast实例
    meal.toast = String::from("Wheat");                         //由于toast是pub的，所以可以修改
    println!("I'd like {} toast please",meal.toast);
    //meal.seasonal_fruit = String::from("nlueberries");        //seasonal是私有的，修改就会报错

}


fn server() {}

mod back_of_house {
    fn fix_incorrect_order(){
        cook_order();               //调用同级目录
        super::server();            //使用super调用上级目录，相对路径
        crate::server();            //使用绝对路径调用上级目录
    }

    fn cook_order(){}
}

//struct默认私有,需要单独设置pub变公有
mod back_house {
    pub struct Breakfast {
        pub toast: String,                                  //这里要加pub才是公共的
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer (toast:&str) -> Breakfast{            //传入一个toast参数（切片类型），构建Breakfast实例
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit:String::from("peaches"),
            }
        }
    }

    //pub放在enum前，除了枚举类型是公共的，其所有的变体都是公共的
    pub enum Appetizer {            //这个是公共的
        Soup,
        Salad,                      //这两个也是公共的，不需要再加pub
    }
}


//把front_of_house::hosting引用到当前作用域，针对函数，通常引用到父模块,不加pub的引用在作用域内也是私有的
use crate::front_of_house::hosting;                             //使用绝对路径
//use front_of_house::hosting;                                  //使用相对路径，与调用目录一致
//mod hosting{}                                                 //相当于这个效果
//pub use crate::front_of_house::hosting;                       //加pub外部就可以访问了

pub fn eat_restaurant() {
    hosting::add_to_waitlist();
    //hosting::seat_at_table();                                 //非pub一样不能引用，会报错，也需要遵守私有性规则
}

