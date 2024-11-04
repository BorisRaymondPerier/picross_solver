use super::*;

pub fn clue_array_from_substring(clue_string : &mut String) -> ClueArray{
    let group_delimiter : &[_] = &[',', '.'];
    let group_strings : Vec<&str> = clue_string[1..].split(group_delimiter).collect();
    let mut values = vec![ Vec::<usize>::with_capacity(0); group_strings.len()];
    for (i, group_string) in group_strings.iter().enumerate() {
        let mut long_number = String::from("");
        let mut is_in_parenthesis = false;
        for c in group_string.chars() {
            match c {
                '0'..='9' => {
                    if is_in_parenthesis {
                        long_number.push_str(&c.to_string());
                    } else {
                        values[i].push(c.to_digit(10).unwrap() as usize);
                    }
                }, '(' => {
                    is_in_parenthesis = true;
                }, ')' => {
                    is_in_parenthesis = false;
                    values[i].push(long_number.parse().unwrap());
                    long_number.clear();
                },_ => {
                    println!("Not valid value as input")
                },
            }
        }
    }
    values
}

pub fn clue_boards_from_clue_string( clue_string : & String) -> (ClueArray, ClueArray){
    let top_delimiter : &[_] = &['t', 'T', 'u', 'U'];
    let top_start_pos = clue_string.find(top_delimiter);        
    let left_delimiter : &[_] = &['l', 'L', 's', 'S'];
    let left_start_pos = clue_string.find(left_delimiter);

    let left_pos :usize = left_start_pos.unwrap();
    let top_pos :usize = top_start_pos.unwrap();

    if top_pos < left_pos {
        let (top_sub_string, left_sub_string) = clue_string.split_at(left_pos);
        (clue_array_from_substring(&mut String::from(top_sub_string)), clue_array_from_substring(&mut String::from(left_sub_string)))
    } else {
        let (left_sub_string, top_sub_string) = clue_string.split_at(top_pos);
        (clue_array_from_substring(&mut String::from(top_sub_string)), clue_array_from_substring(&mut String::from(left_sub_string)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clue_string_decomposition() {
        let mut input = "L123(45)6789,(10)T987(54)321".to_string();
        let (top, left) = clue_boards_from_clue_string(& mut input);
        assert_eq!(left[0][0], 1);
        assert_eq!(left[0][3], 45);
        assert_eq!(left[0][left[0].len()-1], 9);
        assert_eq!(left[1][0], 10);

        assert_eq!(top[0][0], 9);
        assert_eq!(top[0][3], 54);
        assert_eq!(top[0][top[0].len()-1], 1);
    }

}