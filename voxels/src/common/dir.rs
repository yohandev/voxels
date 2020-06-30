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

impl Into<int3> for Direction
{
    fn into(self) -> int3
    {
        const DIR: [[i32; 3]; 6] =
        [
            [  0,  0,  1  ],
            [  1,  0,  0  ],
            [  0,  0, -1  ],
            [ -1,  0,  0  ],
            [  0,  1,  0  ],
            [  0, -1,  0  ],
        ];
        let n: usize = self.into();

        int3::new(DIR[n][0], DIR[n][1], DIR[n][2])
    }
}

impl Into<float3> for Direction
{
    fn into(self) -> float3
    {
        const DIR: [[f32; 3]; 6] =
        [
            [  0.0,  0.0,  1.0  ],
            [  1.0,  0.0,  0.0  ],
            [  0.0,  0.0, -1.0  ],
            [ -1.0,  0.0,  0.0  ],
            [  0.0,  1.0,  0.0  ],
            [  0.0, -1.0,  0.0  ],
        ];
        let n: usize = self.into();

        float3::new(DIR[n][0], DIR[n][1], DIR[n][2])
    }
}