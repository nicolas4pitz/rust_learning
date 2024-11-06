
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_contain(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn user_create(email: String, username: String) -> User {
    
    User {
        username: username,
        email: email,
        sign_in_count: 1,
        active: true,
    }


}


fn main() {
    let user1 = User {
        username: String::from("gordao"),
        email: String::from("gordao@gmail.com"),
        sign_in_count: 5,
        active: true,
    };
    
    let player= user_create(String::from("bundao@gmail.com"), String::from("Noob"));

    println!("User1: {}", user1.username);
    println!("email: {}", user1.email);
    println!("Sign in Count: {}", user1.sign_in_count);
    println!("Active: {}", user1.active);

    println!("");
    println!("Player: {}", player.username);
    println!("email: {}", player.email);
    println!("");


    let rect1 = Rectangle{
        width: 5,
        height: 5,
    };

    let rect2: Rectangle = Rectangle{
        width: 4,
        height: 4,
    };

    println!("Area do retangulo: {}", rect1.area());
    println!("Retangulo 1 consegue ficar no retangulo 2: {}", rect1.can_contain(&rect2));

}
