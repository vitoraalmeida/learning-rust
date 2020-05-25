struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {

    //tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let user1 = User { //Instancia imutável
        username: String::from("vitoralmd"),
        email: String::from("vitoralmeida@gmail.com"),
        sign_in_count: 1,
        active: true,
    };

    //user1.active = false; //erro, pois a instancia é imutavel

    let mut user2 = User { //instancia mutavel. Todos os campos são mutáveis
        username: String::from("capadocancio"),
        email: String::from("capa@gmail.com"),
        //active: user1.active;
        //sign_in_count: user1.sign_in_count;
        ..user1 //o que não tiver sido definido diretamente sera igual
    };

    let mut user3 = build_user(
        String::from("benzao"),
        String::from("benzao@gmail.com"));

    user2.username = String::from("capa");
    user3.username = String::from("analais");
    println!("User 1 name: {}", user1.username);
    println!("User 2 name: {}", user2.username);
    println!("User 3 name: {}", user3.username);

}
