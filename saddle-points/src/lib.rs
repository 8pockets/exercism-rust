pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut points = Vec::new();

    for (y, row) in input.iter().enumerate() {
        for (x, v) in row.iter().enumerate() {
            let mut col = input.iter().map(|row| row[x]);
            if row.iter().all(|w| v >= w) && col.all(|w| *v <= w) {
                points.push((y, x));
            }
        }
    }
    points
}
