use std::cell::RefCell;
use std::rc::Rc;
use std::collections::BTreeMap;

type InputType = Rc<RefCell<Dir>>;
type OutputType = usize;

#[derive(Debug)]
pub struct Dir {
    name: String,
    full_path: String,
    dirs: Vec<Rc<RefCell<Dir>>>,
    files: Vec<File>,
}

impl Dir {
    fn new(path: &str, name: &str) -> Self {
        Dir {
            name: name.to_string(),
            full_path: path.to_string(),
            dirs: vec![],
            files: vec![],
        }
    }
}

impl File {
    fn new(name: &str, size: usize) -> Self {
        File {
            name: name.to_string(),
            size,
        }
    }
}

#[derive(Debug)]
pub struct File {
    #[allow(dead_code)]
    name: String,
    size: usize,
}

pub enum Command {
    CD(String),
    LS(),
}

fn get_subdir_if_exists(dir: &Rc<RefCell<Dir>>, dir_name: &str) -> Option<Rc<RefCell<Dir>>> {
    let next_dir_ref = dir.as_ref().clone().borrow();
    next_dir_ref
        .dirs
        .iter()
        .cloned()
        .find(|d| d.as_ref().borrow().name.to_owned() == dir_name)
}

fn calculate_naive_dir_size(dir: &Rc<RefCell<Dir>>,all_dirs : &mut BTreeMap<String,usize>) -> usize {
    let mut size = 0;
    let current_dir_name = dir.as_ref().borrow().name.clone();
    let cur_path = dir.as_ref().borrow().full_path.clone();
    //println!("Looking at {}",current_dir_name); 
    for file in dir.as_ref().clone().borrow().files.iter() {
        size += file.size;
    }
    for dir in dir.as_ref().borrow().dirs.iter() {
        size += calculate_naive_dir_size(&dir,all_dirs);
    }

    all_dirs.insert(format!("{}/{}",cur_path,current_dir_name), size);
    size
}

#[aoc_generator(day7)]
fn day7_parse(input: &str) -> InputType {
    let top_level = Rc::new(RefCell::new(Dir::new("","/")));
    let mut cwd = vec![top_level.clone()]; //Directory stack

    let mut lines = input.lines().peekable();

    while let Some(line) = lines.next() {
        //println!("Current cwd: {:?}", cwd.iter().map(|d| d.as_ref().borrow().name.clone()).collect::<Vec<_>>());
        let cur_path =  cwd.iter().map(|d| d.as_ref().borrow().name.clone()).collect::<Vec<String>>().join("/");
        if line.starts_with("$ cd") {
            let mut line_spaces = line.split(" ");
            let dir_name = line_spaces.nth(2).unwrap();
            //println!("Saw dir name: {}",dir_name);
            if dir_name == ".." {
                //If you're going up a directory, pop one off the stack
                cwd.pop();
            } else if dir_name == "/" {
                //If it's the top level, just go to the top
                cwd.clear();
                cwd.push(top_level.clone());
            } else {
                //If you need to move down a dir, check if it exists first, if it doesn't, create
                //it and traverse down
                let next_dir = get_subdir_if_exists(&cwd.last().unwrap().clone(), dir_name);


                if next_dir.is_some() {
                    cwd.push(next_dir.unwrap().clone());
                } else {
                    //println!("Making new dir '{}'",dir_name);
                    let new_dir = Rc::new(RefCell::new(Dir::new(&cur_path,dir_name)));
                    let cur_dir = cwd.last_mut().unwrap().clone();

                    cur_dir.as_ref().borrow_mut().dirs.push(new_dir.clone());
                    cwd.push(new_dir);
                }
            }
        } else if line.starts_with("$ ls") {
            //Is a command, get all of the lines until the next command (or end)
            let mut objects = vec!();
            let obj_iter = lines
                .by_ref();

            while let Some(obj) = obj_iter.next_if(|line| !line.starts_with("$")) {
                objects.push(obj);
            }
                //.take_while(|cmd_line| !cmd_line.starts_with("$"))
                //.collect::<Vec<_>>();
            for obj in objects {
                if obj.starts_with("dir") {
                    //is a directory
                    let new_dir_name = obj.split(" ").nth(1).unwrap();
                    // println!("Creating listed dir {}",new_dir_name);
                    if let Some(next_dir) = get_subdir_if_exists(&cwd.last().unwrap(), new_dir_name)
                    {
                        cwd.push(next_dir.clone());
                    } else {
                        cwd
                            .last_mut()
                            .unwrap()
                            .as_ref()
                            .borrow_mut()
                            .dirs
                            .push(Rc::new(RefCell::new(Dir::new(&cur_path,new_dir_name))));
                    }
                } else {
                    //file
                    let mut file_iter = obj.split(" ");
                    let size = file_iter.next().unwrap().parse::<usize>().unwrap();
                    let file_name = file_iter.next().unwrap();
                    cwd.last_mut()
                        .unwrap()
                        .as_ref()
                        .borrow_mut()
                        .files
                        .push(File::new(file_name, size));
                }
            }
        }
    }

    top_level
}

#[aoc(day7, part1)]
pub fn part1(input: &InputType) -> OutputType {
    let mut dir_map = BTreeMap::new();
    calculate_naive_dir_size(&input, &mut dir_map);

    //println!("{:?}", dir_map);

    //yea yea filter_map
    dir_map.iter().filter(|(_,size)| **size <= 100000).map(|(_,size)| size).sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &InputType) -> OutputType {
    let mut dir_map = BTreeMap::new();
    calculate_naive_dir_size(&input, &mut dir_map);

    static TOTAL_SIZE: usize = 70000000;
    static NEEDED_SPACE: usize = 30000000;
    let current_used = dir_map.get("//").unwrap();

    let unused_space = TOTAL_SIZE - current_used;
    let need_to_delete = NEEDED_SPACE - unused_space;


    *dir_map.iter().map(|(_,size)| size).filter(|&size| *size >= need_to_delete).min().unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "$ cd /
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
7214296 k"
    }

    #[test]
    fn day7_part1() {
        assert_eq!(part1(&day7_parse(get_test_input())), 95437);
    }

    #[test]
    fn day7_part2() {
        assert_eq!(part2(&day7_parse(get_test_input())), 24933642);
    }
}
