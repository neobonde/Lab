#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {

    let rect5 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect6 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect7 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect5 hold rect6? {}", rect5.can_hold(&rect6));
    println!("Can rect5 hold rect7? {}", rect5.can_hold(&rect7));

    let rect8 = Rectangle::square(30);
    dbg!(rect8);


    let width1 = 30;
    let height1 = 50;

    let rect1 = (30, 50);
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    let scale = 2;
    let rect3 = Rectangle{
        width: dbg!(30*scale),
        height: 50,
    };

    dbg!(&rect3);

    let rect4 = Rectangle {
        width: 30,
        height: 50,
    };


    println!(
        "The area of the rectangle is {} square pixels.",
        area_arg(width1,height1)
    );

    println!(
        "The area of the rectangle 1is {} square pixels.",
        area_tup(rect1)
    );


    println!(
        "The area of the rectangle2 is {} square pixels.",
        area_struct(&rect2)
    );

    println!(
        "The area of the rectangle4 is {} square pixels.",
        rect4.area()
    );
    if rect4.width() {
        println!("The rectangle has a nonzero width; it is {}",rect4.width);
    }

    println!("rect2 is {:?}", rect2);
    println!("rect2 is {:#?}", rect2);
    dbg!(rect2);

}

fn area_arg(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tup(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
