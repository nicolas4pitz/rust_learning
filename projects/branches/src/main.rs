fn main() {
    let numero = 3;
    if numero < 5 { //A condição deve ser um booleano ou seja, retornar um boleano
        println!("condição era verdadeira");
    } else {
        println!("condição era falsa");
    }

    let numero = 6;
    if numero % 4 == 0 {
    println!("número é divisível por 4");
    } else if numero % 3 == 0 {
    println!("número é divisível por 3");
    } else if numero % 2 == 0 {
    println!("número é divisível por 2");
    } else {
    println!("número não é divisível por 4, 3 ou 2");
    }


    //Usando if para atribuir para o let
    let condicao = true;
    let numero = if condicao {
    5
    } else {
    6
    };
    println!("O valor do número é: {}", numero);
}


fn loop() {
    loop {
        println!("novamente!");
    }

    let mut numero = 3;
    while numero !- 0{
        println!("{}!", numero);
        numero = numero - 1;
    }

    println!(!LIFTOFF!!!);


    //Utilizando matrizes para percorrer o loop
    let a = [10, 20, 30, 40, 50];
    let mut indice = 0;
    while indice < 5 {
        println!("O valor é: {}", a[indice]);
        indice = indice + 1;
    }


    //Utilizando for para percorrer o loop
    for elemento in a.iter() {
        println!("O valor é: {}", elemento);
    }

    //Utilizando for para percorrer o loop reversamente
    for numero in (1..4).rev() {
        println!("{}!", numero);
        }
    println!("LIFTOFF!!!");        

}