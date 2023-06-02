mod library;
// mod lifetime;
// mod matrix_transpose;

/// Determine whether the first argument is divisible by the second argument.
///
/// If the second argument is zero, the result is false.
// fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
//     if rhs == 0 {
//         return false; // Corner case, early return
//     }
//     lhs % rhs == 0 // The last expression in a block is the return value
// }

fn main() {
    // // Program entry point
    // let mut x = 10; // Mutable variable binding
    // print!("{}", x); // Macro for printing, like printf

    // while x != 1 {
    //     // No parenthesis around expression
    //     if x % 2 == 0 {
    //         // Math like in other languages
    //         x = x / 2;
    //     } else {
    //         x = 3 * x + 1;
    //     }
    //     print!(" -> {x}");
    // }

    // is_divisible_by(20, 2);
    // println!();

    // matrix_transpose::run_transpose();
    // matrix_transpose::run_transpose_generic();
    // lifetime::life_time();
    // lifetime::life_time_different_scope();

    library::library_runner()
}
