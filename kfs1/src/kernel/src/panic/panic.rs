use core::panic::PanicInfo;
use crate::println;

// Panic handler pour le mode normal
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// Panic handler pour les tests
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[failed]\n");
    println!("Error: {}", info);
    // On ne peut pas utiliser println! ici car VGA buffer n'est pas dispo
    // Le test va juste crasher et cargo test va le détecter

    loop {}
}