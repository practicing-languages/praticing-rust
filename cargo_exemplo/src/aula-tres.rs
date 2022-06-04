const PI:f32 = 3.14;
static mut GLOBAL:u8 = 1;

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


	let variavel:i32 = 128;
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
	//println!("decimal = {}", decimal);
}
