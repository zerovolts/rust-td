use rand::{
    distributions::{Distribution, Standard},
    seq::SliceRandom,
    Rng,
};

const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Right,
    Direction::Left,
];

pub struct EnemyPath {
    pub starting_coord: (i32, i32),
    pub path: Vec<Direction>,
}

pub struct TileMap {
    tiles: Vec<TileType>,
    width: i32,
    height: i32,
}

impl TileMap {
    pub fn new(width: i32, height: i32, tile: TileType) -> Self {
        TileMap {
            tiles: vec![tile; (width * height) as usize],
            width,
            height,
        }
    }

    pub fn get(&self, coord: (i32, i32)) -> Option<TileType> {
        if self.is_within(coord) {
            Some(self.tiles[coord_to_index(coord, self.width)])
        } else {
            None
        }
    }

    // Return the four Tiles around the given coordinate. Can be less if the
    // coordinate is on an edge/corner.
    pub fn get_neighbors(&self, coord: (i32, i32)) -> Vec<TileType> {
        let mut neighbors = vec![];
        let direction_coords = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
        for (x, y) in direction_coords {
            if x == 0 && y == 0 {
                continue;
            }
            if let Some(neighbor) = self.get((coord.0 + x, coord.1 + y)) {
                neighbors.push(neighbor)
            }
        }
        neighbors
    }

    pub fn set(&mut self, coord: (i32, i32), tile: TileType) -> Option<TileType> {
        if self.is_within(coord) {
            self.tiles[coord_to_index(coord, self.width)] = tile;
            Some(tile)
        } else {
            None
        }
    }

    pub fn is_within(&self, coord: (i32, i32)) -> bool {
        coord.0 >= 0 && coord.0 < self.width && coord.1 >= 0 && coord.1 < self.height
    }
}

fn coord_to_index(coord: (i32, i32), width: i32) -> usize {
    (coord.0 + coord.1 * width) as usize
}

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Grass,
    Rock,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0, 4) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            _ => Direction::Right,
        }
    }
}

fn direction_to_coord(dir: Direction) -> (i32, i32) {
    match dir {
        Direction::Up => (0, 1),
        Direction::Down => (0, -1),
        Direction::Right => (1, 0),
        Direction::Left => (-1, 0),
    }
}

pub fn generate_map(width: i32, height: i32) -> (TileMap, EnemyPath) {
    let mut tile_map = TileMap::new(width, height, TileType::Grass);
    let mut rng = rand::thread_rng();
    let mut path = vec![];
    let starting_coord = (rng.gen_range(0, width), rng.gen_range(0, height));
    let mut cursor_coord = starting_coord;
    let mut current_direction = *ALL_DIRECTIONS.choose(&mut rng).unwrap();
    // place first tile
    tile_map.set(cursor_coord, TileType::Rock);

    let mut reached_end = false;
    while !reached_end {
        // Filter out the directions we can't move in
        let valid_directions = ALL_DIRECTIONS
            .iter()
            .filter(|&dir| {
                let direction_coord = direction_to_coord(*dir);
                let test_coord = (
                    cursor_coord.0 + direction_coord.0,
                    cursor_coord.1 + direction_coord.1,
                );
                // Make sure we aren't overlapping a previous part of the path
                let is_not_rock = tile_map
                    .get(test_coord)
                    .map_or(false, |tile| tile != TileType::Rock);
                // Make sure we aren't touching a previous part of the path
                let is_not_rock_adjacent = tile_map
                    .get_neighbors(test_coord)
                    .iter()
                    .filter(|&tile| *tile == TileType::Rock)
                    .count()
                    <= 1;
                is_not_rock && is_not_rock_adjacent
            })
            .collect::<Vec<_>>();

        let can_move_forward = valid_directions.contains(&&current_direction);
        let r = rng.gen::<f32>();
        if can_move_forward && r < 0.6 {
            // Keep moving forward
            add_path_tile(
                &current_direction,
                &mut cursor_coord,
                &mut path,
                &mut tile_map,
            );
        } else {
            // Pivot to a random (valid) direction
            match valid_directions.choose(&mut rng) {
                Some(&random_direction) => {
                    current_direction = *random_direction;
                    add_path_tile(
                        &current_direction,
                        &mut cursor_coord,
                        &mut path,
                        &mut tile_map,
                    );
                }
                None => {
                    // No more valid directions; end the path
                    reached_end = true;
                }
            }
        }
    }
    (
        tile_map,
        EnemyPath {
            starting_coord,
            path,
        },
    )
}

fn add_path_tile(
    direction: &Direction,
    cursor_coord: &mut (i32, i32),
    path: &mut Vec<Direction>,
    tile_map: &mut TileMap,
) {
    let movement_coord = direction_to_coord(*direction);
    *cursor_coord = (
        (*cursor_coord).0 + movement_coord.0,
        (*cursor_coord).1 + movement_coord.1,
    );
    path.push(*direction);
    tile_map.set(*cursor_coord, TileType::Rock);
}
