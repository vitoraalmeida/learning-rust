const MAX_POINTS_GLOBAL: u32 = 100_000;//constante global
//constantes exigem que o tipo seja definido.
//valem durante toda a execução do programa
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    //shadowing: usar o mesmo nome para outros valores ou/e tipos
    let x = "seis";
    println!("The value of x is: {}", x);
    //x = 6; //erro! variaveis são imutáveis por padrão em rust
    let mut y = 10;
    println!("The value of x is: {}", y);
    y = 15;
    println!("The value of x is: {}", y);
    //y = "dez" //erro! mutabilidade apenas do valor não do tipo

    const MAX_POINTS: u32 = 100_000;//constante válida apenas em main
}
