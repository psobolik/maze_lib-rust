use maze_lib::maze_generator;
use maze_lib::maze::cell_edge::CellEdge;
use maze_lib::maze::coordinates::Coordinates;
use maze_lib::maze::direction::Direction;
fn main() {
    static WALL_CHAR: &'static str = "▏";
    static FLOOR_CHAR: &'static str = "_";
    static PASSAGE_CHAR: &'static str = " ";

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
