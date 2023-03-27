#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle{                 //定义块
    //只用一个参数
    fn area(&self) -> u32 {
        self.width * self.length
    }

    //除第一个参数外，还需要传入第二个参数
    fn can_hold(&self,other:&Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }

    //关联函数
    fn square(size:u32) -> Rectangle {
        Rectangle{
            width:size,
            length:size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width:30,
        length:50,
    };
    println!("rect1的面积为:{}",rect1.area());
    println!("rect1的参数为{:#?}",rect1);

    let rect2 = Rectangle {
        width:10,
        length:40,
    };

    let rect3 = Rectangle {
        width:35,
        length:55,
    };

    println!("rect1是否能框住rect2:{}",rect1.can_hold(&rect2));
    println!("rect1是否能框住rect3:{}",rect1.can_hold(&rect3));

    let s = Rectangle::square(20);

    println!("正方形的参数为:{:#?}",s);

}
