//定义一个struct
struct User {
    username:String,
    email:String,
    sign_in_count:u64,
    active:bool,
}

//定义两个Tuple struct
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

fn main() {
    
    //实例化struct：一旦实例是可变的，那么实例中所有的字段都是可变的
    let mut user1 = User {
        email:String::from("abc@126.com"),
        username:String::from("Nikky"),
        active:true,
        sign_in_count:556,
    };
    println!("实例化的email:{}",user1.email);

    //获取struct里面的某个值
    user1.email = String::from("anotheremail@example.com");
    println!("获取实例化的email并重新赋值后:{}",user1.email);

    //struct更新语法
    let mut user2 = User {
        email:String::from("abc@qq.com"),
        username:String::from("anothername"),
        active:user1.active,
        sign_in_count:user1.sign_in_count,
    };

    //user2和user等价
    let mut user3 = User {
        email:String::from("abc@qq.com"),
        username:String::from("anothername"),
        ..user1
    };

    //Tuple struct
    let black = Color(0,0,0);
    let origin = Point(0,0,0);          //即使black和origin值一样，但是是不同的类型，不同Tuple struct的实例



}

//struct作为函数的返回值
fn build_user(email:String,username:String) -> User {
    User{
        email:email,            //字段名和值对应的变量名相同可以简写为“email，”
        username,               //简写例子
        active:true,
        sign_in_count:1,
    }
}