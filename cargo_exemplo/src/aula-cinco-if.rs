const PI:f32 = 3.14;
static mut GLOBAL:u8 = 1;

fn soma(a:i32, b:i32) -> i32 {
	println!("{} + {} = {}", a, b, a + b);
	a + b
}


//fn soma(a:i32, b:i32) -> i32 {
//	println!("{} + {} = {}", a, b, a + b);
//
//	return a + b;
//}

fn sombra(){
	let a = 123;

	{
		let b = 456;
		println!("dentro, b = {}", b);

		let a = 777;
		println!("dentro, a = {}", a);
	}

	println!("fora, a = {}", a);
}

fn escopo(){
	println!("PI = {}", PI);

	unsafe { 
		println!("Variavel global = {}", GLOBAL);
	}


	// let variavel:i32 = 128;
	let variavel:i32 = 301;
	println!("Aula de rust 2");
	println!("a variavel e = {}, tamanho da variavel = {} bytes", variavel, std::mem::size_of_val(&variavel));
	
	let decimal:f32 = 2.5;
	println!("decimal = {}", decimal);

	let booleana:bool = true;

	println!("booleana value = {}, tamanho do boolean = {}", std::mem::size_of_val(&booleana), booleana);

	let letra:char = 'C';
	println!("tamanho do char = {}, valor atribuido ao char = {}", std::mem::size_of_val(&letra), letra);
}

fn main(){
	escopo();
	sombra();

	let result:i32 = soma(4, 3);
	println!("O resultado da soma = {}", result);	

	println!("Soma = {}", soma(2, 2));

	//println!("decimal = {}", decimal);

    condicionais();
}

fn condicionais(){    
    let idade:u8 = 17;
    let responsavel_autorizou = true;
    let is_maior = idade >= 18;

    if is_maior {
        println!("Pode entrar na balada!");
    }else if idade > 16 && responsavel_autorizou {
        println!("Pode entrar com a assinatura do responsavel!");
    }else {
        println!("Nao pode entrar na balada!");
    }

    let condicao = if is_maior { "maior" }else { "menor" };

    println!("é {} de idade", condicao);
}