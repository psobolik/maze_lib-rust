use maze::maze_generator;
use maze::maze_generator::cell_edge::CellEdge;
use maze::maze_generator::direction::Direction;

fn main() {
    static WALL_CHAR: &'static str = "▏";
    static FLOOR_CHAR: &'static str = "_";
    static PASSAGE_CHAR: &'static str = " ";

    let maze = maze_generator::generate(20, 10);
    println!("{:?}x{:?}", maze.columns(), maze.rows());

    println!("{}", (" ".to_owned() + FLOOR_CHAR).repeat((maze.columns()) as usize));

    for cell in maze {
        if let Some(cell) = cell {
            if let Some(west_edge) = cell.get_edge(&Direction::West) {
                match west_edge {
                    CellEdge::Wall | CellEdge::Border => print!("{}", WALL_CHAR),
                    _ => print!("{}", PASSAGE_CHAR)
                }
            }
            if let Some(south_edge) = cell.get_edge(&Direction::South) {
                match south_edge {
                    CellEdge::Wall | CellEdge::Border => print!("{}", FLOOR_CHAR),
                    _ => print!("{}", PASSAGE_CHAR)
                }
            }
            if let Some(east_edge) = cell.get_edge(&Direction::East) {
                if east_edge == CellEdge::Border {
                    println!("{}", WALL_CHAR);
                }
            }
        }
    }
}
