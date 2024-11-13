

struct Ponto<T>{
    x: T,
    y: T, //se mudar para U ele aceita int e float, mas os T não podem ser diferentes na mesma definicao
}

struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}


enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T> Ponto<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl<X1, Y1> Point<X1, Y1>{
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2>{
        Point {
            x: self.x,
            y: other.y,
        }
    }
}


fn main() {
    println!("Maior numero: {}", maior(10, 20));
    println!("Menor numero: {}", maior('a', 'z'));

    point_impl();
    mexer_genericos();
}

fn maior<T: PartialOrd>(a: T, b: T) -> T {
    if a>b {
        a
    } else {
        b
    }
}

fn int_float() {
    let ponto_int = Ponto {x: 5, y: 10};
    let ponto_float = Ponto {x: 1.0, y: 4.0};

    println!("Ponto int: ({}, {})", ponto_int.x, ponto_int.y);
    println!("Ponto float: ({}, {})", ponto_float.x, ponto_float.y);

}

fn point_impl() {
    let p = Ponto {x: 5, y: 10};

    println!("p.x = {}", p.x());
}

fn mexer_genericos() {
    let p1 = Point {x: 5, y: 10.4};
    let p2 = Point {x: "Teste", y: "c"};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}