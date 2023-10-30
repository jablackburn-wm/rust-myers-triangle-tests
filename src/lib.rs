
fn get_triangle_type<'a>(a: u32, b: u32, c: u32, ) -> &'a str {
    let mut longest_side = a;

    if b > a {
        longest_side = b;
    }
    if c > b {
        longest_side = c;
    }

    let perimeter = a + b + c;
    if (perimeter - longest_side) <= longest_side {
        panic!("invalid triangle");
    }

    if a == b && b == c {
        return "equilateral";
    }
    if a == b || a == c || b == c {
        return "isosceles";
    }
    return "scalene";
}

#[cfg(test)]
mod tests {
    use super::*;

    // 1 - valid scalene
    // 2 - valid equilateral
    // 3 - valid isosceles 1
    // 4 - valid isosceles 2
    // 5 - valid isosceles 3
    // 6 - invalid because 0 length side
    // 7 - invalid negative side (expect panic)
    // 8 - invalid because a + b = c
    // 9 - invalid because a + c = b
    // 10 - invalid because b + c = a
    // 11 - invalid because a + b < c 
    // 12 - invalid because a + c < b
    // 13 - invalid because b + c < a
    // 14 - invalid because all sides are 0
    // 15 - invalid type string (expect panic)
    // 16 - invalid type float (expect panic)
}
