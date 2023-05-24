pub struct TileMap<Tile> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Tile>,
    default_value: Tile,
}

struct RadiateIterator {
    x: usize,
    y: usize,
    center_x: usize,
    center_y: usize,
    radius: usize,
    max_radius: usize,
    i: usize,
}

impl RadiateIterator {
    fn new(x: usize, y: usize, max_radius: usize) -> Self {
        RadiateIterator { center_x: x, center_y: y, x, y, radius: 0, max_radius, i: 0 }
    }
}

impl Iterator for RadiateIterator {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        let current = Some((self.x, self.y));
        if self.radius > self.max_radius {
            return None;
        }
        if self.radius == 0 || self.i >= self.radius * 8 - 1 {
            self.radius += 1;
            self.x = self.center_x - self.radius;
            self.y = self.center_y - self.radius;
            self.i = 0;
        } else {
            match self.i / (self.radius * 2) {
                0 => self.x += 1,
                1 => self.y += 1,
                2 => self.x -= 1,
                3 => self.y -= 1,
                _ => {}
            }
            self.i += 1;
        }

        return current;

    }
}

impl<Tile> TileMap<Tile> where Tile: Clone {
    pub fn new(width: usize, height: usize, default_value: Tile) -> TileMap<Tile> {
        let mut data = Vec::new();
        data.resize(width * height, default_value.clone());
        TileMap { width, height, default_value, data, }
    }
    fn idx(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(y * self.width + x)
    }
    pub fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        return Some(&self.data[self.idx(x, y)?]);
    }
    pub fn set(&mut self, x: usize, y: usize, value: Tile) -> Result<(),()> {
        let idx = self.idx(x, y).ok_or(())?;
        self.data[idx] = value;
        Ok(())
    }
    pub fn set_slice(&mut self, x: usize, y: usize, value: &[Tile]) -> Result<(),()> {
        let idx = self.idx(x, y).ok_or(())?;
        self.data.splice(idx..idx + value.len(), value.iter().cloned());
        Ok(())
    }
}


mod tests {
    use super::*;

    fn create_grid(w: usize, h: usize, default_value: i32) -> TileMap<i32> {
        TileMap::new(w, h, default_value)
    }
    #[test]
    fn tile_map_creation() {
        let tile_map = create_grid(3, 2, 0);
        assert_eq!(tile_map.width, 3);
        assert_eq!(tile_map.height, 2);
    }
    #[test]
    fn tile_map_get_default_value() {
        let tile_map = create_grid(3, 2, 123);
        assert_eq!(tile_map.get(0, 0), Some(&123));
    }
    #[test]
    fn tile_map_set_then_get() {
        let mut tile_map = create_grid(20, 10, 123);
        let x = 3;
        let y = 5;
        tile_map.set(x, y, 1000);
        tile_map.set(0, 0, 2000);
        assert_eq!(tile_map.get(x, y), Some(&1000));
        assert_eq!(tile_map.get(0, 0), Some(&2000));
    }
    #[test]
    fn get_out_of_bounds() {
        let mut tile_map = create_grid(20, 10, 123);
        assert_eq!(tile_map.get(30, 1), None);
        assert_eq!(tile_map.get(1, 30), None);
    }
    #[test]
    fn set_out_of_bounds() {
        let mut tile_map = create_grid(20, 10, 123);
        let res = tile_map.set(30, 1, 100);
        assert_eq!(res, Err(()));
        let res = tile_map.set(1, 15, 100);
        assert_eq!(res, Err(()));

        // in bounds
        let res = tile_map.set(1, 3, 100);
        assert_eq!(res, Ok(()));
    }
    #[test]
    fn radiate() {
        let radiate = |x, y, max_radius| {
            return RadiateIterator::new(x, y, max_radius);
        };

        let tile_map = create_grid(20, 20, 123);

        let v: Vec<(usize, usize)> = radiate(5, 5, 0).collect();
        assert_eq!(v[0], (5, 5));
        assert_eq!(v.len(), 1);

        let v: Vec<(usize, usize)> = radiate(5, 5, 1).collect();

        assert_eq!(v[0], (5, 5));
        assert_eq!(v[1], (4, 4));
        assert_eq!(v[2], (5, 4));
        assert_eq!(v[3], (6, 4));
        assert_eq!(v[4], (6, 5));
        assert_eq!(v[5], (6, 6));
        assert_eq!(v[6], (5, 6));
        assert_eq!(v[7], (4, 6));
        assert_eq!(v[8], (4, 5));
        assert_eq!(v.len(), 9);

        let v: Vec<(usize, usize)> = radiate(10, 10, 2).collect();
        assert_eq!(v[0], (10, 10));

        assert_eq!(v[1], (9, 9));
        assert_eq!(v[2], (10, 9));
        assert_eq!(v[3], (11, 9));
        assert_eq!(v[4], (11, 10));
        assert_eq!(v[5], (11, 11));
        assert_eq!(v[6], (10, 11));
        assert_eq!(v[7], (9, 11));
        assert_eq!(v[8], (9, 10));

        assert_eq!(v[9], (8, 8));
        assert_eq!(v[10], (9, 8));
        assert_eq!(v[11], (10, 8));
        assert_eq!(v[12], (11, 8));
        assert_eq!(v[13], (12, 8));
        assert_eq!(v[14], (12, 9));
        assert_eq!(v[15], (12, 10));
        assert_eq!(v[16], (12, 11));
        assert_eq!(v[17], (12, 12));
        assert_eq!(v[18], (11, 12));
        assert_eq!(v[19], (10, 12));
        assert_eq!(v[20], (9, 12));
        assert_eq!(v[21], (8, 12));
        assert_eq!(v[22], (8, 11));
        assert_eq!(v[23], (8, 10));
        assert_eq!(v[24], (8, 9));

        assert_eq!(v.len(), 25);

    }
}

