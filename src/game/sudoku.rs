const BOARD_SIZE: usize = 9;
type CvtFn = fn([usize; 2]) -> usize;

fn cvt_row(coord: [usize; 2]) -> usize {
    coord[0]
}
const CVT_ROW: CvtFn = cvt_row;
fn cvt_col(coord: [usize; 2]) -> usize {
    coord[1]
}
const CVT_COL: CvtFn = cvt_col;
fn cvt_blk(coord: [usize; 2]) -> usize {
    (coord[0] / 3) * 3 + (coord[1] / 3)
}
const CVT_BLK: CvtFn = cvt_blk;

fn backtrack(
    vec: &mut Vec<u32>,
    cell_vec: &mut Vec<[usize; 2]>,
    cvt_vec: [CvtFn; 3],
    constraint_vec: &mut [[[bool; BOARD_SIZE]; BOARD_SIZE]; 3],
) -> Option<Vec<u32>> {
    let [i, j] = if let Some(inner) = cell_vec.pop() {
        inner
    } else {
        let ret = vec.clone().into_iter().rev().collect::<Vec<_>>();
        return Some(ret);
    };

    let id_vec = cvt_vec.clone().map(|f| f([i, j]));

    for val in 0..BOARD_SIZE {
        let mut avail = true;
        for (visited, &id) in constraint_vec.iter_mut().zip(id_vec.iter()) {
            if visited[id][val] {
                avail = false;
                break;
            }
        }
        if !avail {
            continue;
        }

        // we can pick val
        vec.push(val as u32);
        {
            for (visited, &id) in constraint_vec.iter_mut().zip(id_vec.iter()) {
                visited[id][val] = true;
            }

            if let Some(ret) = backtrack(vec, cell_vec, cvt_vec, constraint_vec) {
                return Some(ret);
            }

            for (visited, &id) in constraint_vec.iter_mut().zip(id_vec.iter()) {
                visited[id][val] = false;
            }
        }
        vec.pop();
    }

    cell_vec.push([i, j]);

    None
}

// Given an incomplete Sudoku board, attempt to solve it.
// Constraints:
//   values in the board are between [0, 9).
pub fn sudoku(board: Vec<Vec<Option<u32>>>) -> Option<Vec<Vec<u32>>> {
    let mut cell_vec = vec![];
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            if board[i][j].is_some() {
                continue;
            }
            cell_vec.push([i, j]);
        }
    }

    let cvt_vec = [CVT_ROW, CVT_COL, CVT_BLK];
    let mut constraint_vec = [
        [[false; BOARD_SIZE]; BOARD_SIZE],
        [[false; BOARD_SIZE]; BOARD_SIZE],
        [[false; BOARD_SIZE]; BOARD_SIZE],
    ];
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            if let Some(val) = board[i][j] {
                for (visited, f) in constraint_vec.iter_mut().zip(cvt_vec.clone().into_iter()) {
                    visited[f([i, j])][val as usize] = true;
                }
            }
        }
    }

    let ret = backtrack(
        &mut vec![],
        &mut cell_vec.clone(),
        cvt_vec,
        &mut constraint_vec,
    )?;

    let mut board = board
        .into_iter()
        .map(|v| {
            v.into_iter()
                .map(|e| e.unwrap_or(u32::MIN))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for ([i, j], val) in cell_vec.into_iter().zip(ret.into_iter()) {
        board[i][j] = val;
    }

    Some(board)
}
