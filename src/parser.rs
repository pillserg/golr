use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashSet;
use world::Point;


pub fn parse_plaintext(data: String) -> HashSet<Point> {
    let mut ret: HashSet<Point> = HashSet::new();

    for (row_num, row) in data.split("\n")
        .filter(| row | !row.starts_with('!'))
        .enumerate() {
        for (cell_num, cell) in UnicodeSegmentation::graphemes(row, true).filter(|c|["O", "."].contains(c)).enumerate() {
            match cell {
                "O" => ret.insert((cell_num as isize, row_num as isize)),
                _ => continue
            };
        };
    };
    ret
}
    
