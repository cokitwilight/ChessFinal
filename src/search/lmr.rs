pub fn lmr_reduction(depth: usize, move_index: usize) -> usize {
    if depth < 3 || move_index < 3 {
        return 0;
    }

    let r = match (depth, move_index) {
        (16.., 8..) => 7,
        (14.., 12..) => 6,
        (12.., 10..) => 5,
        (10.., 14..) => 4,
        (8.., 12..) => 3,
        (6.., 7..) => 2,
        (4.., 5..) => 1,
        _ => 0,
    };

    r.min(depth - 2)
}
