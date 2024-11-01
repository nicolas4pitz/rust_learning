fn main() {

    //Tipos Floats
    let _x = 2.0; // f64

    let _y: f32 = 3.0; // f32


    //Operações Numéricas
    // adição
    let soma = 5 + 10;

    // subtração
    let diferenca = 95.5 - 4.3;

    // multiplicação
    let produto = 4 * 30;

    // divisão
    let quociente = 56.7 / 32.2;

    // resto
    let resto = 43 % 5;

    //Tipo Booleano
    let t = true;

    let f: bool = false; // com tipo explícito


    //Tipo de caractere/Unicode
    let c = 'z'; //Expressado em aspas simples
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    //Tipos compostos
    //Tuplas
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("O valor do y é: {}", y);

    let quinhentos = x.0;

    let seis_ponto_quatro = x.1;

    let um = x.2;


    //Matriz
    let a = [1, 2, 3, 4, 5];
    let meses = ["Janeiro", "Fevereiro", "Março", "Abril", "Maio", "Junho", "Julho",
                "Agosto", "Setembro", "Outubro", "Novembro", "Dezembro"];
    
    let primeiro = a[0];
    let segundo = a[1];

    

}
