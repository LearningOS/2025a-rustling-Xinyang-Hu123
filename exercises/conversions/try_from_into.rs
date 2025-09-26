use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug, PartialEq)]
enum IntoColorError {
    BadLen,
    IntConversion,
}

fn to_u8(v: i16) -> Result<u8, IntoColorError> {
    if (0..=255).contains(&v) {
        Ok(v as u8)
    } else {
        Err(IntoColorError::IntConversion)
    }
}

// Tuple implementation
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;
    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        Ok(Color {
            red: to_u8(tuple.0)?,
            green: to_u8(tuple.1)?,
            blue: to_u8(tuple.2)?,
        })
    }
}

// Array implementation
impl TryFrom<[i16; 3]> for Color {
    type Error = IntoColorError;
    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        Ok(Color {
            red: to_u8(arr[0])?,
            green: to_u8(arr[1])?,
            blue: to_u8(arr[2])?,
        })
    }
}

// Slice implementation
impl TryFrom<&[i16]> for Color {
    type Error = IntoColorError;
    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        if slice.len() != 3 {
            return Err(IntoColorError::BadLen);
        }
        Ok(Color {
            red: to_u8(slice[0])?,
            green: to_u8(slice[1])?,
            blue: to_u8(slice[2])?,
        })
    }
}

fn main() {
    let c1 = Color::try_from((183, 65, 14));
    println!("{:?}", c1);

    let c2: Result<Color, _> = [183, 65, 14].try_into();
    println!("{:?}", c2);

    let v = vec![183, 65, 14];
    let c3 = Color::try_from(&v[..]);
    println!("{:?}", c3);

    let c4: Result<Color, _> = (&v[..]).try_into();
    println!("{:?}", c4);
}