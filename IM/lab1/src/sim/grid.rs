pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<crate::sim::cell::Cell>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![crate::sim::cell::Cell::empty(); width * height],
        }
    }

    pub fn n4(idx: usize, w: usize, h: usize) -> [usize; 4] {
        let x = idx % w;
        let y = idx / w;
        let up = ((y + h - 1) % h) * w + x;
        let down = ((y + 1) % h) * w + x;
        let left = y * w + ((x + w - 1) % w);
        let right = y * w + ((x + 1) % w);
        [up, left, right, down] // вверх → влево → вправо → вниз
    }

    pub fn n8(idx: usize, w: usize, h: usize) -> [usize; 8] {
        let x = idx % w;
        let y = idx / w;
        let up = (y + h - 1) % h;
        let down = (y + 1) % h;
        let left = (x + w - 1) % w;
        let right = (x + 1) % w;

        [
            up * w + x,
            down * w + x,
            y * w + left,
            y * w + right, // Грани
            up * w + left,
            up * w + right,
            down * w + left,
            down * w + right, // Углы
        ]
    }
}
