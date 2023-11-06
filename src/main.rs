use maze_lib::maze::cell_edge::CellEdge;
use maze_lib::maze::coordinates::Coordinates;
use maze_lib::maze::direction::Direction;
use maze_lib::maze_generator;

fn main() {
    static WALL_CHAR: &str = "▏";
    static FLOOR_CHAR: &str = "_";
    static PASSAGE_CHAR: &str = " ";

    let maze = maze_generator::generate(20, 10);
    println!("{:?}x{:?}", maze.columns(), maze.rows());

    println!(
        "{}",
        (" ".to_owned() + FLOOR_CHAR).repeat((maze.columns()) as usize)
    );

    for row in (0..maze.rows() as i32).rev() {
        for column in (0..maze.columns() as i32).rev() {
            if let Some(cell) = maze.cell(&Coordinates::new(column, row)) {
                if let Some(edge) = cell.edge(&Direction::East) {
                    match edge {
                        CellEdge::Wall | CellEdge::Border => print!("{}", WALL_CHAR),
                        _ => print!("{}", PASSAGE_CHAR),
                    }
                }
                if let Some(edge) = cell.edge(&Direction::South) {
                    match edge {
                        CellEdge::Wall | CellEdge::Border => print!("{}", FLOOR_CHAR),
                        _ => print!("{}", PASSAGE_CHAR),
                    }
                }
                if let Some(edge) = cell.edge(&Direction::West) {
                    if edge == CellEdge::Border {
                        println!("{}", WALL_CHAR);
                    }
                }
            }
        }
    }
}
