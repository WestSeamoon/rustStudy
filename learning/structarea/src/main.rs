#[derive(Debug)]                //必须为结构体显式地向阿选择debug功能
struct Rectangle {
    width:u32,
    length:u32,
}

fn main() {
    //普通方法,长和宽貌似没有关系
    let w=30;
    let l=50;
    println!("普通方法计算的长方形面积为:{}",area(w,l));

    //Tuple方法，不知道哪个是长哪个是宽
    let rect = (30,50);
    println!("Tuple方法计算的长方形面积为:{}",areatuple(rect));

    //struct方法，可读性最强
    let rectstruct = Rectangle {
        width:30,
        length:50,
    };
    println!("struct方法计算的长方形面积为:{}",areastruct(&rectstruct));

    //打印struct
    println!("{:#?}",rectstruct);           //加#号可以分行，更易读

}

fn area(width:u32,length:u32) -> u32 {
    width * length
}

fn areatuple (dim:(u32,u32)) -> u32 {
    dim.0 * dim.1
}

fn areastruct(rect:&Rectangle) -> u32 {
    rect.width * rect.length
}