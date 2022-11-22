pub fn line_bresenham<F: FnMut(i64, i64)>(
    mut x0: i64,
    mut y0: i64,
    mut x1: i64,
    mut y1: i64,
    mut f: F,
) {
    let steep = (x1 - x0).abs() < (y1 - y0).abs();

    if steep {
        (x0, y0) = (y0, x0);
        (x1, y1) = (y1, x1);
    }

    if x0 > x1 {
        (x0, x1) = (x1, x0);
        (y0, y1) = (y1, y0);
    }

    let dx = x1 - x0;
    let dx2 = dx << 1;

    let slope = (y1 - y0).abs() << 1;
    let mut slope_sum = 0;

    let step_y = if y0 > y1 { -1 } else { 1 };

    let mut y = y0;
    for x in x0..=x1 {
        if steep {
            f(y, x);
        } else {
            f(x, y);
        }

        slope_sum += slope;

        if dx < slope_sum {
            y += step_y;
            slope_sum -= dx2;
        }
    }
}
