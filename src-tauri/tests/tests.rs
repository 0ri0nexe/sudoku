#[test]
fn new_board() {
    let sudoku = sudoku::Sudoku::new(true).get_playable_sudoku(0.5);
    println!("{}", sudoku);
}
