
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
    #[test]
    fn valid_scalene() {
        let result = get_triangle_type(2, 3, 4);
        assert_eq!(result, "scalene");
    }

    // 2 - valid equilateral
    #[test]
    fn valid_equilateral() {
        let result = get_triangle_type(1, 1, 1);
        assert_eq!(result, "equilateral");
    }

    // 3 - valid isosceles 1
    #[test]
    fn valid_isosceles1() {
        let result = get_triangle_type(1, 2, 2);
        assert_eq!(result, "isosceles");
    }

    // 4 - valid isosceles 2
    #[test]
    fn valid_isosceles2() {
        let result = get_triangle_type(2, 1, 2);
        assert_eq!(result, "isosceles");
    }

    // 5 - valid isosceles 3
    #[test]
    fn valid_isosceles3() {
        let result = get_triangle_type(2, 2, 1);
        assert_eq!(result, "isosceles");
    }


    // 6 - invalid because a + b = c
    #[test]
    #[should_panic(expected ="invalid triangle")]
    fn invalid_zero_area1() {
        let _result = get_triangle_type(1, 1, 2);
    }

    // 7 - invalid because a + c = b
    #[test]
    #[should_panic(expected ="invalid triangle")]
    fn invalid_zero_area2() {
        let _result = get_triangle_type(1, 2, 1);
    }

    // 8 - invalid because b + c = a
    #[test]
    #[should_panic(expected ="invalid triangle")]
    fn invalid_zero_area3() {
        let _result = get_triangle_type(2, 1, 1);
    }

    // 9 - invalid because a + b < c 
    #[test]
    #[should_panic(expected ="invalid triangle")]
    fn invalid_side_too_long_1() {
        let _result = get_triangle_type(1, 1, 3);
    }

    // 10 - invalid because a + c < b
    #[test]
    #[should_panic(expected ="invalid triangle")]
    fn invalid_side_too_long_2() {
        let _result = get_triangle_type(1, 3, 1);
    }

    // 11 - invalid because b + c < a
    #[test]
    #[should_panic(expected ="invalid triangle")]
    fn invalid_side_too_long_3() {
        let _result = get_triangle_type(3, 1, 1);
    }




    // 12 - invalid because 0 length side
    #[test]
    #[should_panic(expected ="invalid triangle")]
    fn invalid_side_length_zero() {
        let _result = get_triangle_type(0, 1, 1);
    }

    // 13 - invalid because all sides are 0
    #[test]
    #[should_panic(expected ="invalid triangle")]
    fn invalid_all_side_lengths_zero() {
        let _result = get_triangle_type(0, 0, 0);
    }

    // invalid inputs impossible, enforced by rust compiler

}
