// Conway's Game of Life

const SIZE: usize = 8;

struct World {
    map: [[u8; SIZE + 2]; SIZE + 2], // leave a border of 1 cell around the map
}
impl World {
    pub fn new() -> World {
        World {
            map: [[0; SIZE + 2]; SIZE + 2], // initialize map with 0s
        }
    }

    fn show(self) {
        for i in 1..=SIZE {
            for j in 1..=SIZE {
                print!("{}", self.map[i][j]);
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
}

fn main() {
    let mut world = World::new();
    world.randomize();
    world.show();
}
