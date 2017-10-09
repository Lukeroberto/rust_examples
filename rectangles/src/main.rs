fn main() {
    //test_1();
    //test_2();
    test_3();
}

struct Rectangle {
    length: u32,
    width:  u32,
}

struct Rect_impl {
    length: u32,
    width:  u32,
}

fn test_1(){
    let length = 50;
    let width  = 30;

    println!("The area of the rectangle is {} units squared", area(length,width));
}

fn test_2(){
    let rect = Rectangle { length: 50, width: 30};

    println!(" The area of the rectangle is {} square pixels.", area_struct(&rect));
}

fn test_3(){
    let sqr = Rect_impl::square(3);

    println!("The area of the rectangle is {} square pixels.", sqr.area())
}

fn area(length: u32, width: u32) -> u32 {
    length*width
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}

impl Rect_impl {
    fn area(&self) -> u32 {
        self.length*self.width
    }

    fn square(size: u32) -> Rect_impl {
        Rect_impl { length: size, width: size}
    }
}
