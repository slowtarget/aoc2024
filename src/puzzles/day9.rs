use std::time::Instant;

/// Parse all digits from the input into an array of integers.
fn parse_digits(input: &str) -> Vec<usize> {
    input
        .chars()
        .filter_map(|ch| ch.to_digit(10).map(|d| d as usize))
        .collect()
}

/// Create the initial disk array based on parsed digits.
fn create_disk(digits: &[usize]) -> Vec<i32> {
    let disk_size: usize = digits.iter().sum(); // Total size of the disk
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
        if read_pos == 0 { break; } // Prevent underflow
        read_pos -= 1;
    }
}
/// Compact the disk by moving whole files to the leftmost available span of free spaces.
fn compact_disk_by_files(disk: &mut Vec<i32>) {
    // Identify all files and their positions
    let mut files = Vec::new();
    let mut current_file_id = None;
    let mut start = 0;

    for (pos, &block) in disk.iter().enumerate() {
        match block {
            id if id >= 0 => {
                // New file found
                if current_file_id.is_none() {
                    current_file_id = Some(id);
                    start = pos;
                } else if current_file_id != Some(id) {
                    files.push((current_file_id.expect("should be something here 83"), start, pos - 1));
                    current_file_id = Some(id);
                    start = pos;
                }
            }
            -1 => {
                // Free space
                if let Some(file_id) = current_file_id {
                    files.push((file_id, start, pos - 1));
                    current_file_id = None;
                }
            }
            _ => {}
        }
    }

    // Add the last file if it ends at the disk's end
    if let Some(file_id) = current_file_id {
        files.push((file_id, start, disk.len() - 1));
    }
    // Sort files by descending file ID
    files.sort_by(|a, b| b.0.cmp(&a.0));
    // Try to move each file
    for (file_id, start, end) in files {
        let file_length = end - start + 1;

        // Find the leftmost span of free spaces that fits the file
        let mut free_start = None;
        let mut free_length = 0;

        for (pos, &block) in disk[..start].iter().enumerate() {
            if block == -1 {
                if free_start.is_none() {
                    free_start = Some(pos);
                }
                free_length += 1;

                if free_length == file_length {
                    // Found a span large enough
                    let free_pos = free_start.unwrap();

                    // Move the file
                    for i in 0..file_length {
                        disk[free_pos + i] = file_id;
                        disk[start + i] = -1;
                    }
                    break;
                }
            } else {
                free_start = None;
                free_length = 0;
            }
        }
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
pub fn solve(input: String) -> (i64, i64) {
    let start = Instant::now();
    let  digits = parse_digits(&input);
    let mut disk = populate_disk(&digits);
    let mut disk2 = disk.clone();
    let parse_duration = start.elapsed();

    let start_compact = Instant::now();
    compact_disk(&mut disk);
    let part1 = calculate_checksum(&disk);
    let part1_duration = start_compact.elapsed();
    let start_compact = Instant::now();
    compact_disk_by_files(&mut disk2);
    let part2 = calculate_checksum(&disk2);
    let part2_duration = start_compact.elapsed();

    println!("Checksums: {} {}", part1, part2);
    println!("Parsing took: {} microseconds", parse_duration.as_micros());
    println!("Compaction and checksum took: {}, {} microseconds", part1_duration.as_micros(), part2_duration.as_micros());
    println!("total duration {} ms", start.elapsed().as_millis());
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    mod integration {
        use crate::puzzles::day9::solve;

        #[test]
        fn provided() {
            assert_eq!(solve(String::from("2333133121414131402")),(1928, 2858))
        }
    }
    #[test]
    fn test_parse_digits() {
        let input = "2333133121414131402";
        let digits = parse_digits(input);
        assert_eq!(digits, vec![2, 3, 3, 3, 1, 3, 3, 1, 2, 1, 4, 1, 4, 1, 3, 1, 4, 0, 2]);
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
            let digits = vec![2, 3,   3, 3,   1, 2,   1, 4,   1, 4,   1, 3,   1, 4,   1, 2];
                       //                  0       1       2       3       4       5       6       7
            let disk = populate_disk(&digits);
            assert_eq!(
                disk,
                vec![0, 0, -1, -1, -1,    
                     1, 1, 1, -1, -1, -1,   
                     2, -1, -1,   
                     3, -1, -1, -1, -1, 
                     4,-1, -1, -1, -1,
                     5,-1, -1, -1,
                     6, -1, -1, -1, -1,
                     7,-1, -1
                ]
            );
        }
        #[test]
        fn simple() {
            let digits = vec![2, 3];
            let disk = populate_disk(&digits);
            assert_eq!(
                disk,
                vec![
                    0, 0, -1, -1, -1
                ]
            );
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
    mod compact_disk_by_files {
        use crate::puzzles::day9::compact_disk_by_files;

        #[test]
        fn test_compact_disk_by_files() {
            let mut disk = vec![
                0, 0, 
                -1, -1, -1, 
                1, 1, 1, 
                -1, -1, -1, 
                2, 
                -1, -1, -1, 
                3, 3, 3, 
                -1, 
                4, 4, 
                -1, 
                5, 5, 5, 5,
                -1, 
                6, 6, 6, 6, 
                -1, 
                7, 7, 7, 
                -1, 
                8, 8, 8, 8, 
                9, 9,
            ];
            compact_disk_by_files(&mut disk);
            assert_eq!(
                disk,
                vec![
                    0, 0, 
                    9, 9, 
                    2, 
                    1, 1, 1, 
                    7, 7, 7, 
                    -1, 
                    4, 4, 
                    -1, 
                    3, 3, 3, 
                    -1, -1, -1, -1,
                    5, 5, 5, 5, 
                    -1,
                    6, 6, 6, 6, 
                    -1, -1, -1, -1, -1, 
                    8,8,8,8,
                    -1, -1
                ]
            );
        }
        #[test]
        fn simple() {
            let mut disk = vec![-1, -1, 0, 0];
            compact_disk_by_files(&mut disk);
            assert_eq!(disk, vec![0, 0, -1, -1]);
        }
        #[test]
        fn more() {
            let mut disk = vec![-1, -1, 0, 0, 1, 1];
            compact_disk_by_files(&mut disk);
            assert_eq!(disk, vec![1, 1, 0, 0, -1, -1]);
        }
    }
    

    mod calculate_checksum {
        use crate::puzzles::day9::calculate_checksum;

        #[test]
        fn test_calculate_checksum() {
            let disk = vec![
                0, 0, 1, 1, 1, 2, 3, 3, 3, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 8, 8, 8, 8, 9, 9, -1,
                -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            ];
            let checksum = calculate_checksum(&disk);
            assert_eq!(checksum, 2453);
        }
        #[test]
        fn simple() {
            let disk = vec![
                0, 0, 1, 1, 1, 
            ];
            let checksum = calculate_checksum(&disk);
            assert_eq!(checksum, 9); // 0 + 0 + 2 + 3 + 4
        }
        #[test]
        fn more() {
            let disk = vec![
                0, 0, 1, 1, 1, 2, 3, 4
            ];
            let checksum = calculate_checksum(&disk);
            assert_eq!(checksum, 65); // 0 + 0 + 2 + 3 + 4 + 10 + 18 + 28
        }
    }
}
