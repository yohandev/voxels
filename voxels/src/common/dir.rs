use ezmath::*;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[allow(dead_code)]
/// direction enum that can be translated to
/// a u8, usize, and int vector
pub enum Direction
{
    // axes repr
    ZPos,
    XPos,
    ZNeg,
    XNeg,
    YPos,
    YNeg,

    // hand repr
    Front,
    Right,
    Back,
    Left,
    Up,
    Down,
}

const DIR: [[i32; 3]; 6] =
[
    [  0,  0,  1  ],
    [  1,  0,  0  ],
    [  0,  0, -1  ],
    [ -1,  0,  0  ],
    [  0,  1,  0  ],
    [  0, -1,  0  ],
];

impl Into<u8> for Direction
{
    fn into(self) -> u8
    {
        self as u8 % 6
    }
}

impl Into<usize> for Direction
{
    fn into(self) -> usize
    {
        (self as u8 % 6) as usize
    }
}

impl From<usize> for Direction
{
    fn from(num: usize) -> Self
    {
        match num % 6
        {
            0 => Direction::ZPos,
            1 => Direction::XPos,
            2 => Direction::ZNeg,
            3 => Direction::XNeg,
            4 => Direction::YPos,
            5 => Direction::YNeg,
            _ => Direction::ZPos,
        }
    } 
}

impl From<u8> for Direction
{
    fn from(num: u8) -> Self
    {
        (num as usize).into()
    }
}

impl Into<int3> for Direction
{
    fn into(self) -> int3
    {
        let n: usize = self.into();

        int3::new(DIR[n][0], DIR[n][1], DIR[n][2])
    }
}

impl Into<float3> for Direction
{
    fn into(self) -> float3
    {
        let n: usize = self.into();

        float3::new(DIR[n][0] as f32, DIR[n][1] as f32, DIR[n][2] as f32)
    }
}

impl std::convert::TryFrom<(i32, i32, i32)> for Direction
{
    type Error = ();

    fn try_from(vec: (i32, i32, i32)) -> Result<Self, ()>
    {
        let x = vec.0.signum();
        let y = vec.1.signum();
        let z = vec.2.signum();

        for i in 0..6
        {
            if DIR[i][0] == x
                && DIR[i][1] == y
                && DIR[i][2] == z
            {
                return Ok(i.into());
            }
        }
        Err(())
    }
}