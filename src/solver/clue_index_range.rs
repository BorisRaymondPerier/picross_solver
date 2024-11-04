use super::*;

pub type ClueIndexRange = Vec<(usize, usize)>;
type PackedIndices = Vec<usize>;

#[derive(PartialEq)]
pub enum Direction {
    Forward,
    Backward,
}

fn get_packed_line(clues : & ClueLine, line_size : usize, packing : Direction) -> PackedIndices {
    let mut indices : PackedIndices =  vec![0; line_size];
    if packing == Direction::Forward {
        let mut current_clue = 0;
        let mut current_counter = 0;
        for i in 0..line_size {
            indices[i] = current_clue;
            current_counter += 1;
            if current_clue < clues.len()-1 && current_counter == clues[current_clue] + 1 {
                current_clue += 1;
                current_counter = 0;
            }
        }
        return indices;
    } else {
        let mut current_clue = clues.len() - 1;
        let mut current_counter = 0;
        for i in (1..line_size).rev() {
            indices[i] = current_clue;
            current_counter += 1;
            if current_clue > 0 && current_counter == clues[current_clue] + 1 {
                current_clue -= 1;
                current_counter = 0;
            }
        }
        return indices;
    }
    
}

pub fn get_clue_index_range(clues : & ClueLine, line_size : usize) -> ClueIndexRange {
    let forward_range : PackedIndices = get_packed_line(& clues, line_size, Direction::Forward);
    let backward_range : PackedIndices = get_packed_line(& clues, line_size, Direction::Backward);
    let mut range = vec![(0,0); line_size];
    for i in 0..line_size {
        range[i] = (std::cmp::min(forward_range[i], backward_range[i]), std::cmp::max(forward_range[i], backward_range[i]));
    }
    return range;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_packing() {
        let size = 10;
        let clues = vec![3, 2, 1];
        let range : ClueIndexRange = get_clue_index_range(&clues, size);
        assert_eq!(range[0], (0,0));
        assert_ne!(range[1], (0,1));
        assert_eq!(range[4], (0,1));
        assert_eq!(range[7], (1,2));
        assert_eq!(range[size-1], (2,2));
        assert_ne!(range[size-1], (2,3));
    }
}