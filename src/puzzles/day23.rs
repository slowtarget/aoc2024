use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, line_ending};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use std::collections::{HashMap, HashSet};

// --- Day 23: LAN Party ---
// read all connections in to a list of tuples
// populate a map for each connection: key: computer - value: list of connected computers
// for any computer starting with a t,
//  for each pair of connected computers
//     check for a connection between the two
//      if there is a connection, add the three computers to the set of connected computers
//
// lets give them all numeric id, and use a map to convert between the id and the computer name

fn parse_computer(input: &str) -> IResult<&str, &str> {
    let (input, computer) = alpha1(input)?;
    Ok((input, computer))
}
fn parse_pair(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, (a, b)) = separated_pair(parse_computer, tag("-"), parse_computer)(input)?;
    Ok((input, (a, b)))
}

fn parse(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    let (input, grid) = separated_list1(line_ending, parse_pair)(input)?;
    Ok((input, grid))
}
fn prep<'a>(
    pairs: &'a Vec<(&'a str, &'a str)>,
) -> (
    Vec<&'a str>,
    HashMap<&'a str, usize>,
    HashMap<usize, Vec<usize>>,
) {
    let computer_set: HashSet<&str> = pairs.iter().flat_map(|(a, b)| vec![*a, *b]).collect();
    let mut computer_lookup: HashMap<&str, usize> = HashMap::new();
    let mut computer_index: Vec<&str> = Vec::with_capacity(computer_set.len());
    let mut computer_map: HashMap<usize, Vec<usize>> = HashMap::new();
    computer_set.iter().enumerate().for_each(|(i, c)| {
        computer_lookup.insert(c, i);
        computer_index.push(c);
        computer_map.insert(i, [i].to_vec()); // add self to the list of connected computers helps with comparisons
    });

    computer_index.iter().enumerate().for_each(|(i, c)| {
        let entry = computer_map.get_mut(&i).unwrap();
        pairs.iter().for_each(|(a, b)| {
            if a == c {
                entry.push(*computer_lookup.get(b).unwrap());
            } else if b == c {
                entry.push(*computer_lookup.get(a).unwrap());
            }
        });
    });
    for (_k, v) in computer_map.iter_mut() {
        v.sort();
    }

    (computer_index, computer_lookup, computer_map)
}
fn part_1(computer_index: &Vec<&str>, computer_map: &HashMap<usize, Vec<usize>>) -> usize {
    let mut connected_computers: HashSet<usize> = HashSet::new();
    let base = computer_index.len();
    computer_index
        .iter()
        .enumerate()
        .filter(|(_i, c)| c.starts_with("t"))
        .for_each(|(a, _c)| {
            computer_map.get(&a).unwrap().iter().for_each(|b| {
                computer_map
                    .get(b)
                    .unwrap()
                    .iter()
                    .filter(|c| **c != a)
                    .filter(|c| computer_map.get(c).unwrap().contains(&a))
                    .map(|c| [a, *b, *c])
                    .map(|mut triple| {
                        triple.sort();
                        triple
                    })
                    .map(|triple| triple[0] + triple[1] * base + triple[2] * base * base)
                    .for_each(|triple| {
                        connected_computers.insert(triple);
                    });
            });
        });
    connected_computers.len()
}

fn combi<T: Clone>(n: usize, lst: &[T]) -> Vec<Vec<T>> {
    // If n == 0, the only "combination" is an empty vector
    if n == 0 {
        return vec![vec![]];
    }
    // If the list is empty but we still need to pick items, there are no combinations
    if lst.is_empty() {
        return vec![];
    }

    // 1) Pick the first element. We need one fewer item now, so n - 1.
    //    Recurse on the tail of the list. Then prepend the picked element to each result.
    let mut with_first = combi(n - 1, &lst[1..]);
    for combo in &mut with_first {
        combo.insert(0, lst[0].clone());
    }

    // 2) Skip the first element. Don't decrement n; still need n items.
    let without_first = combi(n, &lst[1..]);

    // Combine the two sets of results
    [with_first, without_first].concat()
}

fn part_2(computer_index: &Vec<&str>, computer_map: &HashMap<usize, Vec<usize>>) -> String {
    
    let mut max = 0;
    let mut found: Vec<usize> = Vec::with_capacity(14);

    for (_a, c) in computer_map.iter() {
        for i in 0..c.len() {
            let n = c.len() - i;
            if n <= max {
                break;
            }
            for combination in combi(n, &c) {
                // if we get all the friends in this combination and create a frequency distribution,
                // then if it's a match we will have the frequency of each computer in the group will be n
                let mut distribution: HashMap<usize, usize> = HashMap::new();
                combination.iter().for_each(|computer| {
                    computer_map
                        .get(computer)
                        .unwrap()
                        .iter()
                        .for_each(|friend| *distribution.entry(*friend).or_insert(0) += 1)
                });

                if combination.iter().map(|computer| distribution.get(computer).unwrap()).all(|x| *x == n) {
                    max = n;
                    found = combination;
                    println!("Found: {:?}", n);
                    break;
                }
            }
        }
    }
    let mut computers = found.iter().map(|i| computer_index[*i]).collect::<Vec<&str>>();
    computers.sort();
    computers.join(",")
}

pub(crate) fn solve(input: String) -> (String, String) {
    match parse(&input) {
        Ok((_remaining, pairs)) => {
            // println!("Parsed Pairs: {pairs:?}");
            // println!("Remaining: {remaining:?}");

            let (computer_index, computer_lookup, computer_map) = prep(&pairs);
            println!("Pairs: {:?}", pairs.len());
            println!("Computer Set: {:?}", computer_index.len());
            println!("Computer Lookup: {:?}", computer_lookup.len());
            println!("Computer Map: {:?}", computer_map.len());
            let part_1_result: usize = part_1(&computer_index, &computer_map);
            let part_2_result: String = part_2(&computer_index, &computer_map);
            (part_1_result.to_string(), part_2_result)
        }
        Err(err) => {
            eprintln!("Error parsing input: {err}");
            (String::new(), String::new())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn provided_part_1_test() {
        let pairs = &parse(input()).unwrap().1;
        let (computer_index, computer_lookup, computer_map) = prep(pairs);
        println!("Computer Index: {:?}", computer_index);
        println!("Computer Map: {:?}", computer_map);
        assert_eq!(part_1(&computer_index, &computer_map), 7);
    }
    #[test]
    fn provided_part_2_test() {
        let pairs = &parse(input()).unwrap().1;
        let (computer_index, computer_lookup, computer_map) = prep(pairs);
        println!("Computer Index: {:?}", computer_index);
        println!("Computer Map: {:?}", computer_map);
        assert_eq!(
            part_2(&computer_index, &computer_map),
            "co,de,ka,ta".to_string()
        );
    }

    mod combi_tests {
        use super::*;
        #[test]
        fn test_combi_pick_2_from_4() {
            let items = vec![1, 2, 3, 4];
            let result = combi(2, &items);
            assert_eq!(result.len(), 6);
            assert_eq!(result, [[1, 2], [1, 3], [1, 4], [2, 3], [2, 4], [3, 4]]);
        }
        #[test]
        fn test_combi_pick_3_from_4() {
            let items = vec![1, 2, 3, 4];
            let result = combi(3, &items);
            assert_eq!(result.len(), 4);
            assert_eq!(result, [[1, 2, 3], [1, 2, 4], [1, 3, 4], [2, 3, 4]]);
        }
        #[test]
        fn test_combi_pick_2_from_3() {
            let result = combi(2, &vec![1, 2, 3]);
            // We expect all 2-element subsets: [1,2], [1,3], and [2,3]
            assert_eq!(result, vec![vec![1, 2], vec![1, 3], vec![2, 3]]);
        }

        #[test]
        fn test_combi_pick_0_from_3() {
            let result = combi(0, &vec![1, 2, 3]);
            // Picking 0 elements should yield the empty subset as the only "combination"
            assert_eq!(result, vec![vec![]]);
        }

        #[test]
        fn test_combi_pick_3_from_3() {
            let result = combi(3, &vec![1, 2, 3]);
            // Picking all elements from the list should yield exactly one combination
            assert_eq!(result, vec![vec![1, 2, 3]]);
        }

        #[test]
        fn test_combi_pick_more_than_available() {
            let result = combi(4, &vec![1, 2, 3]);
            // It's impossible to pick 4 elements from a 3-element list, so this should be empty
            assert_eq!(result, Vec::<Vec<i32>>::new());
        }
    }
    fn input() -> &'static str {
        "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"
    }
}
