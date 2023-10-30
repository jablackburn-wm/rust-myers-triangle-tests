
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

    #[test]
    fn test1() {
        let result = get_triangle_type(1, 2, 2);
        assert_eq!(result, "isosceles");
    }
}
