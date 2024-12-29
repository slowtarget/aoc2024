use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;

fn parse(input: String) -> (Vec<String>, Vec<String>) {
    let (_input, (available, desired) ) = separated_pair(
        separated_list1(tag(", "), alpha1::<&str, nom::error::Error<&str>>),
        tag("\n\n"),
        separated_list1(tag("\n"), alpha1)
    )(input.as_str()).unwrap();
    let available = available.into_iter().map(|x| x.to_string()).collect();
    let desired = desired.into_iter().map(|x| x.to_string()).collect();
    (available, desired)
}

fn part_2(available: &Vec<String>, desired: &Vec<String>) -> String {
    
    "bob".to_string()
}

fn part_1(available: &Vec<String>, desired: &Vec<String>) -> String {
    "bob".to_string()
}

pub(crate) fn solve(input: String) -> (String, String) {
    let (available, desired) = parse(input);
    let part_1_result = part_1(&available, &desired);
    let part_2_result = part_2(&available, &desired);

    (part_1_result.to_string(), part_2_result.to_string())
}


#[cfg(test)]
mod tests {
    use super::*;
    fn input () -> String {
        "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb".to_string()
    }
    #[test]
    fn parse_test() {
        
        let (available, desired) = parse(input());
        assert_eq!(available, vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"]);
        assert_eq!(desired, vec!["brwrr", "bggr", "gbbr", "rrbgbr", "ubwu", "bwurrg", "brgr", "bbrgwb"]);
    }
}
