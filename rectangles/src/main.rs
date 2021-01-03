#[derive(Debug)]
struct Rectangle {
    width:  u32,
    height: u32,
}

// Methods
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated function
    #[allow(dead_code)]
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width:  size,
            height: size,
        }
    }
}

fn main() {

    let rect1 = Rectangle {
        width:  30,
        height: 50,
    };
    let rect2 = Rectangle {
        width:  10,
        height: 40,
    };
    let rect3 = Rectangle {
        width:  60,
        height: 45,
    };


    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

//    println!("{:#?}", rect1)

}

#[test]
fn test_area() {

    fn area_vars(width: u32, height: u32) -> u32 {
        width * height
    }

    fn area_tuple(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }

    fn area_struct(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

    let rect1 = (30, 50);
    let width1   = 30;
    let height1  = 50;

    let rect2 = Rectangle {
        width:  30,
        height: 50,
    };

    let sq = Rectangle::square(3);

    assert_eq!(area_tuple(rect1), area_vars(width1, height1));
    assert_eq!(area_tuple(rect1), area_struct(&rect2));
    assert_eq!(sq.height, sq.width);
}
