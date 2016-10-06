pub fn square_of_sum(number: i32) -> i32 {
    (0..number).fold(0, |acc, n| acc + (n + 1)).pow(2)
}

pub fn sum_of_squares(number: i32) -> i32 {
    (0..number).fold(0, |acc, n| acc + (n + 1).pow(2))
}

pub fn difference(number: i32) -> i32 {
    square_of_sum(number) - sum_of_squares(number)
}
