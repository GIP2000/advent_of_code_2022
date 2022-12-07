use anyhow::{bail, Context, Result};
use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    fmt::Debug,
    rc::Rc,
    str::FromStr,
};

#[derive(Debug)]
struct File {
    pub name: String,
    pub size: u64,
}

struct Dir {
    pub name: String,
    pub contents: HashMap<String, FileType>,
    pub parent: Option<Rc<RefCell<Dir>>>,
    size: Cell<Option<u64>>,
}

impl FromIterator<Command> for Result<Rc<RefCell<Dir>>> {
    fn from_iter<T: IntoIterator<Item = Command>>(iter: T) -> Self {
        let mut iiter = iter.into_iter();
        let first_command = iiter.next().context("First value must exist")?;
        let name = match first_command {
            Command::CD(r) => r,
            Command::LS(_) => {
                bail!("First command must be cd")
            }
        };

        let fs = Rc::new(RefCell::new(Dir {
            name,
            ..Dir::default()
        }));
        let mut cd = fs.clone();

        for cmd in iiter {
            match cmd {
                Command::CD(folder_name) => {
                    if folder_name == ".." {
                        let parent = match &cd.borrow().parent {
                            Some(d) => d.clone(),
                            None => {
                                bail!("Error can't .. on root")
                            }
                        };
                        cd = parent;
                        continue;
                    }
                    let exists = match cd
                        .borrow()
                        .contents
                        .get(&folder_name)
                        .context("Folder should exist")?
                    {
                        FileType::Dir(d) => d.clone(),
                        FileType::File(_) => {
                            bail!("Value must be Dir")
                        }
                    };
                    cd = exists;
                    continue;
                }
                Command::LS(fts) => {
                    cd.borrow_mut().contents = fts
                        .into_iter()
                        .map(|ft| match ft {
                            FileType::Dir(dir) => {
                                let name = dir.borrow().name.clone();
                                dir.borrow_mut().parent = Some(cd.clone());
                                return (name, FileType::Dir(dir));
                            }
                            FileType::File(file) => (file.name.clone(), FileType::File(file)),
                        })
                        .collect::<HashMap<_, _>>();
                }
            };
        }
        Ok(fs)
    }
}

impl Default for Dir {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            contents: HashMap::new(),
            parent: None,
            size: Cell::new(None),
        }
    }
}

impl Dir {
    pub fn build_fs(input: &str) -> Result<Rc<RefCell<Self>>> {
        return input
            .split("$ ")
            .filter(|s| !s.is_empty())
            .flat_map(str::parse::<Command>)
            .collect::<Result<Rc<RefCell<Self>>>>();
    }

    pub fn get_size(&self) -> u64 {
        if let None = self.size.get() {
            self.size
                .set(Some(self.contents.iter().fold(0, |acc, (_, c)| {
                    acc + match c {
                        FileType::Dir(d) => d.borrow().get_size(),
                        FileType::File(f) => f.size,
                    }
                })));
        }
        return self.size.get().unwrap();
    }
}

impl Debug for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}: {:?}", self.name, self.contents)
    }
}

#[derive(Debug)]
enum FileType {
    Dir(Rc<RefCell<Dir>>),
    File(File),
}

impl FromStr for FileType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s
            .split_once(" ")
            .context("Invalid cmd format for parsing")?;
        match first {
            "dir" => Ok(FileType::Dir(Rc::new(RefCell::new(Dir {
                name: second.to_string(),
                contents: HashMap::new(),
                ..Dir::default()
            })))),
            _ => {
                let size = first.parse::<u64>()?;
                return Ok(FileType::File(File {
                    name: second.to_string(),
                    size,
                }));
            }
        }
    }
}

#[derive(Debug)]
enum Command {
    CD(String),
    LS(Vec<FileType>),
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cmd, rest) = s
            .split_once("\n")
            .context("Invalid cmd format for parsing")?;
        match &cmd[0..2] {
            "cd" => Ok(Self::CD(cmd[3..].to_string())),
            "ls" => {
                let files = rest
                    .lines()
                    .map(str::parse::<FileType>)
                    .collect::<Result<Vec<_>>>()?;
                return Ok(Self::LS(files));
            }
            _ => {
                bail!("");
            }
        }
    }
}

fn recurssive_sum(root: &Rc<RefCell<Dir>>, sum: &mut u64) {
    let cur_size = root.borrow().get_size();
    if cur_size < 100000 {
        *sum += cur_size;
    }
    for (_, ft) in root.borrow().contents.iter() {
        match ft {
            FileType::Dir(d) => {
                recurssive_sum(d, sum);
            }
            FileType::File(_) => {}
        };
    }
}

fn recurssive_min(root: &Rc<RefCell<Dir>>, min: &mut u64, free_space: &u64) {
    let cur_min = root.borrow().get_size();
    if cur_min + *free_space >= 30000000 && cur_min < *min {
        *min = cur_min;
    }
    for (_, ft) in root.borrow().contents.iter() {
        match ft {
            FileType::Dir(d) => {
                recurssive_min(d, min, free_space);
            }
            FileType::File(_) => {}
        };
    }
}
pub fn part_1(input: &str) -> u64 {
    let fs = Dir::build_fs(input).unwrap();

    let mut sum = 0u64;
    recurssive_sum(&fs, &mut sum);

    return sum;
}
pub fn part_2(input: &str) -> u64 {
    let fs = Dir::build_fs(input).unwrap();

    let mut min = u64::MAX;
    let cur_min = fs.borrow().get_size();
    let free_space = 70000000 - cur_min;
    recurssive_min(&fs, &mut min, &free_space);
    return min;
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &'static str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 95437);
    }
    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 24933642);
    }
}
