extern crate rand;

use std::io; //input/output
use std::cmp::Ordering; //comparação

use rand::Rng; //gerador de numeros aleatorios

fn main() {
    println!("Adivinhe o numero!");

    let numero_secreto = rand::thread_rng().gen_range(1, 101);

    println!("O numero secreto é {}", numero_secreto);

    loop {
        println!("Digite o seu palpite:");

        let mut palpite = String::new();

        io::stdin().read_line(&mut palpite) //Standard input = stdin -> Stdin
            .expect("Falha ao ler a entrada");

        let palpite: u32 = match palpite.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Você disse: {}", palpite);

        match palpite.cmp(&numero_secreto) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
                println!("Você acertou!");
                break;
            }
        }
    }
}
