
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String, //só pode ser de um tipo
}


enum IpAddr2 { //torna desnecessário o uso do struct
    V4(u8, u8, u8, u8), //permite diferentes tipos para cada variante
    V6(String),
}

//kinds of message
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

//with enum
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

//também pode ter métodos
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}


fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let home2 = IpAddr2::V4(127,0,0,1);
    let loopback = IpAddr2::V6(String::from("::1"));

}
