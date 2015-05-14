use std::collections::HashSet;
use std::iter::repeat;
use world::Point;


pub fn parse_plaintext(data: String) -> HashSet<Point> {
    data.split("\n")
        .filter(|r| !r.starts_with('!'))
        .enumerate()
        .flat_map(|(rn, r)|
            repeat(rn).zip(r.chars().filter(|c| ['O', '.'].contains(c)).enumerate())
            .filter_map(|(rn, (cn, c))| if c == 'O' {Some((cn as isize, rn as isize))} else {None}))
        .collect::<HashSet<Point>>()
}
