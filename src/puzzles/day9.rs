use std::time::Instant;
use timing_util::measure_time;

//add clone
#[derive(Debug, Clone, PartialEq, Default)]
struct File {
    id: usize,
    size: usize,
}
#[derive(Debug, Clone, PartialEq, Default)]
struct Partition {
    id: usize,
    data: [Option<File>; 9],
    free: usize,
}

/// Parse all digits from the input into an array of integers.
fn parse_digits(input: &str) -> Vec<usize> {
    input
        .chars()
        .filter_map(|ch| ch.to_digit(10).map(|d| d as usize))
        .collect()
}

/// Chunk the digits array into an array of Partitions
fn chunk(input: &[usize]) -> Vec<Partition> {
    let capacity = input.len() / 2; // Assuming each file has size + free space
    let mut partitions = Vec::with_capacity(capacity);

    input.chunks(2).enumerate().for_each(|(id, chunk)| {
        let size = chunk[0];
        let free = if chunk.len() == 2 { chunk[1] } else { 0 };

        let file = File { size, id };
        let mut data: [Option<File>; 9] = Default::default();
        data[0] = Some(file);

        partitions.push(Partition { id, data, free });
    });

    partitions
}
fn compact_partitions(mut partitions: Vec<Partition>) -> Vec<Partition> {
    let mut last_free_of_size = [0; 9];

    // Iterate over partitions in reverse
    for i in (0..partitions.len()).rev() {
        // Clone the file to avoid immutable borrow conflicts
        let file = partitions[i].data[0].as_ref().cloned();

        if let Some(file) = file {
            let last = last_free_of_size[file.size - 1];
            if last < partitions[i].id {
                // Use split_at_mut to safely create non-overlapping mutable slices
                let (left, _right) = partitions.split_at_mut(i);

                // Check partitions in the range `last..i` for available space
                for candidate_partition in &mut left[last..] {
                    if candidate_partition.free >= file.size {
                        // Find the first empty slot in the candidate partition
                        if let Some(slot) = candidate_partition
                            .data
                            .iter_mut()
                            .find(|entry| entry.is_none())
                        {
                            // Move the file
                            *slot = Some(file.clone());
                            candidate_partition.free -= file.size;

                            // Update the last free of this size
                            last_free_of_size[file.size - 1] = candidate_partition.id;

                            // Set the file.id in the original position to 0
                            if let Some(original_file) = partitions[i].data[0].as_mut() {
                                original_file.id = 0;
                                // 0 works it will not now accrue to the check sum, and this preserves the empty space in the correct position
                                // there is a weird edge case where a small size file followed by a large space could have some other file moved into its free space, but then get moved itself.
                                // so long as we set the id to 0, and then process the checksum normally, this won't be an issue.
                                // I'll write the test case for this situation
                            }
                            break;
                        }
                    }
                }
            }
        }
    }
    partitions
}

/// Create the initial disk array based on parsed digits.
fn create_disk(digits: &[usize]) -> Vec<i32> {
    let disk_size: usize = digits.iter().sum(); // Total size of the disk
    println!("creating a disk : {}", &disk_size);
    vec![-1; disk_size]
}

/// Fill the disk array with file IDs and free spaces based on parsed digits.
fn populate_disk(digits: &[usize]) -> Vec<i32> {
    let mut disk = create_disk(digits);
    let mut file_id = 0; // Start file IDs at 0
    let mut pos = 0;

    let mut iter = digits.iter();
    while let Some(&file_len) = iter.next() {
        for _ in 0..file_len {
            disk[pos] = file_id;
            pos += 1;
        }
        file_id += 1;

        if let Some(&free_len) = iter.next() {
            pos += free_len; // Skip free spaces
        }
    }
    disk
}

/// Compact the disk by moving blocks one at a time from the end to the leftmost free space.
fn compact_disk(disk: &mut Vec<i32>) {
    let mut write_pos = 0; // Pointer to the leftmost free space
    let mut read_pos = disk.len() - 1; // Start from the last block

    while read_pos >= write_pos {
        if disk[read_pos] != -1 {
            // Find the next free space on the left
            while write_pos < disk.len() && disk[write_pos] != -1 {
                write_pos += 1;
            }
            if write_pos < read_pos {
                disk[write_pos] = disk[read_pos];
                disk[read_pos] = -1;
            }
        }
        if read_pos == 0 {
            break;
        } // Prevent underflow
        read_pos -= 1;
    }
}

/// Calculate the checksum of the disk.
fn calculate_checksum(disk: &[i32]) -> i64 {
    disk.iter()
        .enumerate()
        .filter(|(_, &id)| id != -1)
        .map(|(pos, &id)| pos as i64 * id as i64)
        .sum()
}

/// Solve function for parsing, compaction, and checksum calculation.
pub fn solve(input: String) -> (i64, i64, i64) {
    let start = Instant::now();
    let digits = parse_digits(&input);
    let mut disk = populate_disk(&digits);
    let parse_duration = start.elapsed();

    let start_compact = Instant::now();
    compact_disk(&mut disk);
    let part1 = calculate_checksum(&disk);
    let part1_duration = start_compact.elapsed();

    let start_compact = Instant::now();
    let partitions = measure_time!({ compact_partitions(measure_time!({ chunk(&digits) })) });

    let part2_2: i64 = measure_time!({ calculate_checksum_from_partition(&partitions) });
    let part2_2_duration = start_compact.elapsed();

    println!("Checksums: {} {} 0", part1, part2_2);
    println!("Parsing took: {} microseconds", parse_duration.as_micros());
    println!(
        "Compaction and checksum took: {}, {} microseconds",
        part1_duration.as_micros(),
        part2_2_duration.as_micros()
    );
    println!("total duration {:?}", start.elapsed());
    (part1, part2_2, part2_2)
}

fn calculate_checksum_from_partition(partitions: &Vec<Partition>) -> i64 {
    let mut sum: i64 = 0;
    let mut index = 0;

    for partition in partitions {
        for maybe_file in &partition.data {
            if let Some(file) = maybe_file {
                let id = file.id as i64;
                for _ in 0..file.size {
                    sum += id * index;
                    index += 1;
                }
            }
        }
        index += partition.free as i64;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    mod integration {
        use crate::puzzles::day9::solve;

        #[test]
        fn provided() {
            assert_eq!(
                solve(String::from("2333133121414131402")),
                (1928, 2858, 2858)
            )
        }
        #[test]
        fn simple() {
            assert_eq!(solve(String::from("1234")), (6, 12, 12))
        }
        #[test]
        fn simple2() {
            assert_eq!(solve(String::from("1245")), (10, 18, 18))
        }
        #[test]
        fn simpler() {
            assert_eq!(solve(String::from("10")), (0, 0, 0))
        }
        #[test]
        fn simpler1() {
            assert_eq!(solve(String::from("1010")), (1, 1, 1))
        }
        #[test]
        fn simpler2() {
            assert_eq!(solve(String::from("101010")), (5, 5, 5))
        }
        #[test]
        fn simpler_move() {
            assert_eq!(solve(String::from("1110")), (1, 1, 1))
        }
        #[test]
        fn simpler_move2() {
            assert_eq!(solve(String::from("101110")), (5, 5, 5))
        }
    }
    #[test]
    fn test_parse_digits() {
        let input = "2333133121414131402";
        let digits = parse_digits(input);
        assert_eq!(
            digits,
            vec![2, 3, 3, 3, 1, 3, 3, 1, 2, 1, 4, 1, 4, 1, 3, 1, 4, 0, 2]
        );
    }

    mod create_disk {
        use crate::puzzles::day9::create_disk;

        #[test]
        fn test_create_disk() {
            let digits = vec![2, 3, 3, 3, 1, 2, 1, 4, 1, 4, 1, 3, 1, 4, 0, 2];
            let disk = create_disk(&digits);
            assert_eq!(disk.len(), 35); // Sum of digits is 44
            assert!(disk.iter().all(|&x| x == -1));
        }
        #[test]
        fn simple() {
            let digits = vec![2, 3];
            let disk = create_disk(&digits);
            assert_eq!(disk.len(), 5);
            assert!(disk.iter().all(|&x| x == -1));
        }
    }

    mod populate_disk {
        use crate::puzzles::day9::populate_disk;

        #[test]
        fn test_populate_disk() {
            let digits = vec![2, 3, 3, 3, 1, 2, 1, 4, 1, 4, 1, 3, 1, 4, 1, 2];
            //                  0       1       2       3       4       5       6       7
            let disk = populate_disk(&digits);
            assert_eq!(
                disk,
                vec![
                    0, 0, -1, -1, -1, 1, 1, 1, -1, -1, -1, 2, -1, -1, 3, -1, -1, -1, -1, 4, -1, -1,
                    -1, -1, 5, -1, -1, -1, 6, -1, -1, -1, -1, 7, -1, -1
                ]
            );
        }
        #[test]
        fn simple() {
            let digits = vec![2, 3];
            let disk = populate_disk(&digits);
            assert_eq!(disk, vec![0, 0, -1, -1, -1]);
        }
    }

    mod compact_example {
        use crate::puzzles::day9::compact_disk;

        #[test]
        fn test_compact_example_steps() {
            let mut disk = vec![0, -1, -1, 1, 1, 1, -1, -1, -1, 2, 2, 2, 2, 2];
            compact_disk(&mut disk);
            assert_eq!(disk, vec![0, 2, 2, 1, 1, 1, 2, 2, 2, -1, -1, -1, -1, -1]);
        }
    }
    mod chunk {
        use crate::puzzles::day9::{chunk, File, Partition};

        #[test]
        fn simple() {
            let input = [1, 2, 3, 4];
            let expected = [
                Partition {
                    id: 0,
                    data: [
                        Some(File { id: 0, size: 1 }),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                    ],
                    free: 2,
                },
                Partition {
                    id: 1,
                    data: [
                        Some(File { id: 1, size: 3 }),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                    ],
                    free: 4,
                },
            ];
            assert_eq!(chunk(&input[..]), expected);
        }
        #[test]
        fn test_chunk_standard_case() {
            let input = &[10, 5, 20, 15, 30, 25];
            let result = chunk(input);

            let expected = vec![
                Partition {
                    id: 0,
                    data: {
                        let mut data: [Option<File>; 9] = Default::default();
                        data[0] = Some(File { size: 10, id: 0 });
                        data
                    },
                    free: 5,
                },
                Partition {
                    id: 1,
                    data: {
                        let mut data: [Option<File>; 9] = Default::default();
                        data[0] = Some(File { size: 20, id: 1 });
                        data
                    },
                    free: 15,
                },
                Partition {
                    id: 2,
                    data: {
                        let mut data: [Option<File>; 9] = Default::default();
                        data[0] = Some(File { size: 30, id: 2 });
                        data
                    },
                    free: 25,
                },
            ];

            assert_eq!(result, expected);
        }

        #[test]
        fn test_chunk_odd_input() {
            let input = &[10, 5, 20];
            let result = chunk(input);

            let expected = vec![
                Partition {
                    id: 0,
                    data: {
                        let mut data: [Option<File>; 9] = Default::default();
                        data[0] = Some(File { size: 10, id: 0 });
                        data
                    },
                    free: 5,
                },
                Partition {
                    id: 1,
                    data: {
                        let mut data: [Option<File>; 9] = Default::default();
                        data[0] = Some(File { size: 20, id: 1 });
                        data
                    },
                    free: 0,
                },
            ];

            assert_eq!(result, expected);
        }

        #[test]
        fn test_chunk_empty_input() {
            let input = &[];
            let result = chunk(input);
            let expected: Vec<Partition> = vec![];
            assert_eq!(result, expected);
        }

        #[test]
        fn test_chunk_large_input() {
            let input: Vec<usize> = (1..=20).collect();
            let result = chunk(&input);

            assert_eq!(result.len(), 10);
            for (i, partition) in result.iter().enumerate() {
                assert_eq!(partition.id, i);
                assert_eq!(
                    partition.data[0],
                    Some(File {
                        size: input[i * 2],
                        id: i
                    })
                );
                if i * 2 + 1 < input.len() {
                    assert_eq!(partition.free, input[i * 2 + 1]);
                } else {
                    assert_eq!(partition.free, 0);
                }
            }
        }
    }
    mod compact_partitions {
        use super::*;
        #[cfg(test)]
        #[test]
        fn test_compact_basic_case() {
            let input = vec![
                Partition {
                    id: 0,
                    data: {
                        let mut data: [Option<File>; 9] = Default::default();
                        data[0] = Some(File { size: 3, id: 0 });
                        data
                    },
                    free: 5,
                },
                Partition {
                    id: 1,
                    data: {
                        let mut data: [Option<File>; 9] = Default::default();
                        data[0] = Some(File { size: 3, id: 1 });
                        data
                    },
                    free: 5,
                },
            ];

            let actual = compact_partitions(input.clone());

            // The file in partition 0 should move to partition 1
            let expected = vec![
                Partition {
                    id: 0,
                    data: {
                        let mut data: [Option<File>; 9] = Default::default();
                        data[0] = Some(File { size: 3, id: 0 });
                        data[1] = Some(File { size: 3, id: 1 });
                        data
                    },
                    free: 2,
                },
                Partition {
                    id: 1,
                    data: {
                        let mut data: [Option<File>; 9] = Default::default();
                        data[0] = Some(File { size: 3, id: 0 });
                        data
                    },
                    free: 5,
                },
            ];

            assert_eq!(actual, expected);
        }

        #[test]
        fn test_compact_edge_case_small_large_space() {
            let partitions = vec![
                Partition {
                    id: 0,
                    data: {
                        let mut data: [Option<File>; 9] = Default::default();
                        data[0] = Some(File { size: 1, id: 1 });
                        data
                    },
                    free: 10,
                },
                Partition {
                    id: 1,
                    data: Default::default(),
                    free: 5,
                },
            ];

            let result = compact_partitions(partitions.clone());

            // The file in partition 0 should remain in place, as the free space is already sufficient
            let expected = vec![
                Partition {
                    id: 0,
                    data: {
                        let mut data: [Option<File>; 9] = Default::default();
                        data[0] = Some(File { size: 1, id: 1 });
                        data
                    },
                    free: 10,
                },
                Partition {
                    id: 1,
                    data: Default::default(),
                    free: 5,
                },
            ];

            assert_eq!(result, expected);
        }

        #[test]
        fn test_compact_no_free_space() {
            let partitions = vec![
                Partition {
                    id: 0,
                    data: {
                        let mut data: [Option<File>; 9] = Default::default();
                        data[0] = Some(File { size: 3, id: 1 });
                        data
                    },
                    free: 0,
                },
                Partition {
                    id: 1,
                    data: Default::default(),
                    free: 2,
                },
            ];

            let result = compact_partitions(partitions.clone());

            // No file should move as there is no free space sufficient for the file
            assert_eq!(result, partitions);
        }

        #[test]
        fn test_compact_empty_partitions() {
            let partitions = vec![
                Partition {
                    id: 0,
                    data: Default::default(),
                    free: 10,
                },
                Partition {
                    id: 1,
                    data: Default::default(),
                    free: 5,
                },
            ];

            let result = compact_partitions(partitions.clone());

            // The partitions are already empty, no changes expected
            assert_eq!(result, partitions);
        }

        #[test]
        fn test_compact_already_compact() {
            let partitions = vec![
                Partition {
                    id: 0,
                    data: {
                        let mut data: [Option<File>; 9] = Default::default();
                        data[0] = Some(File { size: 3, id: 1 });
                        data
                    },
                    free: 10,
                },
                Partition {
                    id: 1,
                    data: Default::default(),
                    free: 5,
                },
            ];

            let result = compact_partitions(partitions.clone());

            // The partitions are already compact
            assert_eq!(result, partitions);
        }

        #[test]
        fn weird() {
            let input = vec![
                Partition {
                    id: 0,
                    data: [
                        Some(File { id: 0, size: 1 }),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                    ],
                    free: 1, // 1 will get moved into here
                },
                Partition {
                    id: 1,
                    data: [
                        Some(File { id: 1, size: 1 }),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                    ],
                    free: 5, // 2 will get moved in here
                },
                Partition {
                    id: 2,
                    data: [
                        Some(File { id: 2, size: 5 }),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                    ],
                    free: 0,
                },
            ];
            let expected = vec![
                Partition {
                    id: 0,
                    data: [
                        Some(File { id: 0, size: 1 }),
                        Some(File { id: 1, size: 1 }),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                    ],
                    free: 0, // 1 has been moved into here
                },
                Partition {
                    id: 1,
                    data: [
                        Some(File {
                            id: 0, // 1 was moved away
                            size: 1,
                        }),
                        Some(File { id: 2, size: 5 }),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                    ],
                    free: 0, // 2 was moved in here
                },
                Partition {
                    id: 2,
                    data: [
                        Some(File {
                            id: 0, // 2 was moved away
                            size: 5,
                        }),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                    ],
                    free: 0,
                },
            ];
            assert_eq!(compact_partitions(input), expected);
        }
    }
    mod calculate_checksum_from_partition {
        use crate::puzzles::day9::{calculate_checksum_from_partition, File, Partition};

        #[test]
        fn simple_1() {
            let input = vec![Partition {
                id: 0,
                data: {
                    let mut data: [Option<File>; 9] = Default::default();
                    data[0] = Some(File { id: 0, size: 1 });
                    data[1] = Some(File { id: 1, size: 1 });
                    data
                },
                free: 0,
            }];
            assert_eq!(calculate_checksum_from_partition(&input), 1);
        }

        #[test]
        fn simple_5() {
            let input = vec![Partition {
                id: 0,
                data: {
                    let mut data: [Option<File>; 9] = Default::default();
                    data[0] = Some(File { id: 0, size: 1 });
                    data[1] = Some(File { id: 1, size: 1 });
                    data[2] = Some(File { id: 2, size: 1 });
                    data
                },
                free: 0,
            }];
            assert_eq!(calculate_checksum_from_partition(&input), 5);
        }

        #[test]
        fn multi_partition() {
            let input = vec![
                Partition {
                    id: 0,
                    data: {
                        let mut data: [Option<File>; 9] = Default::default();
                        data[0] = Some(File { id: 0, size: 1 });
                        data[1] = Some(File { id: 1, size: 1 });
                        data
                    },
                    free: 5,
                },
                Partition {
                    id: 1,
                    data: {
                        let mut data: [Option<File>; 9] = Default::default();
                        data[0] = Some(File { id: 2, size: 2 });
                        data[1] = Some(File { id: 3, size: 1 });
                        data
                    },
                    free: 3,
                },
            ];
            // Expected checksum:
            // Partition 0:
            // File 0 contributes: 0 * 0 = 0
            // File 1 contributes: 1 * 1 = 1
            // Partition 1:
            // File 2 contributes: 2 * 2 + 2 * 3 = 10
            // File 3 contributes: 3 * 4 = 12
            assert_eq!(calculate_checksum_from_partition(&input), 58);
        }

        #[test]
        fn empty_partition() {
            let input = vec![Partition {
                id: 0,
                data: Default::default(),
                free: 10,
            }];
            assert_eq!(calculate_checksum_from_partition(&input), 0);
        }

        #[test]
        fn no_files_in_partition() {
            let input = vec![
                Partition {
                    id: 0,
                    data: Default::default(),
                    free: 10,
                },
                Partition {
                    id: 1,
                    data: Default::default(),
                    free: 5,
                },
            ];
            assert_eq!(calculate_checksum_from_partition(&input), 0);
        }
    }

    mod calculate_checksum {
        use crate::puzzles::day9::calculate_checksum;

        #[test]
        fn test_calculate_checksum() {
            let disk = vec![
                0, 0, 1, 1, 1, 2, 3, 3, 3, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 8, 8, 8, 8, 9, 9,
                -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            ];
            let checksum = calculate_checksum(&disk);
            assert_eq!(checksum, 2453);
        }
        #[test]
        fn simple() {
            let disk = vec![0, 0, 1, 1, 1];
            let checksum = calculate_checksum(&disk);
            assert_eq!(checksum, 9); // 0 + 0 + 2 + 3 + 4
        }
        #[test]
        fn more() {
            let disk = vec![0, 0, 1, 1, 1, 2, 3, 4];
            let checksum = calculate_checksum(&disk);
            assert_eq!(checksum, 65); // 0 + 0 + 2 + 3 + 4 + 10 + 18 + 28
        }
    }
}
