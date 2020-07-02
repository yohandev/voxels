
/// gives english names to the 'variants' number of blocks
/// of the half type
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum HalfBlockVariants
{
    /// vertical half block touching block to the north
    North = 0,
    /// vertical half block touching block to the south
    South = 1,
    /// vertical half block touching block to the west
    West = 2,
    /// vertical half block touching block to the east
    East = 3,
    /// horizontal half block touching block below
    Down = 4,
    /// horizontal half block touching block above
    Up = 5,
    /// 'full' block consisting of a north and south half block
    NorthSouth = 6,
    /// 'full' block consisting of an west and east half block
    WestEast = 7,
    /// 'full' block consisting of a down and up half block
    DownUp = 8,
}

impl From<u16> for HalfBlockVariants
{
    fn from(num: u16) -> Self
    {
        match num
        {
            0 => Self::North,
            1 => Self::South,
            2 => Self::West,
            3 => Self::East,
            4 => Self::Down,
            5 => Self::Up,
            6 => Self::NorthSouth,
            7 => Self::WestEast,
            8 => Self::DownUp,
            _ => Self::Down,
        }
    }
}