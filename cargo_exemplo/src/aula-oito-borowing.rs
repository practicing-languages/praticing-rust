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
    let mut uma_string = String::from("Rafael");

    rouba(&mut uma_string);
    println!("{}", uma_string);
}

fn rouba(string: &mut String) {
    string.push_str(" Dias");
    println!("{}", string);
}