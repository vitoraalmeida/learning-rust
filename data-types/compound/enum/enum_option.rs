fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, //se o caso None não tivesse sido tratado, 
        Some(i) => Some(i + 1),                    // daria erro
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let unwraped_six = six.unwrap(); //Para pegar o valor do Some(i).

    println!("5 + 1 = {}", unwraped_six);

    let none = plus_one(None);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), //todo caso que não for os que foram listados, 
                 // cairão no _
    }

    //using if let
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("Anything else");
    }
}
