// Matches Python Encoding.py

use crate::data;

fn calculate_bits(tnt: i32) -> (i32, i32, i32) {
    let big = tnt / 418;
    let tnt = tnt % 418;
    let medium = tnt / 11;
    let small = tnt % 11;
    (big, medium, small)
}

fn calculate_upaccel_bits(tnt: i32) -> (i32, i32) {
    (tnt / 8, tnt % 8)
}

// Matches Python binary() — outputs '1'/'0' chars based on greedy subtraction
fn binary(mut n: i32, values: &[i32]) -> String {
    values
        .iter()
        .map(|&v| {
            if n >= v {
                n -= v;
                '1'
            } else {
                '0'
            }
        })
        .collect()
}

fn rev(s: String) -> String {
    s.chars().rev().collect()
}

// Matches Python EncodingData.__str__()
fn format_encoding(
    purple: i32,
    blue: i32,
    cyan: i32,
    light_blue: i32,
    lime: i32,
    yellow: i32,
    orange: i32,
    red: i32,
    pink: i32,
    magenta: i32,
    purple2: i32,
    block: &str,
) -> String {
    format!(
        "purple:      [{}]\nblue:        [{}]\ncyan:        [{}]\nlight_blue:  [{}]\nlime:        [{}]\nyellow:      [{}]\norange:      [{}]\nred:         [{}]\npink:        [{}]\nmagenta:     [{}]\npurple:      [{}]\n\nplace a block at: {}",
        rev(binary(purple,     &[4, 2, 1])),
        rev(binary(blue,       &[2, 1])),
        rev(binary(cyan,       &[4, 4, 2, 1])),
        rev(binary(light_blue, &[4, 2, 1])),
        rev(binary(lime,       &[11, 11, 8, 4, 2, 1])),
        binary(yellow,         &[8, 4, 2, 1]),       // NOT reversed
        rev(binary(orange,     &[11, 11, 8, 4, 2, 1])),
        rev(binary(red,        &[4, 2, 1])),
        rev(binary(pink,       &[4, 4, 2, 1])),
        rev(binary(magenta,    &[2, 1])),
        rev(binary(purple2,    &[2, 1])),
        block,
    )
}

// Matches Python calculateEncoding()
// Returns empty string if the encoding path is not handled (shouldn't happen in practice)
pub fn compute_encoding(
    early_tnt: i32,
    late_tnt: i32,
    upaccel_tnt: i32,
    long_range: bool,
    direction: i32,
    direction_angle: i32,
    block: &str,
) -> String {
    let (blue, purple) = calculate_upaccel_bits(upaccel_tnt);
    let yellow = direction + if long_range { 8 } else { 0 };
    let purple2 = direction_angle;

    let half = data::MAX_TNT / 2;

    // Normal FTL encoding: both sides within MAX_TNT/2
    if early_tnt <= half && late_tnt <= half {
        let (red, orange, pink) = calculate_bits(early_tnt);
        let (light_blue, lime, cyan) = calculate_bits(late_tnt);
        let magenta = 0;
        return format_encoding(purple, blue, cyan, light_blue, lime, yellow, orange, red, pink, magenta, purple2, block);
    }

    // Variable bit = 1: earlyTnt < lateTnt
    if early_tnt < late_tnt {
        let magenta = 1;
        let red_tnt_in_use = (early_tnt / 11) * 11;
        let (red, orange, pink) = calculate_bits(early_tnt);
        let (light_blue, lime, cyan) = calculate_bits(late_tnt - (data::MAX_VARIABLE_TNT / 2 - red_tnt_in_use));
        return format_encoding(purple, blue, cyan, light_blue, lime, yellow, orange, red, pink, magenta, purple2, block);
    }

    // Variable bit = 2: lateTnt < earlyTnt
    if late_tnt < early_tnt {
        let magenta = 2;
        let red_tnt_single = early_tnt % 11;
        let blue_tnt_single = late_tnt % 11;
        let early_adj = early_tnt - red_tnt_single;
        let late_adj = late_tnt - blue_tnt_single;
        let red_tnt = data::MAX_VARIABLE_TNT / 2 - late_adj;
        let blue_tnt = early_adj - red_tnt;
        let (red, orange, pink) = calculate_bits(red_tnt + red_tnt_single);
        let (light_blue, lime, cyan) = calculate_bits(blue_tnt + blue_tnt_single);
        return format_encoding(purple, blue, cyan, light_blue, lime, yellow, orange, red, pink, magenta, purple2, block);
    }

    String::new()
}
