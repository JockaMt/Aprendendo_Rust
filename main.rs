fn main() {
  println!("Hello World!");
  mostrar_algo(numerical_variables());
}

// Definição de variável e retorno
fn numerical_variables() -> i8{
  let signed_1: i8 = 4; // Menor espaço na memória de numero com sinal (i) != (u)
  signed_1
}

// For loop
fn mostrar_algo(num: i8){
  for i in 0..num {
    println!("{}", i)
  }
}

