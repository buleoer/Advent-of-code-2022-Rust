
use std::{fs, path::Path};
use id_tree::*;
use id_tree::InsertBehavior::*;

const TOTAL_SIZE: u64 = 70_000_000;
const NEEDED_SIZE: u64 = 30_000_000;


#[derive(Debug, PartialEq)]
pub enum CdSpec<'a> {
    Folder { name: &'a str },
    Root,
    Parent,
}

#[derive(Debug, PartialEq)]
pub enum LsOutputRecord<'a> {
    Dir { name: &'a str },
    File { name: &'a str, size: u32 }
}

#[derive(Debug, PartialEq)]
pub enum Command<'a> {
    Cd { spec: CdSpec<'a> },
    Ls { result: Vec<LsOutputRecord<'a>> },
}

const SIZE_LIMIT: u32 = 100000;


pub fn run_part_1<P: AsRef<Path>>(filename: P) -> u32 {
    let input = fs::read_to_string(filename).unwrap();
    let (_, input) = parser::parse_input(&input).unwrap();
    calculate_part_1(input)
}

fn calculate_part_1(cmds: Vec<Command>) -> u32 {
    let mut stack_current_folders: Vec<(&str, u32)> = vec!(("/", 0));
    let mut total_size = 0u32;
    for cmd in cmds.iter().skip(1) {
        match cmd {
            Command::Cd{ spec: CdSpec::Folder{ name: nm } } => {
                stack_current_folders.push((nm, 0));
            }
            Command::Cd{ spec: CdSpec::Root } => {
                panic!("Should not happen!");
            }
            Command::Cd{ spec: CdSpec::Parent } => {
                let this_folder = stack_current_folders.pop().unwrap();
                if this_folder.1 <= SIZE_LIMIT {
                    total_size += this_folder.1;
                }
            }
            Command::Ls{ result: ls_output } => {
                for ls_record in ls_output {
                    match ls_record {
                        LsOutputRecord::Dir {name: _name} => {
                            // Nothing to do
                        }
                        LsOutputRecord::File {name: _name, size} => {
                            for i in 0..stack_current_folders.len() {
                                stack_current_folders[i].1 += size;
                            }
                        }
                    }
                }
            }
        }
    }

    for fld in stack_current_folders {
        if fld.1 <= SIZE_LIMIT {
            total_size += fld.1;
        }
    }

    total_size
}

pub fn run_part_2<P: AsRef<Path>>(filename: P) -> u64 {
    let input = fs::read_to_string(filename).unwrap();
    let (_, input) = parser::parse_input(&input).unwrap();
    let tree = build_folder_tree(&input);
    let used_size = tree.get(tree.root_node_id().unwrap())
            .unwrap()
            .data().size;
    let free_space = TOTAL_SIZE - used_size;
    let space_to_free = NEEDED_SIZE - free_space;
    let folder = find_folder_to_delete(&tree, space_to_free);
    folder.unwrap().size
}

#[derive(Debug)]
struct Folder {
    _name: String,
    size: u64,
}

fn build_folder_tree(commands: &Vec<Command>) -> Tree<Folder> {
    let mut tree: Tree<Folder> = Tree::new();
    let root_id = tree.insert(
        Node::new(Folder { _name: "/".to_string(), size: 0, }),
        AsRoot
    ).unwrap();
    let mut current_node_id = root_id.clone();

    for cmd in commands.iter().skip(1) {
        match cmd {
            Command::Cd{ spec: CdSpec::Folder{ name: nm } } => {
                current_node_id = tree.insert(
                    Node::new(Folder { _name: nm.to_string(), size: 0, }),
                    UnderNode(&current_node_id))
                    .unwrap();
            }
            Command::Cd{ spec: CdSpec::Root } => {
                panic!("Should never happen!");
            }
            Command::Cd{ spec: CdSpec::Parent } => {
                current_node_id = tree.get(&current_node_id).unwrap().parent().unwrap().clone();
            }
            Command::Ls{ result: ls_output } => {
                let current_folder_mut = tree.get_mut(&current_node_id).unwrap().data_mut();
                for ls_record in ls_output {
                    match ls_record {
                        LsOutputRecord::Dir {name: _name} => {
                            // Nothing to do
                        }
                        LsOutputRecord::File {name: _name, size} => {
                            current_folder_mut.size += *size as u64;    // ??? why do we need to dereference *size ??? this is not a reference!
                        }
                    }
                }
            }
        }
    }

    // Update parent folders with the size of their children
    update_folder_size(&mut tree, &root_id);

    tree
}

#[allow(dead_code)]
fn print_tree(tree: &Tree<Folder>) {
    let mut s = String::new();
    tree.write_formatted(&mut s).unwrap();
    println!("{s}");
}


// Update the size of all folders by adding the size of all their children
fn update_folder_size(tree: &mut Tree<Folder>, root_id: &NodeId) {
    for node_id in tree.traverse_post_order_ids(root_id).unwrap() {
        let mut size_children = 0u64;
        let node = tree.get(&node_id).unwrap();
        for child in node.children() {
            let child_size = tree.get(child).unwrap().data().size;
            size_children += child_size;
        }

        let node = tree.get_mut(&node_id).unwrap();
        let data = node.data_mut();
        data.size += size_children;
    }
}

fn find_folder_to_delete(tree: &Tree<Folder>, space_to_free: u64) -> Option<&Folder> {
    let root_id = (*tree).root_node_id().unwrap();
    let mut current_size_found = TOTAL_SIZE;
    let mut folder_found: Option<&Folder> = None;
    for node in tree.traverse_post_order(&root_id).unwrap() {
        if (node.data().size >= space_to_free) && (node.data().size < current_size_found) {
            current_size_found = node.data().size;
            folder_found = Some(&node.data());
        }
    }
    folder_found
}


mod parser {

    use super::{CdSpec, LsOutputRecord, Command};

    use nom::IResult;
    use nom::bytes::complete::{tag, take_while1};
    use nom::character::complete::{char, u32, line_ending};
    use nom::sequence::{preceded, terminated, delimited, separated_pair};
    use nom::multi::many1;
    use nom::branch::alt;
    use nom::combinator::map;

    fn is_alphabetic_lowercase(c: char) -> bool {
        c >= 'a' && c <= 'z'
    }

    fn is_alphabetic_lowercase_or_dot(c: char) -> bool {
        (c >= 'a' && c <= 'z') || c == '.'
    }

    fn parse_folder_name(input: &str) -> IResult<&str, &str> {
        take_while1(is_alphabetic_lowercase)(input)
    }

    fn parse_file_name(input: &str) -> IResult<&str, &str> {
        // TODO: Improve this
        take_while1(is_alphabetic_lowercase_or_dot)(input)
    }

    fn parse_cd_command(input: &str) -> IResult<&str, Command> {
        delimited(
            tag("$ cd "),
            map(
                alt((
                    map(tag("/"), |_| CdSpec::Root),
                    map(tag(".."), |_| CdSpec::Parent),
                    map(parse_folder_name, |f| CdSpec::Folder{ name: f} ),
                )),
                |cd_spec| Command::Cd{ spec: cd_spec }
            ),
            line_ending
        )(input)
    }

    fn parse_ls_output_line_dir(input: &str) -> IResult<&str, LsOutputRecord> {
        map(
            preceded(
                tag("dir "),
                parse_folder_name
            ),
            |s| LsOutputRecord::Dir{name: s}
        )(input)
    }

    fn parse_ls_output_line_file(input: &str) -> IResult<&str, LsOutputRecord> {
        map(
            separated_pair(
                u32,
                char(' '),
                parse_file_name,
            ),
            |(size, name)| LsOutputRecord::File{ name: name, size: size}
        )(input)
    }

    fn parse_ls_command(input: &str) -> IResult<&str, Command> {
        preceded(
            terminated(
                tag("$ ls"),
                line_ending
            ),
            map(
                many1(
                    terminated(
                        alt((
                            parse_ls_output_line_dir,
                            parse_ls_output_line_file
                        )),
                        line_ending
                    )
                ),
                |v_ls_output| Command::Ls{ result: v_ls_output }
            )
        )(input)
    }

    pub fn parse_input(input: &str) -> IResult<&str, Vec<Command>> {
        many1(
            alt((
                parse_cd_command,
                parse_ls_command
            ))
        )(input)
    }

    #[cfg(test)]
    mod test {

        use super::{CdSpec, Command, LsOutputRecord};

        #[test]
        fn test_parse_cd() {
            assert_eq!(super::parse_cd_command("$ cd ..\n"), Ok(("", Command::Cd { spec: CdSpec::Parent } )));
            assert_eq!(super::parse_cd_command("$ cd /\n"), Ok(("", Command::Cd { spec: CdSpec::Root } )));
            assert_eq!(super::parse_cd_command("$ cd folder\n"), Ok(("", Command::Cd { spec: CdSpec::Folder { name: "folder" } } )));
        }

        #[test]
        fn test_parse_ls() {
            assert_eq!(
                super::parse_ls_command("$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n"),
                Ok(("",
                    Command::Ls {
                        result: vec!(
                            LsOutputRecord::Dir { name: "a" },
                            LsOutputRecord::File { name: "b.txt", size: 14848514 },
                            LsOutputRecord::File { name: "c.dat", size: 8504156 },
                            LsOutputRecord::Dir { name: "d" },
                        )
                    }
                ))
            );
        }

    }
}



#[cfg(test)]
mod test {

    #[test]
    fn test_part1() {
        assert_eq!(super::run_part_1("test_input/day07.txt"), 95437);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::run_part_2("test_input/day07.txt"), 24933642);
    }

}

