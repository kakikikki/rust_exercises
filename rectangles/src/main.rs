#[derive(Debug)]
struct Rectangle {
    width:  u32,
    height: u32,
}

fn main() {

    let rect1 = Rectangle {
        width:  30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect1)
    );

    println!("{:#?}", rect1)

}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[test]
fn test_area() {

    fn area_vars(width: u32, height: u32) -> u32 {
        width * height
    }

    fn area_tuple(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }

    let rect1 = (30, 50);
    let width1   = 30;
    let height1  = 50;

    assert_eq!(area_tuple(rect1), area_vars(width1, height1));
}
