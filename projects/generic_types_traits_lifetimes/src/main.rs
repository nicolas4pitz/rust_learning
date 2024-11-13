//Parte de generic
// struct Ponto<T>{
//     x: T,
//     y: T, //se mudar para U ele aceita int e float, mas os T não podem ser diferentes na mesma definicao
// }

// struct Point<X1, Y1> {
//     x: X1,
//     y: Y1,
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// impl<T> Ponto<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// impl<X1, Y1> Point<X1, Y1>{
//     fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2>{
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// } Parte de generic

//Traits definition
// pub trait Summary {
//     fn summarize(&self) -> String;
//     fn summarize_author(&self) -> String;
// }

// pub struct NewsArticle {
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }

//     fn summarize_author(&self) -> String {
//         format!("@{}", self.author)
//     }
// }

// pub struct Tweet {
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub retweet: bool,
// }

// impl Summary for Tweet{
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }

//     fn summarize_author(&self) -> String {
//         format!("@{}", self.username)
//     }
// } 

// fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// } traits definition 

struct Livro<'a>{
    titulo: &'a str,
    autor: &'a str,
}

fn main() {
    //println!("Maior numero: {}", maior(10, 20));
    //println!("Menor numero: {}", maior('a', 'z'));

    //point_impl();
    //mexer_genericos();

    //tweet();

    let texto1 = "Ola";
    let texto2 = "Mundo";
    let resultado = maior_texto(texto1, texto2);
    println!("Maior texto: {}", resultado);

    livro_struic();
}

// fn maior<T: PartialOrd>(a: T, b: T) -> T {
//     if a>b {
//         a
//     } else {
//         b
//     }
// }

// fn int_float() {
//     let ponto_int = Ponto {x: 5, y: 10};
//     let ponto_float = Ponto {x: 1.0, y: 4.0};

//     println!("Ponto int: ({}, {})", ponto_int.x, ponto_int.y);
//     println!("Ponto float: ({}, {})", ponto_float.x, ponto_float.y);

// }

// fn point_impl() {
//     let p = Ponto {x: 5, y: 10};

//     println!("p.x = {}", p.x());
// }

// fn mexer_genericos() {
//     let p1 = Point {x: 5, y: 10.4};
//     let p2 = Point {x: "Teste", y: "c"};

//     let p3 = p1.mixup(p2);

//     println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
// }

// fn tweet(){
//     let tweet = Tweet {
//         username: String::from("horse_benjack"),
//         content: String::from("of course, probaly"),
//         reply: false,
//         retweet: false,
//     };

//     println!("1 new tweet: {}", tweet.summarize());
//     println!("Author: {}", tweet.summarize_author())
// } traits definition 

fn maior_texto<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn livro_struic() {
    let titulo = "Rust Book";
    let autor = "Steve Klanb";
    let livro = Livro {titulo, autor};
    println!("Livro: '{}' por {}", livro.titulo, livro.autor);
}