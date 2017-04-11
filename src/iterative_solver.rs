use super::*;

pub fn next_move(n: u64, board: &Board) -> Option<(usize, usize)> {
    if board.is_finished() {
        None
    } else {
        calculate_next(n, board)
    }
}

fn calculate_next(n: u64, board: &Board) -> Option<(usize, usize)> {
    let num_disks = board.num_disks();
    let (aux_pole, dest_pole) = if num_disks % 2 == 1 { (1, 2) } else { (2, 1) };

    let r = n % 3;
    match r {
        1 => legal_movement_between(0, dest_pole, board),
        2 => legal_movement_between(0, aux_pole, board),
        _ => legal_movement_between(aux_pole, dest_pole, board),
    }
}

fn legal_movement_between(a: usize, b: usize, board: &Board) -> Option<(usize, usize)> {
    if a > 2 || b > 2 {
        return None;
    }

    let disk_a = board.pegs()[a].peek();
    let disk_b = board.pegs()[b].peek();

    match (disk_a, disk_b) {
        (Some(disk_a), Some(disk_b)) if disk_a > disk_b => Some((b, a)),
        (Some(_), Some(_)) => Some((a, b)),
        (Some(_), None) => Some((a, b)),
        (None, Some(_)) => Some((b, a)),
        (None, None) => None,
    }
}
