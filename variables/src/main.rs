fn main() {
    let mut x: i32 = 10;
    println!("x: {x}");
    x = 20;
    println!("x: {x}");

    let sixteen_bit_signed: i16 = -32500;
    let sixteen_bit_unsigned: u16 = 64000;

    let thirsty_two_bit_signed: i32 = -2147483648;
    let thirsty_two_bit_unsigned: u32 = 4294967295;

    //strings

    println!("Dear Emily, \nHow have you been?");
    println!("\tOnce upon a time");
    println!("Juliet said \"i love you Romeo\"");
    let filepath: &str = r"C:\My Documents\new\videos";
    println!("{filepath}");

    //Floating point

    let pi: f64 = 3.1415926535897932384;

    println!("{}", pi.floor()); // arendona numero mais proximo para baixo
    println!("{}", pi.ceil()); // aredonda numero mais proxima para proxima
    println!("{}", pi.round()); // arendoda para o mais proximo

    println!("The current value of pi is {pi:.4}");

    // Corversao de tipos

    let miles_away: i32 = 50;
    let miles_away_i8 = miles_away as i8;
    let miles_away: f64 = 100.12976;
    let miles_away_int = miles_away as i32;

    // Booleans

    let is_handsome = true;
    let is_silly = false;

    println!("Handsome: {is_handsome}. Silly: {is_silly}");

    // Inversoes de boleanos

    let age = 14;
    let idade_para_assistir_filme_adulto = age >= 18;
    let ele_pode_assitir = !idade_para_assistir_filme_adulto;

    println!("Eu tenho {age} anos. Eu posso assistir o filme? {ele_pode_assitir} ");

    // Operadores de igualdade

    println!("{}", "Coke" == "Pepsi");
    println!("{}", "Coke" != "Pepsi");

    // Operador And

    let comprou_passagem = false;
    let chegou_no_horario_voo = true;

    let pode_voar = comprou_passagem && chegou_no_horario_voo;

    println!("O passageiro pode voar? {pode_voar}");

    // Operador Or/ ou

    let usuario_se_inscreveu = true;
    let usuario_admin = true;
    let usuario_tem_experiencia_premium = usuario_se_inscreveu || usuario_admin;

    println!("O usuario tem acesso ao premium?{usuario_tem_experiencia_premium}");

    // Characters

    let first_initial: char = 'B';
    let emoji: char = '😂';

    println!("{first_initial}");
    println!("{emoji}");

    // arrays

    let numbers: [i32; 5] = [4, 8, 18, 19, 25];

    let names = ["Marcos", "Joao", "Pedro"];

    let currency_rates: [f64; 0] = [];

    let mut seasons = ["Spring", "Summer", " Fall", "Winter"];

    println!("{}", seasons[2]);
    seasons[2] = "Autumn";
    println!("{}", seasons[2]);

    // imprimir array

    println!("{seasons:?}");

    // imprimir de forma mais bonita

    println!("{seasons:#?}");

    // imprimir na tela rapido para ajudar no desenvolvimento

    dbg!(seasons);

    // tuplas

    let empresa = ("Molly", 32, "Marketing");
    let (name, idade, departamento) = empresa;

    println!("Name: {name}, idade: {idade}, departamento: {departamento}");

    println!("{empresa:#?}");

    // ranges --- intervalos

    let numeros = 1..31; // dessa forma sera 1 ao 30, pois excluiu o ultimo numero

    let month_days = 1..=31; // dessa forma 1 ao 31

    // forma de uso ranges

    for numbers in numeros {
        println!("{numbers}");
    }
}
