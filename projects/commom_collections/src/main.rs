enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
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

}
