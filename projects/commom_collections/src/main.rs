use std::{io, vec};
use std::collections::HashMap;


enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    //storing_vector();

    //////
    
    //storing_utf8();

    //

    storing_hash_map();
}


fn storing_vector() {
    let mut v: Vec<i32> = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    let v2 = vec![1, 2, 3, 4, 5];

    println!("Vector 1: {:?}", v);
    println!("Vector 2: {:?}", v2);

    let v = vec![1, 2, 3];
    println!("Primeiro elemento: {:?}", v[0]);

    match &v.get(2) {
        Some(value) => println!("Elemento encontrado: {}", value),
        None => println!("Elemento não encontrado"),
    }

    for value in &v {
        println!("Value: {}", value);
    }

    ////
    
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for cell in &row {
        match cell {
            SpreadsheetCell::Int(value) => println!("Int: {}", value),
            SpreadsheetCell::Text(value) => println!("Text: {}", value),
            SpreadsheetCell::Float(value) => println!("Float: {}", value),
        }
    }

    let mut vetor:Vec<i32> = vec![];

    println!("Saia com '666'");
    loop {
        println!("Digite o numero para aumentar o vetor: ");
        let mut palpite= String::new();
        io::stdin().read_line(&mut palpite) //Standard input = stdin -> Stdin
            .expect("Falha ao ler a entrada");
        if palpite.trim() == "666" {
            break;
        }
        vetor.push(palpite.trim().parse().unwrap());
    }

    println!("Vetor: {:?}", vetor);
}

fn storing_utf8() {
    let mut s = String::from("Ola");
    s.push_str(", mundo!");
    println!("{}", s);

    let s1 = String::from("Rust");
    let s2 = String::from("Lang");

    let s3 = s1 + " " + &s2;
    println!("{}", s3);

    let s4 = format!("{} é incrivel!", s2);
    println!("{}", s4);

    let s = String::from("Olá");
    for c in s.chars() {
        println!("{}", c);
    }

}

fn storing_hash_map() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Time A"), 10);
    scores.insert(String::from("Time B"), 20);

    println!("{:?}", scores);

    for (kit, value) in &scores {
        println!("{}: {}", kit, value);
    }

    match scores.get("Time A") {
        Some(value) => println!("Pontuação: {}", value),
        None => println!("Time não encontrado"),
    }

    scores.insert(String::from("Time A"), 15);

    scores.entry(String::from("Time B")).or_insert(25);

    println!("{:?}", scores);
    
}

