#[cfg(test)]
use crate::{print};

use crate::{println};

/// Test runner qui n'utilise pas le VGA buffer
// #[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    // On ne peut pas utiliser println! car il accède au VGA buffer
    // Les tests vont simplement paniquer s'ils échouent
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    println!("All tests passed!");
    // Si on arrive ici, tous les tests ont réussi
    // On sort avec un code de succès (dans un vrai environnement de test)
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... "); // -> ici crash car cargo test run or pas dans une vm et donc on a pas acces au vga
    assert_eq!(1, 1);
    println!("[ok]");
}

#[test_case]
fn test_addition() {
    print!("test addition... "); // -> ici crash car cargo test run or pas dans une vm et donc on a pas acces au vga
    assert_eq!(2 + 2, 4);
    println!("[ok]");
}

#[test_case]
fn test_vga_color_code() {
    use crate::{println, print};
    print!("test_vga_color_code... "); // -> ici crash car cargo test run or pas dans une vm et donc on a pas acces au vga
    use crate::vga_buffer::{Color, ColorCode};
    let code = ColorCode::new(Color::White, Color::Black);
    assert_eq!(code.0, 0x0F); // White (15) sur Black (0) = 0x0F
    println!("[ok]");
}
