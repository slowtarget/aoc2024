use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space0},
    combinator::{map_res, opt},
    multi::many1,
    sequence::terminated,
    error::Error,
    IResult,
};
use std::time::Instant;

fn parse_i64(input: &str) -> IResult<&str, i64, Error<&str>> {
    let (input, num_str) = digit1::<&str, Error<&str>>(input)?;
    let val = num_str.parse::<i64>().unwrap();
    Ok((input, val))
}

fn parse_nums(input: &str) -> IResult<&str, Vec<i64>, Error<&str>> {
    let (mut input, _) = space0::<&str, Error<&str>>(input)?;
    let mut result = Vec::new();
    while let Ok((new_input, num_val)) = map_res(digit1::<&str, Error<&str>>, |s: &str| s.parse::<i64>())(input) {
        result.push(num_val);
        input = new_input;
        let (new_input, _) = space0::<&str, Error<&str>>(input)?;
        input = new_input;
    }
    Ok((input, result))
}

fn parse_line(input: &str) -> IResult<&str, (i64, Vec<i64>), Error<&str>> {
    let (input, (test_val, nums_line)) =
        nom::sequence::separated_pair(parse_i64, tag(":"), parse_nums)(input)?;
    Ok((input, (test_val, nums_line)))
}

fn parse_input(input: &str) -> IResult<&str, Vec<(i64, Vec<i64>)>, Error<&str>> {
    many1(terminated(parse_line, opt(tag("\n"))))(input)
}

fn can_make_target_part1(target: i64, nums: &[i64]) -> bool {
    if nums.is_empty() {
        return false;
    }
    if nums.len() == 1 {
        return nums[0] == target;
    }
    fn recurse(target: i64, nums: &[i64]) -> bool {
        if nums.len() == 1 {
            return nums[0] == target;
        }
        let last = nums[nums.len()-1];
        let add_candidate = target - last;
        if add_candidate >= 0 && recurse(add_candidate, &nums[..nums.len()-1]) {
            return true;
        }
        if last != 0 && target % last == 0 {
            let mul_candidate = target / last;
            if recurse(mul_candidate, &nums[..nums.len()-1]) {
                return true;
            }
        }
        false
    }
    recurse(target, nums)
}

fn part1(data: &[(i64, Vec<i64>)]) -> i64 {
    data.iter()
        .filter(|(t, nums)| can_make_target_part1(*t, nums))
        .map(|(t, _)| t)
        .sum()
}

// Part 2: Now we also have the concatenation operator. We will try all combinations of +, *, and concatenation.
fn can_make_target_with_concatenation(target: i64, nums: &[i64]) -> bool {
    if nums.is_empty() {
        return false;
    }
    if nums.len() == 1 {
        return nums[0] == target;
    }

    fn recurse(target: i64, nums: &[i64]) -> bool {
        if nums.len() == 1 {
            return nums[0] == target;
        }

        let last = nums[nums.len() - 1];

        // Case 1: Try addition
        if target >= last && recurse(target - last, &nums[..nums.len() - 1]) {
            return true;
        }

        // Case 2: Try multiplication
        if last != 0 && target % last == 0 && recurse(target / last, &nums[..nums.len() - 1]) {
            return true;
        }

        // Case 3: Try concatenation
        let last_digits = last.to_string();
        let target_digits = target.to_string();
        if target_digits.ends_with(&last_digits) {
            let remaining_target = target_digits[..target_digits.len() - last_digits.len()]
                .parse::<i64>()
                .unwrap_or(0);
            if recurse(remaining_target, &nums[..nums.len() - 1]) {
                return true;
            }
        }

        false
    }

    recurse(target, nums)
}
fn part2(data: &[(i64, Vec<i64>)]) -> i64 {
    data.iter()
        .filter(|(target, nums)| can_make_target_with_concatenation(*target, nums))
        .map(|(target, _)| target)
        .sum()
}

pub fn solve(input: String) {
    let start = Instant::now();
    let (_, data) = parse_input(&input).unwrap();
    let parse_duration = start.elapsed();
    let start_solve = Instant::now();
    let ans_part1 = part1(&data);
    let ans_part2 = part2(&data);
    let solve_duration = start_solve.elapsed();
    println!("Part1: {}", ans_part1);
    println!("Part2: {}", ans_part2); 
    println!("Parsing took: {} microseconds", parse_duration.as_micros());
    println!("Solving took: {} microseconds", solve_duration.as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_can_make_target_with_concatenation() {
        // 6 * 8 = 48, 48 || 6 = 486, 486 * 15 = 7290
        assert!(can_make_target_with_concatenation(7290, &[6, 8, 6, 15]));

        // 15 || 6 = 156
        assert!(can_make_target_with_concatenation(156, &[15, 6]));

        // 17 || 8 + 14 = 192
        assert!(can_make_target_with_concatenation(192, &[17, 8, 14]));

        // Valid case: 12 || 3 = 123
        assert!(can_make_target_with_concatenation(123, &[12, 3]));

        // Concatenation needed for success
        assert!(can_make_target_with_concatenation(5693516, &[5, 692, 6, 83, 833]));

        // Invalid case
        assert!(!can_make_target_with_concatenation(999, &[10, 20, 30])); // No way to make 999
    }
    mod check_part2 {
        use crate::puzzles::day7::{can_make_target_with_concatenation};

        fn get_test_cases() -> Vec<(i32, Vec<i64>, Vec<&'static str>, i64)> { vec![
        (7, vec![577, 48, 1, 160, 20, 923, 2], vec!["||", "*", "*", "+", "||", "*"], 18479401846),
        (18, vec![3, 662, 9, 9, 3, 70], vec!["*", "||", "||", "||", "||"], 198699370),
        (28, vec![3, 5, 843, 31, 259], vec!["*", "||", "+", "+"], 16133),
        (41, vec![80, 35, 37, 522, 4, 699], vec!["||", "||", "||", "+", "||"], 803537526699),
        (42, vec![431, 75, 59, 6, 9, 24, 590], vec!["||", "+", "+", "*", "+", "*"], 229618560),
        (51, vec![3, 69, 543, 729, 76], vec!["||", "*", "*", "||"], 14606754376),
        (55, vec![68, 9, 131, 366, 2, 941, 79], vec!["*", "||", "*", "+", "*", "+"], 210821591147),
        (58, vec![3, 9, 93, 9, 87], vec!["*", "*", "||", "+"], 25206),
        (60, vec![126, 9, 6, 23, 2, 92, 1, 1, 1, 3], vec!["*", "+", "||", "+", "+", "+", "*", "*", "*"], 342354),
        (68, vec![4, 7, 5, 7, 6, 710, 389, 92, 4, 9], vec!["*", "||", "*", "+", "*", "||", "||", "+", "*"], 1278639350964),
        (70, vec![2, 15, 41], vec!["||", "*"], 8815),
        (75, vec![288, 15, 85, 61, 997, 2], vec!["+", "||", "||", "||", "||"], 30385619972),
        (82, vec![43, 8, 8, 5, 71, 3, 98, 683, 8], vec!["+", "||", "*", "||", "||", "||", "||", "+"], 259071398691),
        (88, vec![97, 27, 1, 476, 30], vec!["*", "*", "+", "||"], 309530),
        (100, vec![5, 692, 6, 83, 833], vec!["||", "||", "||", "+"], 5693516),
        (115, vec![68, 8, 8, 91, 2], vec!["*", "||", "*", "*"], 991536),
        (117, vec![4, 5, 84, 6, 362, 93, 58, 8, 1, 3], vec!["+", "*", "||", "||", "+", "*", "*", "*", "*"], 10532505360),
        (120, vec![246, 800, 824, 4, 499, 9], vec!["||", "||", "||", "*", "||"], 12315361137569),
        (122, vec![966, 11, 54, 87, 1, 6, 252], vec!["+", "+", "||", "*", "+", "*"], 26004636),
        (124, vec![2, 9, 6, 6, 58, 202, 1, 9, 36, 5], vec!["+", "||", "||", "+", "+", "*", "*", "+", "||"], 128705),
        (128, vec![7, 919, 513, 1, 7, 1], vec!["||", "||", "||", "+", "||"], 791951381),
        (140, vec![305, 3, 97, 91, 3, 12, 5], vec!["+", "||", "*", "*", "+", "||"], 84348935),
        (142, vec![15, 6, 475, 6, 686, 38, 557], vec!["||", "+", "*", "||", "+", "+"], 3787281),
        (147, vec![9, 5, 677, 85, 86, 758, 9], vec!["+", "||", "+", "+", "*", "+"], 11254793),
        (148, vec![7, 455, 1, 9, 8, 9, 3, 2, 4, 4, 3, 17], vec!["*", "*", "||", "+", "*", "*", "*", "||", "||", "*", "||"], 51624553217),
        (159, vec![4, 3, 45, 9, 5, 93, 484, 6, 62], vec!["*", "+", "*", "*", "||", "+", "*", "+"], 1542524),
        (168, vec![706, 737, 6, 645, 877], vec!["||", "*", "*", "+"], 2735073067),
        (176, vec![215, 6, 81, 77, 6], vec!["+", "||", "+", "+"], 22264),
        (179, vec![416, 24, 578, 31, 1, 59, 23], vec!["+", "||", "*", "*", "*", "+"], 805817185),
        (190, vec![40, 3, 938, 163, 99], vec!["+", "*", "*", "||"], 657444299),
        (195, vec![8, 9, 508, 3, 4, 1, 9, 5, 1, 6, 43, 1], vec!["*", "||", "+", "*", "||", "||", "*", "+", "+", "||", "||"], 145022102431),
        (202, vec![17, 5, 62, 46, 365, 8, 7, 8, 2], vec!["*", "||", "+", "*", "+", "*", "*", "*"], 351895936),
        (205, vec![59, 736, 5, 3, 6, 3, 2, 6, 239, 2], vec!["+", "||", "*", "+", "*", "||", "+", "*", "*"], 342313964),
        (217, vec![616, 16, 7, 8, 82, 69, 70, 3], vec!["||", "||", "+", "+", "+", "||", "+"], 61632673),
        (220, vec![95, 7, 6, 441, 665], vec!["+", "||", "+", "*"], 975555),
        (224, vec![523, 2, 6, 4, 27, 6], vec!["*", "*", "+", "||", "||"], 6280276),
        (235, vec![65, 76, 8, 2, 127, 5, 3], vec!["||", "*", "+", "+", "*", "+"], 263688),
        (243, vec![210, 8, 6, 208, 8, 87, 1, 5, 4], vec!["*", "*", "+", "+", "+", "*", "||", "+"], 103839),
        (247, vec![33, 8, 6, 428, 48, 55, 62, 1], vec!["*", "||", "||", "*", "||", "+", "*"], 12702854517),
        (253, vec![9, 3, 98, 4, 4], vec!["*", "||", "||", "||"], 279844),
        (255, vec![74, 58, 9, 878, 30], vec!["*", "+", "||", "+"], 4301908),
        (263, vec![7, 208, 6, 890, 18, 25, 3], vec!["*", "+", "*", "||", "*", "+"], 3252950453),
        (272, vec![9, 1, 744, 643, 79], vec!["*", "*", "||", "*"], 529034797),
        (274, vec![6, 71, 7, 914, 76], vec!["*", "||", "+", "||"], 518176),
        (275, vec![9, 51, 38, 95, 162], vec!["||", "||", "||", "||"], 9513895162),
        (277, vec![200, 9, 63, 1, 88], vec!["+", "*", "||", "+"], 131759),
        (280, vec![3, 6, 355, 2, 5, 1, 4, 92, 3, 4, 56], vec!["||", "+", "*", "*", "+", "*", "+", "*", "*", "+"], 188888),
        (285, vec![374, 765, 5, 1, 67], vec!["||", "||", "+", "*"], 251092952),
        (291, vec![8, 8, 195, 3, 5], vec!["||", "+", "*", "+"], 854),
        (293, vec![1, 22, 5, 901, 739], vec!["||", "*", "+", "||"], 1511739),
        (298, vec![22, 35, 640, 4, 5, 2, 7, 3, 7], vec!["+", "+", "||", "||", "+", "+", "*", "||"], 2092627),
        (299, vec![191, 9, 2, 77, 39], vec!["+", "||", "*", "+"], 154193),
        (302, vec![81, 8, 4, 3, 716, 4, 5, 4, 4, 877], vec!["||", "+", "+", "||", "+", "||", "+", "+", "+"], 8258090),
        (309, vec![7, 1, 8, 8, 76, 4, 44, 615, 5, 4], vec!["||", "*", "+", "+", "||", "||", "||", "+", "||"], 6524446204),
        (310, vec![909, 749, 4, 2, 69, 5], vec!["+", "||", "+", "||", "||"], 16586695),
        (315, vec![3, 4, 653, 8, 9, 1, 2, 8, 7, 515, 1], vec!["||", "||", "+", "*", "*", "*", "+", "*", "+", "*"], 4367857),
        (327, vec![8, 6, 2, 53, 7, 727, 9, 2, 5, 5, 3, 4], vec!["+", "*", "*", "*", "||", "*", "+", "+", "+", "*", "*"], 1121982660),
        (339, vec![21, 4, 8, 7, 1, 374, 91], vec!["+", "||", "+", "||", "+", "||"], 302591),
        (343, vec![5, 5, 3, 4, 659, 8, 8, 9, 3, 7, 2, 32], vec!["*", "*", "+", "||", "+", "+", "||", "*", "+", "*", "||"], 478056832),
        (348, vec![127, 2, 262, 546, 78, 72], vec!["||", "*", "||", "+", "||"], 33326462472),
        (350, vec![10, 78, 795, 6, 537, 184, 6], vec!["||", "*", "+", "*", "||", "||"], 4602175921846),
        (374, vec![7, 92, 8, 20, 5, 4, 3, 8, 1, 74, 7, 1], vec!["+", "+", "||", "+", "||", "+", "*", "*", "*", "*", "*"], 444473008),
        (376, vec![9, 5, 4, 45, 1, 4, 4, 7, 3, 9, 481, 6], vec!["+", "*", "+", "*", "*", "+", "+", "||", "+", "*", "||"], 20019226),
        (397, vec![2, 63, 8, 9, 5, 759, 6, 267], vec!["||", "||", "*", "||", "||", "||", "*"], 633926778132),
        (426, vec![5, 96, 46, 7, 750], vec!["*", "+", "||", "*"], 3950250),
        (430, vec![22, 1, 5, 59, 218, 68, 3, 727], vec!["*", "*", "||", "+", "*", "+", "||"], 766839727),
        (434, vec![56, 85, 3, 7, 1], vec!["+", "+", "||", "||"], 14471),
        (438, vec![21, 6, 5, 17, 5, 5, 4, 8, 5, 5, 87], vec!["*", "+", "||", "||", "||", "+", "+", "*", "||", "*"], 5706186885),
        (449, vec![23, 8, 5, 606, 6, 88, 878, 96], vec!["+", "+", "*", "||", "+", "+", "*"], 21036672),
        (453, vec![5, 760, 189, 2, 3], vec!["||", "*", "*", "*"], 6531840),
        (476, vec![4, 7, 4, 623, 945], vec!["+", "||", "||", "*"], 108318735),
        (484, vec![1, 3, 5, 44, 85, 60, 3, 3, 550], vec!["+", "||", "*", "||", "*", "*", "*", "||"], 106965900550),
        (502, vec![6, 3, 348, 5], vec!["+", "||", "*"], 46740),
        (506, vec![720, 221, 81, 3, 8, 27, 232], vec!["||", "*", "+", "||", "+", "||"], 583379075232),
        (508, vec![6, 1, 2, 3, 3, 88, 195, 6, 13, 1, 7], vec!["*", "*", "+", "+", "+", "+", "*", "*", "*", "||"], 234787),
        (517, vec![99, 24, 8, 782, 662], vec!["||", "+", "+", "+"], 11376),
        (522, vec![5, 7, 78, 170, 36, 44, 7], vec!["+", "*", "||", "||", "||", "||"], 93617036447),
        (526, vec![58, 1, 653, 279, 866], vec!["*", "*", "||", "||"], 37874279866),
        (527, vec![6, 16, 659, 112, 21, 2], vec!["*", "+", "+", "+", "||"], 8882),
        (530, vec![2, 4, 790, 6, 929], vec!["||", "*", "||", "*"], 176143974),
        (533, vec![5, 1, 535, 16, 644], vec!["||", "*", "||", "||"], 2728516644),
        (565, vec![8, 2, 4, 64, 2, 2, 1, 701, 7, 9, 4], vec!["+", "||", "+", "||", "+", "+", "||", "+", "+", "||"], 16857174),
        (577, vec![4, 8, 92, 3, 249, 8, 1, 630, 6], vec!["||", "+", "||", "+", "+", "||", "||", "+"], 16601636),
        (579, vec![284, 98, 3, 537], vec!["+", "||", "||"], 3823537),
        (585, vec![7, 7, 85, 8, 4, 3, 38, 4, 4, 78], vec!["+", "||", "+", "*", "*", "+", "*", "*", "+"], 287342),
        (597, vec![610, 260, 558, 721, 12], vec!["+", "*", "||", "+"], 485460733),
        (600, vec![706, 36, 1, 8, 85], vec!["*", "+", "*", "||"], 20333685),
        (603, vec![1, 3, 5, 992, 6, 9, 7, 7, 4, 8, 3, 3], vec!["+", "*", "||", "+", "+", "+", "*", "||", "||", "+", "*"], 44129553),
        (604, vec![7, 49, 727, 44, 5, 979, 729], vec!["||", "+", "||", "*", "*", "+"], 722718109),
        (610, vec![74, 84, 45, 865, 328], vec!["||", "||", "+", "*"], 245773680),
        (612, vec![28, 18, 6, 1, 3, 9, 944], vec!["+", "+", "*", "||", "+", "*"], 502208),
        (632, vec![5, 5, 1, 5, 314, 1, 7, 3, 28, 468], vec!["+", "||", "||", "*", "||", "||", "||", "+", "+"], 318710669),
        (638, vec![10, 24, 7, 813, 898, 9], vec!["+", "||", "||", "*", "*"], 2811024666),
        (639, vec![24, 45, 136, 90, 3, 5, 10, 40], vec!["*", "||", "*", "*", "*", "*", "*"], 583273440000),
        (643, vec![762, 6, 5, 8, 577, 26], vec!["+", "+", "+", "||", "*"], 20321002),
        (650, vec![3, 45, 8, 664, 185, 4, 31], vec!["+", "+", "||", "*", "*", "*"], 1299872160),
        (654, vec![871, 5, 398, 335, 5], vec!["+", "||", "||", "||"], 8763983355),
        (662, vec![778, 528, 84, 527, 11], vec!["+", "+", "+", "||"], 191711),
        (672, vec![98, 4, 4, 956, 6], vec!["||", "*", "+", "+"], 4898),
        (679, vec![7, 8, 6, 2, 3, 587, 36, 3, 4, 7, 5, 6], vec!["||", "||", "*", "+", "+", "||", "*", "+", "||", "*", "||"], 324356356),
        (681, vec![6, 60, 2, 1, 5], vec!["||", "||", "||", "+"], 66026),
        (685, vec![83, 7, 98, 5, 439, 3], vec!["+", "||", "||", "||", "||"], 909854393),
        (696, vec![43, 855, 21, 460, 893, 78], vec!["+", "*", "||", "||", "||"], 1885846089378),
        (698, vec![62, 159, 51, 8, 40, 736, 78], vec!["||", "||", "*", "+", "+", "||"], 4972838478),
        (700, vec![42, 110, 3, 40, 9, 93, 7], vec!["+", "+", "+", "+", "||", "*"], 143451),
        (702, vec![2, 737, 7, 5, 2, 9, 2, 2, 8, 9, 9, 9], vec!["+", "||", "+", "+", "*", "+", "||", "+", "+", "||", "*"], 59975991),
        (711, vec![9, 73, 76, 899], vec!["+", "+", "||"], 158899),
        (724, vec![443, 5, 683, 27, 294], vec!["*", "||", "+", "||"], 2215710294),
        (747, vec![83, 78, 10, 18, 23, 5], vec!["||", "||", "*", "||", "||"], 15080580235),
        (750, vec![698, 608, 60, 1, 6], vec!["||", "||", "+", "||"], 698608616),
        (751, vec![275, 9, 8, 7, 95, 612], vec!["+", "+", "*", "*", "||"], 194180612),
        (758, vec![63, 9, 86, 991, 23, 742], vec!["||", "||", "*", "||", "||"], 6341012623742),
        (759, vec![970, 293, 102, 662, 7], vec!["+", "||", "+", "+"], 1263771),
        (764, vec![76, 8, 5, 6, 66, 9, 731, 8, 58], vec!["+", "+", "+", "*", "+", "||", "+", "*"], 364224862),
        (776, vec![91, 196, 50, 3, 9, 7, 3], vec!["*", "+", "||", "*", "||", "||"], 160976773),
        (778, vec![333, 5, 8, 66, 355, 4], vec!["||", "+", "||", "+", "||"], 3347214),
        (780, vec![1, 9, 1, 26, 4, 239, 725, 1, 11], vec!["*", "*", "||", "+", "+", "+", "*", "+"], 1905),
        (793, vec![505, 9, 9, 6, 53], vec!["*", "||", "*", "||"], 27275453),
        (797, vec![73, 637, 6, 12, 6, 161, 86, 7], vec!["||", "*", "*", "||", "||", "||", "||"], 53018646161867),
        (812, vec![577, 634, 286, 206, 7], vec!["*", "*", "||", "||"], 1046239482067),
        (814, vec![9, 7, 3, 7, 5, 3, 7, 5, 41, 178, 5], vec!["*", "*", "||", "+", "+", "*", "*", "+", "+", "*"], 334470),
        (817, vec![5, 3, 70, 6, 8, 89], vec!["||", "*", "||", "*", "||"], 29684889),
        (831, vec![6, 8, 373, 2, 28], vec!["+", "||", "*", "*"], 804888),
        (850, vec![3, 226, 80, 49, 2, 2, 11, 1], vec!["||", "||", "*", "||", "*", "||", "*"], 31622640411),
    ]
        }

        #[test]
        fn verify_successful_sequences() {
            let vec = get_test_cases();
            for (line, nums, ops, expected_result) in vec {
                let valid_with_can_make = can_make_target_with_concatenation(expected_result, &nums);
                assert!(
                    valid_with_can_make,
                    "Line {}: nums = {:?}, ops = {:?}, expected = {}, valid_with_can_make = {}",
                    line, nums, ops, expected_result, valid_with_can_make
                );
            }
        }
    }

    #[test]
    fn parse_test() {
        let input = "190: 10 19\n3267: 81 40 27\n";
        let (_, data) = parse_input(input).unwrap();
        assert_eq!(data.len(), 2);
        assert_eq!(data[0], (190, vec![10, 19]));
        assert_eq!(data[1], (3267, vec![81, 40, 27]));
    }

    #[test]
    fn part1_test_simple() {
        let data = vec![
            (190, vec![10, 19]),
            (3267, vec![81, 40, 27]),
            (83, vec![17, 5]),
            (156, vec![15, 6]),
            (7290, vec![6, 8, 6, 15]),
            (161011, vec![16,10,13]),
            (192, vec![17,8,14]),
            (21037, vec![9,7,18,13]),
            (292, vec![11,6,16,20]),
        ];
        let ans = part1(&data);
        assert_eq!(ans, 3749);
    }
    #[test]
    fn test_old_vs_new_logic_failure_case() {
        // Original logic incorrectly fails to produce 192 from [17,8,14].
        // Correct logic:
        // 17||8 = 178; 178+14 = 192
        // The old approach wouldn't get 192, but the improved two-step logic will.
        let data = vec![(192, vec![17, 8, 14])];
        let ans = part2(&data);
        assert_eq!(ans, 192);
    }
    #[test]
    fn test_concatenation_and_arithmetic_complex() {
        // This is from the puzzle's example for part two:
        // 7290: 6 8 6 15
        //
        // Correct sequence:
        // 6 * 8 = 48
        // 48 || 6 = 486
        // 486 * 15 = 7290
        //
        let data = vec![(7290, vec![6, 8, 6, 15])];
        assert_eq!(part2(&data), 7290);
    }
    #[test]
    fn part2_test_simple() {
        let data = vec![
            (190, vec![10, 19]),      // from part1
            (3267, vec![81, 40, 27]), // from part1
            (292, vec![11,6,16,20]),  // from part1
            (156, vec![15,6]),        // new with concatenation: 15||6=156
            (7290, vec![6,8,6,15]),   // new with concatenation: 6*8||6*15 = 48||6*15 = 486*15=7290 (left-to-right)
            (192, vec![17,8,14]),     // new with concatenation: 17||8+14 = 178+14=192
        ];
        // sum = 190 + 3267 + 292 + 156 + 7290 + 192 = 11387
        let ans = part2(&data);
        assert_eq!(ans, 11387);
    }

    #[test]
    fn part1_test_simpler() {
        let data = vec![
            (10, vec![5, 2]),  // 5*2=10 -> true
            (15, vec![5, 3]),  // 5*3=15 -> true
            (11, vec![5, 3]),  // no
        ];
        let ans = part1(&data);
        assert_eq!(ans, 25);
    }

    #[test]
    fn part2_test_simpler() {
        // With concatenation:
        // 12: 1 2 can form 1||2=12 -> matches 12
        // 11: 1 1 can form 1||1=11 -> matches 11
        // 25: 5 2 no standard ops produce 25; check concatenation: 5||2=52, 5+2=7, 5*2=10. No match.
        let data = vec![
            (12, vec![1,2]),
            (11, vec![1,1]),
            (25, vec![5,2]),
        ];
        // 12 + 11 = 23 no 25
        let ans = part2(&data);
        assert_eq!(ans, 23);
    }

    // Additional tests to help narrow down issues:

    // Test pure concatenation:
    #[test]
    fn test_concatenation_only() {
        // Only concatenation makes sense:
        // 123: 1 2 3 -> 1||2||3 = 123
        assert_eq!(part2(&vec![
            (123, vec![1, 2, 3])
        ]), 123);
    }

    // Test concatenation + addition:
    #[test]
    fn test_concatenation_plus_addition() {
        // 25: 2 5 -> can be 2||5=25 or 2+5=7 or 2*5=10
        // Here concatenation should yield 25.
        let data = vec![
            (25, vec![2, 5])
        ];
        let ans = part2(&data);
        assert_eq!(ans, 25);
    }

    // Test concatenation + multiplication:
    #[test]
    fn test_concatenation_plus_multiplication() {
        // 102: 10 2 -> can be 10||2=102, 10+2=12, 10*2=20
        let data = vec![
            (102, vec![10, 2])
        ];
        let ans = part2(&data);
        assert_eq!(ans, 102);
    }

    // Test a sequence where concatenation and addition are needed:
    // Example: 170: 17 0 -> 17||0=170 or 17+0=17
    #[test]
    fn test_concat_with_leading_zero() {
        let data = vec![
            (170, vec![17, 0])
        ];
        let ans = part2(&data);
        assert_eq!(ans, 170);
    }

    // Test a more complex sequence where multiple concatenations are possible:
    // 1234: 1 23 4 can yield 1||23=123; 123||4=1234 
    #[test]
    fn test_multiple_concats() {
        let data = vec![
            (1234, vec![1, 23, 4])
        ];
        let ans = part2(&data);
        assert_eq!(ans, 1234);
    }
}
