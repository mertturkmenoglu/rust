/**
 * match keyword example
 */
fn main() {
    let student_note: u8 = 20;
    let student_letter_note = match student_note {
        0 | 1 => "F0",
        2...30 => "FF",
        30...35 => "FD",
        35...39 => "DD",
        40 => "DC",
        41...60 => "CC",
        60...70 => "BB",
        70...80 => "BA",
        80...100 => "AA",
        _ => "N/A",
    };

    println!("{}", student_letter_note);

    let midterm_1: u8 = 100;
    let midterm_2: u8 = 80;
    let status = match (midterm_1, midterm_2) {
        (100, 100) => "Excellent",
        (100, _) => "Good",
        (_, 100) => "Good",
        (x, y) if x > 25 && y > 25 => "Normal",
        (_, _) => "Bad"
    };

    println!("{}", status);
}