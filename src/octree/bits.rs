/// get the number of set bits in a byte
pub fn num_bits(n: u8) -> u32
{
    const NIBBLE_LOOKUP: [u32; 16] =
    [
        0, 1, 1, 2, 1, 2, 2, 3, 
        1, 2, 2, 3, 2, 3, 3, 4
    ];
    NIBBLE_LOOKUP[(n & 0x0F) as usize] + NIBBLE_LOOKUP[(n >> 4) as usize]
}

#[test]
#[cfg(test)]
#[cfg(target_endian="little")]
fn test_endian()
{
    println!("running on little endian");
}

#[test]
#[cfg(test)]
#[cfg(target_endian="big")]
fn test_endian()
{
    println!("running on big endian");
}