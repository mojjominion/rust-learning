use std::convert::AsRef;
use std::fmt::Debug;

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result = matrix;

    for j in 0..3 {
        for i in 0..3 {
            result[i][j] = matrix[j][i]
        }
    }
    result
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for row in matrix {
        for cell in row {
            print!("{cell} ")
        }
        print!("\n")
    }
    print!("\n")
}

pub(crate) fn run_transpose() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}

fn pretty_print_generic<Cell, Row, Matrix>(matrix: Matrix)
where
    Cell: Debug,
    Row: AsRef<[Cell]>,
    Matrix: AsRef<[Row]>,
{
    for row in matrix.as_ref() {
        println!("{:?}", row.as_ref());
    }
}

pub(crate) fn run_transpose_generic() {
    // &[&[i32]]
    pretty_print_generic(&[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]]);
    // [[&str; 2]; 2]
    pretty_print_generic([["a", "b"], ["c", "d"]]);
    // Vec<Vec<i32>>
    pretty_print_generic(vec![vec![1, 2], vec![3, 4]]);
}
