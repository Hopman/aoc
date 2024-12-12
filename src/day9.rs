use std::{collections::VecDeque, fmt::Display, fs::read_to_string};

pub fn day9(inputpath: &str) -> usize {
    let text = read_to_string(inputpath).unwrap();

    let mut disk: Vec<Block> = text
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .enumerate()
        .map(|(i, c)| {
            if i % 2 == 0 {
                Block {
                    name: i / 2,
                    size: c,
                    blocktype: BT::File,
                    right_pos: false,
                }
            } else {
                Block {
                    name: 0,
                    size: c,
                    blocktype: BT::Space,
                    right_pos: false,
                }
            }
        })
        .collect();

    loop {
        if let Some(file_pos) = disk
            .iter()
            .rposition(|f| f.blocktype == BT::File && !f.right_pos)
        {
            if let Some(space_pos) = disk
                .iter()
                .position(|e| e.blocktype == BT::Space && e.size >= disk[file_pos].size)
            {
                if file_pos <= space_pos {
                    disk[file_pos].right_pos = true;
                    continue;
                }
                let size_diff = disk[space_pos].size - disk[file_pos].size;
                let mut one = 0;
                if size_diff != 0 {
                    disk.insert(
                        space_pos + 1,
                        Block {
                            size: size_diff,
                            name: 0,
                            blocktype: BT::Space,
                            right_pos: false,
                        },
                    );
                    one = 1;
                    disk[space_pos].size -= size_diff;
                }
                disk.swap(space_pos, file_pos + one);
                disk[space_pos].right_pos = true;
            } else {
                disk[file_pos].right_pos = true;
            }
        } else {
            break;
        }
    }

    let flat: Vec<_> = disk
        .iter()
        .map(|f| vec![f.name; f.size])
        .flatten()
        .enumerate()
        .collect();

    let sum: usize = disk
        .iter()
        .map(|f| vec![f.name; f.size])
        .flatten()
        .enumerate()
        .map(|(i, f)| i * f)
        .sum();

    return sum;
    //return 0;
}

#[derive(Debug)]
struct Block {
    size: usize,
    name: usize,
    blocktype: BT,
    right_pos: bool,
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.blocktype == BT::File {
            for _ in 0..self.size {
                write!(f, "{}", self.name)?
            }
        } else {
            for _ in 0..self.size {
                if self.right_pos {
                    write!(f, ".")?
                } else {
                    write!(f, ",")?
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum BT {
    File,
    Space,
}

fn print_disk(disk: &Vec<Block>) {
    for block in disk {
        print!("{}", block);
    }
    println!();
}
