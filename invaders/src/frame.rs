pub type Frame = Vec<Vec<&'static str>>;

pub fn new_frame(columns: usize, rows: usize) -> Frame {
    let mut cols = Vec::with_capacity(columns);
    for _ in 0..columns {
        let mut col = Vec::with_capacity(rows);
        for _ in 0..rows {
            col.push(" ");
        }
        cols.push(col);
    }
    cols
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
