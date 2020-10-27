enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_row_iter(
    matrix: &Vec<Vec<i32>>,
    row: usize,
    left_col: usize,
    right_col: usize,
) -> impl Iterator<Item = &i32> {
    matrix[row][left_col..right_col].iter()
}

fn get_col_iter(
    matrix: &Vec<Vec<i32>>,
    top_row: usize,
    bot_row: usize,
    col: usize,
) -> impl Iterator<Item = &i32> {
    matrix[top_row..bot_row]
        .iter()
        .filter_map(move |row| row.iter().nth(col))
}

struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() {
            return Vec::new();
        }

        let rows = matrix.len();
        let cols = matrix[0].len();
        let total_size = rows * cols;

        let mut out = Vec::<i32>::with_capacity(total_size);

        let mut direction = Direction::Right;
        let mut left_col = 0;
        let mut right_col = cols;
        let mut top_row = 0;
        let mut bot_row = rows;

        while out.len() < total_size {
            direction = match direction {
                Direction::Right => {
                    out.extend(get_row_iter(&matrix, top_row, left_col, right_col));
                    top_row += 1;
                    Direction::Down
                }
                Direction::Down => {
                    out.extend(get_col_iter(&matrix, top_row, bot_row, right_col - 1));
                    right_col -= 1;
                    Direction::Left
                }
                Direction::Left => {
                    out.extend(
                        get_row_iter(&matrix, bot_row - 1, left_col, right_col)
                            .collect::<Vec<&i32>>()
                            .into_iter()
                            .rev(),
                    );
                    bot_row -= 1;
                    Direction::Up
                }
                Direction::Up => {
                    out.extend(
                        get_col_iter(&matrix, top_row, bot_row, left_col)
                            .collect::<Vec<&i32>>()
                            .into_iter()
                            .rev()
                    );
                    left_col += 1;
                    Direction::Right
                }
            }
        }

        out
    }
}

fn main() {
    let row1: Vec<i32> = vec![1, 2, 3];
    let row2: Vec<i32> = vec![4, 5, 6];
    let row3: Vec<i32> = vec![7, 8, 9];
    let matrix = vec![row1, row2, row3];
    println!("{:?}", Solution::spiral_order(matrix));
}
