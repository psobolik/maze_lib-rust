/// Enumerates the kinds of edges a Cell can have.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CellEdge {
    /// The edge is an exterior wall at the border of the maze.
    Border,
    /// The edge is an interior wall inside the maze.
    Wall,
    /// The edge is a passage to another cell.
    Passage,
}
