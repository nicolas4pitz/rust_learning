// fn main() {
//     referencias_mutaveis_main();

//     /*let s1 = String::from("texto");
//     let s2 = s1.clone();

//     toma_posse(s1);// move o valor de s para dentro da função...

//     println!("s1 = {}, s2 = {}", s1, s2);

//     let x = 5; 

//     faz_uma_copia(x);*/
// }

// fn toma_posse(uma_string: String) { 
//     //uma_string entra em escopo. 
//     println!("{}", uma_string);
// } // Aqui, uma_string sai de escopo, e o método `drop` é chamado. A memória que // guarda seus dados é liberada.
// fn faz_uma_copia(um_inteiro: i32) { // um_inteiro entra em escopo. 
//     println!("{}", um_inteiro);
// } // Aqui, um_inteiro sai de escopo. Nada de especial acontece.


// fn chamar_main() {
//     let s1 = entrega_valor();           // entrega_valor move o valor retornado
//                                         // para s1.

//     let s2 = String::from("texto");     // s2 entra em escopo.

//     let s3 = pega_e_entrega_valor(s2);  // s2 é movido para dentro da função
//                                         // pega_e_entrega_valor, que também
//                                         // move o valor retornado para s3.
// } // Aqui, s3 sai de escopo e é destruída. s2 sai de escopo, mas já foi movida,
//   // então nada demais acontece. s1 sai de escopo e é destruída.

// fn entrega_valor() -> String {               // entrega_valor move o valor
//                                              // retornado para dentro da função
//                                              // que a chamou.

//     let uma_string = String::from("olá");    // uma_string entra em escopo.

//     uma_string                               // uma_string é retornada e movida
//                                              // para a função que chamou
//                                              // entrega_valor.
// }

// // pega_e_entrega_valor vai pegar uma String e retorná-la.
// fn pega_e_entrega_valor(uma_string: String) -> String { // uma_string entra em
//                                                         // escopo.

//     uma_string  // uma_string é retornada e movida para a função que chamou
//                 // pega_e_entrega_valor.
// }


// //Referencias mutaveis

// fn referencias_mutaveis_main() {
//     let mut s = String::from("texto");

//     modifica(&mut s);
// }

// fn modifica(uma_string: &mut String) {
//     uma_string.push_str(" longo");
// }


// fn main(){
//     let mut s = String::from("texto");

    
//     let _r1 = &s;
//     let _r2 = &s;
//     //let _r3 = &mut s;

//     println!("{}", _r1);
//     println!("{}", _r2);
//     //println!("{}", _r3);
// }

//SLices ------------------------------

// fn main(){
//     let s = String::from("texto longo");

//     let range_das_primeira_palavra = primeira_palavra(&s); //valor 5
//     let slice_string = &s[range_das_primeira_palavra..]; //FAz o sliceString, posicao(byte) 5 até o final

//     println!("{}", slice_string);
//     println!("{}", range_das_primeira_palavra);
// }

// fn primeira_palavra(s: &String) -> usize { //Retorna a posicao do primeiro espaço
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

// fn primeira_palavra(s: &String) -> &str{ //Reestruturamos para retornar um slice
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.inter().enumerate() {
//         if item == b' ' {
//             return &s[0..i]; //Retorna uma referencia para a string, do inicio até a posicao i
//     }

//     &s[..]
// }
// }


//Strings literais são slices

//let s = "Olá, mundo!";

//Slices de string como parametros

// fn main(){
//     let minha_string = String::from("texto longo");

//     //slices de String
//     let palavra = primeira_palavra(&minha_string[..]);


//     let minha_string_literal = "texto longo";

//     //string literal
//     let palavra = primeira_palavra(&minha_string_literal[..]);


// }

// fn primeira_palavra(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i]; //Retorna uma referencia para a string, do inicio até a posicao i
//         }
//     }

//     return &s[..]
// }
