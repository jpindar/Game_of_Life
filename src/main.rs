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
}

fn main() {
    let mut world = World::new();
    let density = 40;
    for i in 1..=SIZE {
        for j in 1..=SIZE {
            if rand::random::<u8>() < density {
                world.map[i][j] = 1;
            } else {
                world.map[i][j] = 0;
            }
        }
    }

    world.show();
}
