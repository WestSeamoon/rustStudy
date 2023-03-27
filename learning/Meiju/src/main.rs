//定义枚举类型
enum IpAddrKind {
    V4,
    V6
}

//枚举嵌入struct
struct IpAddr {
    kind:IpAddrKind,
    address:String,
}

//定义带数据附的枚举
enum IpAddrKindData {
    V4(u8,u8,u8,u8),            //嵌入值
    V6(String),
}

//可以将任意类型嵌入到枚举的变体中
enum Message {
    Quit,       //可以不带值
    Move { x: i32, y: i32},
    Write(String),
    ChangColor(i32,i32,i32),
}

//为枚举定义方法
impl Message {
    fn call(&self){

    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);
    route(IpAddrKind::V6);
    
    //枚举嵌入struct
    let home = IpAddr {
        kind:IpAddrKind::V4,
        address:String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind:IpAddrKind::V6,
        address:String::from("::1"),
    };

    //将数据附加到枚举的变体中
    let homedata = IpAddrKindData::V4(127,0,0,1);
    let loopbackdata = IpAddrKindData::V6(String::from("::1"));

    //将任意类型嵌入到枚举的变体中
    let q = Message::Quit;
    let m = Message::Move{ x: 12, y: 24 };
    let w = Message::Write(String::from("Hello"));
    let c = Message::ChangColor(1,255,255);

    m.call();

    //Rust中没有null，但有类似null概念的枚举 -Option<T>
    let some_number = Some(5);
    let some_String = Some("A String");

    let absent_number:Option<i32> = None;

    //不可以把Option<T>直接当成T
    let x:i8 = 5;
    let y:Option<i8> = Some(5);
    //let sum = x + y;                  //会报错，若想使用Option<T>中的T，必须把它转换为T
    //let sumt=some_number+y;           //Option<T>的T之间也不能进行运算

}

fn route(ip_kind:IpAddrKind) {
    //可以为空
}