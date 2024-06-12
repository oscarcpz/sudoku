
fn main() { 
    let numero1 = 5;
    let mut numero2 = 6;
    let puntero1 = &numero1 as *const i32; 
    let puntero2 = &numero1;
    let puntero3 = &mut numero2;
    println!("Address puntero1: 0x{:x}", puntero1 as usize); 
    println!("Address puntero2: 0x{:x}", puntero2); 
    println!("Address puntero3: 0x{:x}", puntero3); 
    println!("Numero1: {}", numero1);
    //println!("Numero2: {}", numero2);
    unsafe {
        println!("Valor puntero: {}", *puntero1);
    }
    println!("Valor puntero 2: {}", *puntero2);
    println!("Valor puntero 3: {}", *puntero3);
    unsafe {
        *puntero3 = *puntero3 + *puntero1;
    }
    println!("Nuevo valor puntero 3: {}", *puntero3);

    let mut array: [i8; 4] = [1, 2, 3, 4];
    let mut puntero_array = &mut array[2];

    //println!("Valor array: {:?}", array);
    //println!("Valor puntero_array: {}", puntero_array);

    *puntero_array = *puntero_array + 10;
    println!("Valor nuevo de puntero_array: {}", puntero_array);
    println!("Valor array: {:?}", array);
}