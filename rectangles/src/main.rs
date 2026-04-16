fn main() {
    let width1 = 30;
    let height1 = 50;

    let rect = (width1, height1);

    let rectangle = Rectangle {
        //width: dbg!(width1 * 2 / 2),
        width: width1,
        height: height1,
    };

    println!(
        "Given width {} and height {} as values, the area of the rectangle is {} square pixels.",
        width1,
        height1,
        area(width1, height1)
    );

    println!(
        "Given width {} and height {} inside a tuple, the area of the rectangle is {} square pixels.",
        rect.0,
        rect.1,
        area_tuples(rect)
    );

    // println!("Printing out rectangle struct: {rectangle:?}"); // debug printout all in the same row
    println!("Printing out rectangle struct: {rectangle:#?}"); // debug printout in different rows
                                                               // dbg!(&rectangle); // other type of debug
    println!(
        "Given a struct containing width {} and height {}, the area of the rectangle is {} square pixels.",
        rect.0,
        rect.1,
        area_struct(&rectangle)
    );

    println!(
        "Given a struct and a method to calculate area, the area is {} square pixels.",
        rectangle.area()
    );
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rect_struct: &Rectangle) -> u32 {
    // we want to borrow the struct rather than take ownership of it
    rect_struct.width * rect_struct.height
}
