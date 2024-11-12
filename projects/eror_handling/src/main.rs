use std::io::{self, Read};
use std::fs::File;
use std::result;

fn main() {
    let v = vec![1, 2, 3];
    println!("Elemento: {}", v[99]);

    let divisor = 0;
    if divisor == 0{
        panic!("Divisao por zero nao é permitida");
    }


    //Recovery Error

    let file = File::open("aquivo.txt");

    match file {
        Ok(file) => println!("Arquivo aberto com sucesso"),
        Err(e) => println!("Erro ao abrir o arquivo: {}", e),
    }

    let file = File::open("aquivo.txt").unwrap();

    match_int_operator();
    
    /*Panic ou nao */
    dividir(10, 0); // Isso Causara um panic!

    //Usando Result
    match dividir_seguro(10, 0) {
        Ok(result) => println!("Resultado: {}", result),
        Err(err) => println!("{}", err),
    }

}


fn read_file() -> Result<String, io::Error> {
    let mut file = File::open("arquivo.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn match_int_operator(){
    match read_file() {
        Ok(c) => println!("Conteúdo: {}", c),
        Err(e) => println!("Erro: {:?}", e),
    }
}

fn dividir(a: i32, b: i32) -> i32 {
    if b==0{
        panic!("Divisão por zero");
    } else {
        a / b
    }
}

fn dividir_seguro(a: i32, b:i32) -> Result<i32, String> {
    if b==0 {
        Err(String::from("Error: Divisão por zero"))
    } else {
        Ok(a / b)
    }
}