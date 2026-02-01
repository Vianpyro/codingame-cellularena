use std::io;
use std::collections::{HashMap, VecDeque};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Grow and multiply your organisms to end up larger than your opponent.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let width = parse_input!(inputs[0], u8); // columns in the game grid
    let height = parse_input!(inputs[1], u8); // rows in the game grid

    let mut game_state = GameState::new(width, height);

    // game loop
    loop {
        game_state.my_organs.clear();

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let entity_count = parse_input!(input_line, u16);


        for i in 0..entity_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let x = parse_input!(inputs[0], u8);
            let y = parse_input!(inputs[1], u8); // grid coordinate
            let cell_type = CellType::from_string(inputs[2].trim()); // WALL, ROOT, BASIC, TENTACLE, HARVESTER, SPORER, A, B, C, D
            let owner = Owner::from_i8(parse_input!(inputs[3], i8)); // 1 if your organ, 0 if enemy organ, -1 if neither
            let organ_id = parse_input!(inputs[4], u16); // id of this entity if it's an organ, 0 otherwise
            let _organ_dir = inputs[5].trim(); // N,E,S,W or X if not an organ
            let _organ_parent_id = parse_input!(inputs[6], u16);
            let _organ_root_id = parse_input!(inputs[7], u16);

            game_state.update_cell(x, y, cell_type, owner, organ_id);
        }

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let my_a = parse_input!(inputs[0], u16);
        let my_b = parse_input!(inputs[1], u16);
        let my_c = parse_input!(inputs[2], u16);
        let my_d = parse_input!(inputs[3], u16); // your protein stock

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let opp_a = parse_input!(inputs[0], u16);
        let opp_b = parse_input!(inputs[1], u16);
        let opp_c = parse_input!(inputs[2], u16);
        let opp_d = parse_input!(inputs[3], u16); // opponent's protein stock

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let required_actions_count = parse_input!(input_line, u8); // your number of organisms, output an action for each one in any order

        for i in 0..required_actions_count {
            if my_a > 0 {
                if let Some(&(root_x, root_y)) = game_state.my_organs.get(&game_state.my_root_id) {
                    if let Some((target_x, target_y)) = game_state.find_closest_protein(root_x, root_y, CellType::ProteinA) {
                        println!("GROW {} {} {} BASIC", game_state.my_root_id, target_x, target_y);
                    } else {
                        // No protein found, just expand in any direction
                        println!("WAIT");
                    }
                } else {
                    println!("WAIT");
                }
            } else {
                println!("WAIT");
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Owner {
    Me,
    Opponent,
    Neutral,
}

impl Owner {
    fn from_i8(value: i8) -> Self {
        match value {
            1 => Owner::Me,
            0 => Owner::Opponent,
            _ => Owner::Neutral,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum CellType {
    Empty,
    Wall,
    Root,
    Basic,
    Tentacle,
    Harvester,
    Sporer,
    ProteinA,
    ProteinB,
    ProteinC,
    ProteinD
}

impl CellType {
    fn from_string(s: &str) -> Self {
        match s {
            "WALL" => CellType::Wall,
            "ROOT" => CellType::Root,
            "BASIC" => CellType::Basic,
            "TENTACLE" => CellType::Tentacle,
            "HARVESTER" => CellType::Harvester,
            "SPORER" => CellType::Sporer,
            "A" => CellType::ProteinA,
            "B" => CellType::ProteinB,
            "C" => CellType::ProteinC,
            "D" => CellType::ProteinD,
            _ => CellType::Empty,
        }
    }

    fn is_protein(&self) -> bool {
        matches!(self, CellType::ProteinA | CellType::ProteinB | CellType::ProteinC | CellType::ProteinD)
    }
}

#[derive(Debug, Clone)]
struct Cell {
    x: u8,
    y: u8,
    cell_type: CellType,
    owner: Owner,
    organ_id: u16,
}

struct GameState {
    width: u8,
    height: u8,
    grid: Vec<Vec<Cell>>,
    my_organs: HashMap<u16, (u8, u8)>,
    my_root_id: u16,
}

impl GameState {
    fn new(width: u8, height: u8) -> Self {
        let grid = vec![vec![Cell {
            x: 0,
            y: 0,
            cell_type: CellType::Empty,
            owner: Owner::Neutral,
            organ_id: 0,
        }; width as usize]; height as usize];

        GameState {
            width,
            height,
            grid,
            my_organs: HashMap::new(),
            my_root_id: 0,
        }
    }

    fn update_cell(&mut self, x: u8, y: u8, cell_type: CellType, owner: Owner, organ_id: u16) {
        self.grid[y as usize][x as usize] = Cell {
            x,
            y,
            cell_type: cell_type.clone(),
            owner,
            organ_id,
        };

        if owner == Owner::Me {
            self.my_organs.insert(organ_id, (x, y));

            if cell_type == CellType::Root {
                self.my_root_id = organ_id;
            }
        }
    }

    fn is_valid(&self, x: i16, y: i16) -> bool {
        x >= 0 && y >= 0 && x < self.width as i16 && y < self.height as i16
    }

    fn is_empty(&self, x: u8, y: u8) -> bool {
        let cell = &self.grid[y as usize][x as usize];
        cell.owner == Owner::Neutral && cell.cell_type != CellType::Wall
    }

    fn is_available(&self, x: u8, y: u8) -> bool {
        let cell = &self.grid[y as usize][x as usize];
        (cell.owner == Owner::Neutral && cell.cell_type != CellType::Wall) ||
        cell.cell_type.is_protein()
    }

    // BFS to find closest protein of type A
    fn find_closest_protein(&self, start_x: u8, start_y: u8, protein_type: CellType) -> Option<(u8, u8)> {
        let mut visited = vec![vec![false; self.width as usize]; self.height as usize];
        let mut queue = VecDeque::new();
        
        queue.push_back((start_x, start_y, 0));
        visited[start_y as usize][start_x as usize] = true;
        
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        
        while let Some((x, y, dist)) = queue.pop_front() {
            let cell = &self.grid[y as usize][x as usize];
            
            if cell.cell_type == protein_type && cell.owner == Owner::Neutral {
                return Some((x, y));
            }
            
            for (dx, dy) in &directions {
                let nx = x as i16 + dx;
                let ny = y as i16 + dy;
                
                if self.is_valid(nx, ny) {
                    let nx = nx as u8;
                    let ny = ny as u8;
                    
                    if !visited[ny as usize][nx as usize] {
                        visited[ny as usize][nx as usize] = true;
                        queue.push_back((nx, ny, dist + 1));
                    }
                }
            }
        }
        
        None
    }
}
