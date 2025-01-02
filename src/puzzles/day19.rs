use std::collections::HashSet;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;

fn parse(input: String) -> (Vec<String>, Vec<String>) {
    let (_input, (available, desired) ) = separated_pair(
        separated_list1(alt((tag(", \n"),tag(", "))), alpha1::<&str, nom::error::Error<&str>>),
        tag("\n \n"),
        separated_list1(tag("\n"), alpha1)
    )(input.as_str()).unwrap();
    let available = available.into_iter().map(|x| x.to_string()).collect();
    let desired = desired.into_iter().map(|x| x.to_string()).collect();
    (available, desired)
}

fn is_possible(available: &[String], desired: &str) -> bool {
    // if desired.is_empty() {
    //     return true;
    // }
    // available.iter().filter(|&x| x.len() > 0 && x.len() <= desired.len() && x[..] == desired[0..x.len()]).any(|x| {
    //     is_possible(available, &desired[x.len()..])
    // })
    // println!("> desired: {:?}", desired);
    let mut queue = vec![desired];
    let mut visited = HashSet::new();
    while let Some(desired) = queue.pop() {
        // println!("desired: {:?} queue: {}", desired, queue.len());
        if !visited.insert(desired) {
            continue;
        }
        if desired.is_empty() {
            return true;
        }
        for x in available {
            if desired.starts_with(x) {
                queue.push(&desired[x.len()..]);
            }
        }
    }
    false
}

fn part_2(available: &[String], desired: &[String]) -> String {

    "bob".to_string()
}

fn part_1(available: &[String], desired: &[String]) -> String {
    let desired_list = desired.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
    desired_list.iter().filter(|&x| is_possible(available, x)).count().to_string()
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
        "r, wr, b, g, 
bwu, rb, gb, br
 
brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
bbrgwb
brgr".to_string()
    }
    fn awkward_input() -> String {
        "rug, uugbb, bubbbr, bbubw, wwb, wbw, gwbbuw, ubg, ruwug, bbwuww, wubwrw, ug, ugu, ggrbg, ruw, rbur, uuubbuu, wb, wurrbgr, rwruwwuu, rwg, rwbb, bwugugg, bur, ggwub, brgbr, guuu, wgg, bbw, ugrbrb, ggrw, wbgu, rwgbg, ggwu, wurwuw, gruub, gugw, bgu, wrr, rbrg, bww, uggwrr, rgg, uugb, bbrwuru, rw, urbu, ggggg, ru, wrrgwrrb, uurrgur, bgrw, ubuwr, gwwgug, bggb, uwub, gwu, wrwbgw, rrbb, uuwrggb, uuugw, bbbubg, bg, ggr, rgw, ugburuw, ggbbww, rrgbru, wurg, wugu, rbuu, wgrg, ubugu, uuwuur, ur, wgbbg, wubu, gbrrwgu, bwbu, wbgguwg, rrwbwru, buggggg, bwu, uw, grubg, bggbw, ugwwgwu, rbubgwg, wgwuu, rrr, uubwwgrr, bbwguw, uuru, wgww, grbwu, gbgww, uuu, gruwwg, urgw, ubu, ggbgb, buwur, bbgb, bub, wbru, gbug, wgrruub, wrwwru, wuwgg, uwg, uuwgw, rrruwb, rbr, grrrugwb, rbgb, wwbr, ggbugg, ggg, buu, uwr, guug, wg, wbwu, wgb, rbbb, rub, wru, bubr, rgruwrgr, gwr, urru, wuw, uuwg, bbr, ruwg, urwgu, bwbugbb, wgbgrwgg, grwb, rwgg, uugg, uww, w, urwgrgb, gw, ubrb, grbrrbrw, wgwgrr, rggurb, buuww, bgrrb, wbrg, ggburu, rgwgb, wrw, bw, wubbb, bbwww, ubbg, bbg, ruururru, rbg, wgug, gur, rrugw, gwurbg, gr, bwwgrr, gub, wrgwrw, gbrrb, rr, rgbu, bbrur, ubgbu, wuwrw, guub, guu, uwbww, gguuww, ubuw, uuggu, wgwg, grwrr, bwg, uwgrb, grwu, gwbw, rwu, buubrb, ggbubw, wrgw, wwu, uuubwrgb, urwgbwu, rwggwrg, uuuw, gbrwb, bbuu, urgbgb, www, guw, uuw, wbubg, brw, gubruu, uwrg, bbbggg, bwr, wgbg, wguw, rbbgruru, uubgggr, ugw, grgugr, uugrrbb, rwgwg, ubgu, burgwgbw, gwrubbu, bwuug, bburur, burwu, wbb, ub, wr, ggubww, grw, ww, buw, urbru, gbgr, burwgur, ugr, gwwbgb, urr, wuwg, bbubrrgu, wuuwb, brrwg, g, gbgurrw, rgu, ubb, urggug, wub, gwubu, bgrr, wrbwwb, guuuuug, rrb, brugwu, wgrgwu, bru, wrbgugur, ubww, uur, uuub, wubwrb, uubggur, bbu, urb, rrggwuw, brgw, urbrrg, uwuur, bubw, bbwuu, gbu, rrg, rrgbw, gbr, wbu, ubrw, rww, ubw, wugr, rrur, bggu, uurrbwr, br, ggu, bgww, gwbbubgu, grwbw, uu, wguwg, urwubg, rruwg, wur, rgbwg, wbrgbg, ugrr, wbbggu, gww, bugb, wrbr, grurrg, gbrgwb, rur, gubbr, gbb, gggubwgu, gwbbbwru, rbbbu, rg, gwg, bgg, rgbg, uubbrwb, uwur, rwr, uwu, wbg, wwwbgurr, grb, gbg, rugg, wgub, bb, bgb, ubwbrww, gugbr, wwgwwwug, brb, rbb, uwgg, ubbbw, rwwr, urrw, ubuuub, wbub, ugbrr, ubrr, bgwr, gwb, wuuw, ggbuwb, wug, wgugr, urgrb, uwbbwr, gwgb, uug, gbur, wwr, gug, rgbubw, ruu, gbbw, grg, ggruwgb, urwggu, rrw, rgguuu, bgwg, uub, rbu, bu, grrr, r, ugb, bbrb, gwwbr, bwwur, wwg, grurrgb, wrguuu, rgggw, ugbwg, rwb, wubgw, grrww, uwb, bgw, bggrw, rgr, rbwg, buugbbb, bgr, gbgrgr, wbr, wgr, wbbr, gg, gru, urg, bguu, bbwbgur, bburgw, ubr, wrg, gu, rbw, rwbrur, bwgru, ubwrugb, uggubw, rgbr, bbgg, wrb, wwuug, bwbuw, rb, gwugbr, uuwrw, u, rburg, wguwgwu, bwrru, uguwgb, wuu, ugg, bgrg, wrrw, brg, rwwru, buuwg, ubuuwwr, ruwr, bbb, guwuwuug, rrbgg, ugwub, rrwwugb, bug, wgw, urw, brr, bwuwrw, bbug, rru, brbubb, bwb, ggbr, rgb, gwrwb, grbgw, rrwuruw, wruuwgrb, brruubu, bgwbu, grr, ubru, rbrr, wubwuub, grbr
 
bwbbrrgrrbrggubuggwgguguburbbgbgrruggugbggggb
bwbwrrugrurrubugwggwugugwggbbwrwbgugwwbgrububrbuuwrrur
rwggubrbububggwwwrgwrgbrrrwgrwwrwwubuwuwrrbbr
ruwbrgguwubbwwrbruwwwgwrbwburrggwwgbrgwrwubur
wwugrrwrwwruwwgrgbwwrbbrbrwrwbbrburgwgrggugr
bbbbgrubuuwgrwuwubuwgrwgugwruuwwbruwrbbrwurgwurugrugrbbb
uuuwuuwurrwrggruwgwwuggwgruguwgwbgubwrgrwwguuburr
wwgwuurwwrbrwbruugbrbubwrwgbbrruwurrbbugwbr
".to_string()
    }
    #[test]
    fn parse_test() {
        let (available, desired) = parse(input());
        assert_eq!(available, vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"]);
        assert_eq!(desired, vec!["brwrr", "bggr", "gbbr", "rrbgbr", "ubwu", "bwurrg", "bbrgwb", "brgr"]);
    }
    #[test]
    fn awkward_input_parse_test() {
        let (available, desired) = parse(awkward_input());
        assert_eq!(available.len(), 447);
        assert_eq!(desired.len(), 15);
        println!("awkward_input_parse_test available: {:#?}", available);
        println!("awkward_input_parse_test desired: {:#?}", desired);
    }
    mod is_possible_tests {
        use super::*;
        #[test]
        fn possible_simple_test() {
            let available: Vec<String> = vec!["r", "wr"].into_iter().map(|x| x.to_string()).collect();
            let desired = "rwrr";
            assert_eq!(is_possible(&available, desired), true);
        }

        #[test]
        fn possible_test() {
            let available: Vec<String> = vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"].into_iter().map(|x| x.to_string()).collect();
            let desired = "brwrrbwurbgb";
            assert_eq!(is_possible(&available, desired), true);
        }

        #[test]
        fn impossible() {
            let available: Vec<String> = vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"].into_iter().map(|x| x.to_string()).collect();
            let desired = "brwrrrbwb";
            assert_eq!(is_possible(&available, desired), false);
        }

        #[test]
        fn exact_match() {
            let available: Vec<String> = vec!["rbwr"].into_iter().map(|x| x.to_string()).collect();
            let desired = "rbwr";
            assert_eq!(is_possible(&available, desired), true);
        }

        #[test]
        fn awkward() {
            let (available, desired) = parse(awkward_input());
            assert_eq!(part_1(&available,&desired), "7");
        }
    }
    mod part_1_tests {
        use super::*;
        #[test]
        fn part_1_test() {
            let (available, desired) = parse(input());
            assert_eq!(part_1(&available, &desired), "6");
        }
    }
}
