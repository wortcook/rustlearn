const N: usize = 8;

fn try_solution(mut board: &mut [[bool; N]; N], row: usize, mut count: &mut i64) {
    if row == N {
        *count += 1;
        println!("Solution #{}:\n", *count);
        for r in board.iter() {
            println!(
                "{}",
                r.iter()
                    .map(|&x| if x { "Q" } else { "." }.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            )
        }
        println!("");
        return;
    }
    for i in 0..N {
        let mut ok: bool = true;
        for j in 0..row {
            if board[j][i]
                || i + j >= row && board[j][i + j - row]
                || i + row < N + j && board[j][i + row - j]
            {
                ok = false
            }
        }
        if ok {
            board[row][i] = true;
            try_solution(&mut board, row + 1, &mut count);
            board[row][i] = false;
        }
    }
}

fn main() {
    let mut board: [[bool; N]; N] = [[false; N]; N];
    let mut count: i64 = 0;
    try_solution(&mut board, 0, &mut count);
}