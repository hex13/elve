struct TileMap<Tile> {
    width: usize,
    height: usize,
    data: Vec<Tile>,
    default_value: Tile,
}

impl<Tile> TileMap<Tile> where Tile: Clone {
    fn new(width: usize, height: usize, default_value: Tile) -> TileMap<Tile> {
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
    fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        return Some(&self.data[self.idx(x, y)?]);
    }
    fn set(&mut self, x: usize, y: usize, value: Tile) -> Result<(),()> {
        let idx = self.idx(x, y).ok_or(())?;
        self.data[idx] = value;
        Ok(())
    }
}

mod tests {
    use super::*;
    #[test]
    fn tile_map_creation() {
        let tile_map = TileMap::new(3, 2, 0);
        assert_eq!(tile_map.width, 3);
        assert_eq!(tile_map.height, 2);
    }
    #[test]
    fn tile_map_get_default_value() {
        let tile_map = TileMap::new(3, 2, 123);
        assert_eq!(tile_map.get(0, 0), Some(&123));
    }
    #[test]
    fn tile_map_set_then_get() {
        let mut tile_map = TileMap::new(20, 10, 123);
        let x = 3;
        let y = 5;
        tile_map.set(x, y, 1000);
        tile_map.set(0, 0, 2000);
        assert_eq!(tile_map.get(x, y), Some(&1000));
        assert_eq!(tile_map.get(0, 0), Some(&2000));
    }
    #[test]
    fn get_out_of_bounds() {
        let mut tile_map = TileMap::new(20, 10, 123);
        assert_eq!(tile_map.get(30, 1), None);
        assert_eq!(tile_map.get(1, 30), None);
    }
    #[test]
    fn set_out_of_bounds() {
        let mut tile_map = TileMap::new(20, 10, 123);
        let res = tile_map.set(30, 1, 100);
        assert_eq!(res, Err(()));
        let res = tile_map.set(1, 15, 100);
        assert_eq!(res, Err(()));

        // in bounds
        let res = tile_map.set(1, 3, 100);
        assert_eq!(res, Ok(()));
    }

}
