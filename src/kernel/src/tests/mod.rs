// use crate::qemu::{exit_qemu, QemuExitCode};
// pub mod qemu;

#[cfg(kfs_test)]
use crate::print;
use crate::println;

/// Fonction appelée depuis kernel.rs pour lancer les tests
#[cfg(kfs_test)]
pub fn run_tests() {
    println!("=== Running KFS Tests ===");

    // Liste des tests à exécuter
    let tests: &[(&str, fn())] = &[
        ("trivial_assertion", trivial_assertion),
        ("test_addition", test_addition),
    ];

    println!("Running {} tests", tests.len());
    for (name, test_fn) in tests {
        print!("{}... ", name);
        test_fn();
        println!("[ok]");
    }

    println!("=== All tests passed! ===");
    // exit_qemu(QemuExitCode::Success);
}

#[cfg(kfs_test)]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[cfg(kfs_test)]
fn test_addition() {
    assert_eq!(2 + 2, 4);
}

// #[test_case]
// fn test_vga_color_code() {
//     use crate::{println, print};
//     print!("test_vga_color_code... "); // -> ici crash car cargo test run or pas dans une vm et donc on a pas acces au vga
//     use crate::vga_buffer::{Color, ColorCode};
//     let code = ColorCode::new(Color::White, Color::Black);
//     assert_eq!(code.0, 0x0F); // White (15) sur Black (0) = 0x0F
//     println!("[ok]");
// }
