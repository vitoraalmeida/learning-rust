#[derive(Debug)] //permite a utilização do print utlizando a trait Debug {:?}
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { //method
    fn area(&self) -> u32 {//&self pega emprestimo tanto de variveis mut ou imut
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle { //associative function: não tem &self
        Rectangle {width: size, height: size} //serve para construtores
    }
}

fn main() {
    let width1 = 30;
    let height1 = 40;

    let rect1 = (30, 40);

    let mut rect2 = Rectangle { width: 30, height: 40};


    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect2)
    );


    rect2.height = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        rect2.area() //rust automaticamente faz (&rect2) ou (&mut rect2)
    );

    let rect2 = Rectangle { width: 30, height: 40};
    let rect3 = Rectangle { width: 40, height: 50};
    let rect4 = Rectangle::square(40);


    println!("rect2: {:#?}", rect2); //# permite um print mais legivel
    println!("rect3: {:#?}", rect3); //# permite um print mais legivel

    println!("rect2 can hold rect 3? {}", rect2.can_hold(&rect3));
    println!("rect4 is a square:\n{:#?}", rect4);

}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32,u32)) -> u32 {
   dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
