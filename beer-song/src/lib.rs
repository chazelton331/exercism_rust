pub fn verse(verse_number: i32) -> String {
    let mut response = "".to_string();

    if verse_number <= 0 {
        response.push_str("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
    } else {
        let string = match verse_number {
            1 => {
                format!("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n")
            },
            2 => {
                format!("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n")
            },
            _ => {
                format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", verse_number.to_string(), verse_number.to_string(), (verse_number - 1).to_string())
            },
        };

        response.push_str(&string);
    }

    response
}

pub fn sing(starting_verse: i32, ending_verse: i32) -> String {
    let mut response = "".to_string();
    let mut iterator = starting_verse;

    loop {
        let piece = format!("{}", verse(iterator));

        response.push_str(&piece);

        iterator -= 1;
        if iterator == ending_verse - 1 { break; }

        response.push_str("\n");
    }

    response
}
