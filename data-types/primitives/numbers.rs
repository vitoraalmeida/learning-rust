fn main() {
    //por padrão, integers são i32
    let twenty = 20;
    let twenty_one: i32 = twenty + 1;
    let floats_okay = 21.0;
    let one_million = 1_000_000;

    println!("{}; {}; {}; {}", twenty, twenty_one, floats_okay, one_million);

    //é possivel definir de forma explicitas em outras bases
    let three = 0b11;
    let thirty = 0o36;
    let three_hundred = 0x12C;
    
    //é possivel vizualizar também na representaçao de cada base
    println!("base 10: {} {} {}", three, thirty, three_hundred);
    println!("base 2: {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("base 8: {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);


    let max_u8: u8 = 255;
    let sixtyfive: u8 = b'A'; //Armazena o valor numérico referente ao caractere na tabela ASCII = 65
    let min_i8: i8 = -128;
    let max_i8: i8 = 127;
    
    println!("max u8: {}",max_u8);
    println!("Min i8: {}, max i8: {}", min_i8, max_i8);

    let max_u16: u16 = 65535;
    let min_i16: i16 = -32768;
    let max_i16: i16 = 32767;
    println!("max u16: {}", max_u16);
    println!("Min i16: {}, max i16: {}", min_i16, max_i16);

    let max_u32: u32 = 4294967295;
    let min_i32: i32 = -2147483648;
    let max_i32: i32 = 214748367;
    println!("max u32: {}",max_u32);
    println!("Min i32: {}, max i32: {}", min_i32, max_i32);

    let max_u64: u64 = 18446744073709551615;
    let min_i64: i64 = -9223372036854775808;
    let max_i64: i64 = 9223372036854775807;
    
    //isize depende da arquitetura. 64 bits am arquiteturas 64, 32 em arquiteturas 32.
    let x86_64: isize = 9223372036854775807;
    println!("max u64: {}", max_u64);
    println!("Min i64: {}, max i64: {}", min_i64, max_i64);

    let max_u128: u128 = 340282366920938463463374607431768211455;
    println!("max u128: {}", max_u128);
    
    //padrão é f64
    let float_padrao = 64.64;
    let float_definido: f32 = 64.64;
    
    //operações entre inteiros literais resultam em inteiro
    let res = 4/3; //resulta em 1
    let res2 = 4.0/3; //erro! não há promoção de um operador para satisfazer o outro
    let res3 = 4.0/3.0; //erro! não há promoção de um operador para satisfazer o outro
    
    //boolean
    let t = true;
    let f: bool = false; // with explicit type annotation
    
    //char = Unicode -> muito mais que ascii
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
     
}
