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
        Err(E) => println("Erro ao abrir o arquivo: {}", e),
    }

    let file = File::open("aquivo.txt").unwrap();


    match read_file() {
        Ok(c) => println!("Conteúdo: {}", c),
        Err(e) => println!("Erro: {:?}", e),
    }
}


fn read_file() -> Result<String, io::Error> {
    let mut file = File::open("arquivo.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}