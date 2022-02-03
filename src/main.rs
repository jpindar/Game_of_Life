// Conway's Game of Life
mod tests;

const SIZE: usize = 8;

pub struct World {
    map: [[u8; SIZE + 2]; SIZE + 2], // leave a border of 1 cell around the map
}
impl World {
    pub fn new() -> World {
        World {
            map: [[0; SIZE + 2]; SIZE + 2], // initialize map with 0s
        }
    }

    fn show(&self) {
        for i in 1..=SIZE {
            for j in 1..=SIZE {
                if self.map[i][j] == 1 {
                    print!("#");
                } else {
                    print!(".");
                }
                //print!("{}", self.map[i][j]);
            }
            println!("");
        }
        println!();
    }

    fn randomize(&mut self) {
        let density = 0.2; // fraction of cells that will be alive
        for i in 1..=SIZE {
            for j in 1..=SIZE {
                self.map[i][j] = if (rand::random::<u8>() < (density * 256.0) as u8) {
                    1
                } else {
                    0
                };
            }
        }
    }

    fn update(&mut self) {
        let mut new_world = World::new(); // initialize new map with 0s
        for i in 1..=SIZE {
            for j in 1..=SIZE {
                let mut count = 0;
                count += self.map[i - 1][j];
                count += self.map[i - 1][j - 1];
                count += self.map[i - 1][j + 1];
                count += self.map[i + 1][j - 1];
                count += self.map[i + 1][j];
                count += self.map[i + 1][j + 1];
                count += self.map[i][j - 1];
                count += self.map[i][j + 1];

                if (self.map[i][j] == 1) && (count == 2 || count == 3) {
                    new_world.map[i][j] = 1;
                }
                if (self.map[i][j] == 0) && (count == 3) {
                    new_world.map[i][j] = 1;
                }
            }
        }
        self.map = new_world.map;
    }
}

fn specify_world(world: &mut World) {
    world.map = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
        [0, 1, 1, 1, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];
}

fn main() {
    let mut world = World::new();
    // world.randomize();
    specify_world(&mut world);
    world.show();

    for i in 0..8 {
        world.update();
        world.show();
    }
}
