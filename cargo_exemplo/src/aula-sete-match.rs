fn main (){
    compara();    

    ownership();
}

fn compara(){
    let linguagem = "JAVA";
    // let linguagem = "RUST";

    let proposito = match linguagem{
        "PHP" => "WEB",
        "KOTLIN" => "ANDROID",
        "SWIFT" => "IOS",
        "PYTHON" => "DATA SCIENCE",
        "RUST" => "PERFORMANCE",
        _ => "DESCONHECIDO"
    };

    println!("Linguagem {}, seu proposito é {}", linguagem, proposito)

}

fn ownership(){
    let uma_string = String::from("Rafael");

    let outra_string = rouba(uma_string);
    println!("{}", outra_string);
}

fn rouba(string: String) -> String {
    println!("{}", string);

    string
}