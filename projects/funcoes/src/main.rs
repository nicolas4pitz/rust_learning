fn main() {
    println!("Hello, world!");

    outra_func(5);

    x = cinco(); //Consegue atribuir ao 5

    println!("O valor de x é: {}", x); //Retorna 5
}

fn outra_func(x: i32){
    println!("O valor de x é: {}", x);
}

fn declaracao(){
    let y: i32 = 5; //Declaração
    let x_dont_get_any_return_from_y = (let y_dont_return_a_answer = 6); 
}

fn espressao(){
    let x = 5; 
    let y = { //Expressão
        let x = 3;
        x + 1
    }; //Adicionando um ponto e virgula ao final, você a transforma em uma declaração, que então não retornará um valor
    println!("O valor de y é: {}", y);
}

fn cinco() -> i32 {
    5 //Não pode colocar ; no final, pois isso transforma a expressão em uma declaração
}
