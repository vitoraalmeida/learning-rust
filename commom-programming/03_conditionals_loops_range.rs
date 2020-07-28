fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    //error: só expressões booleanas podem ser usadas
    //if number { //não há conversão automática para bool como em JS
    //    println!("Was a number");
    //}

    let number = 15;

    if number % 2 == 0 {
        println!("number is divisible by 2");
    } else if number % 3 == 0 && number % 5 == 0 {
        println!("number is divisible by 15");
    } else {
        println!("number is not divisible by 2, 3, 5 or 15");
    }

    let condition = true;
    //let permite que se usem expressões para atribuição
    //atribuição condicional
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    let mut counter = 0;

    //loop repete até encontrar break
    let mut counter = loop {
        counter += 1;

        if counter == 10 {
            //o valor após o break será retornado para a atribuição
            break counter * 2;
        }
    };

    loop {
        println!("counter: {}", counter);
        counter -= 1;
        if counter == 0 {
            break
        }
    }

    while counter <= 20 {
        println!("counter: {}", counter);
        counter += 1;
    }

    let array: [u8; 5] = [1,2,3,4,5];
    let mut index = 0;

    while index < 5 {
        println!("array[{}] = {}", index, array[index]);
        index += 1;
    }

    index = 0;
    //.iter() retorna um iterador para a coleção, permitindo usar cada valor
    //sem saber o numero exato de itens
    for element in array.iter() {
        println!("array[{}] = {}", index, element);
        index += 1;
    }

    //range [1,6[
    for number in 1..6 {
        println!("{}", number);
    }

    //range reverso
    for number in (1..6).rev() {
        println!("{}", number);
    }
}
