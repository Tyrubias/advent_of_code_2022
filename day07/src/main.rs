#![forbid(unsafe_code)]
#![forbid(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::{env::args_os, fs::read_to_string, path::MAIN_SEPARATOR};

use camino::Utf8PathBuf;
use color_eyre::{eyre::eyre, install, Result};
use id_tree::{InsertBehavior, Node, Tree};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::u64,
    combinator::{all_consuming, map},
    sequence::{preceded, separated_pair},
    Finish, IResult,
};

fn main() -> Result<()> {
    install()?;

    let file_path = args_os()
        .nth(1)
        .ok_or_else(|| eyre!("should have more args"))?;
    let contents = read_to_string(file_path)?;

    let lines = contents
        .lines()
        .flat_map(|line| all_consuming(parse_line)(line).finish())
        .map(|(_, line)| line)
        .collect_vec();

    let mut fs_tree = Tree::new();
    let mut current = fs_tree.insert(
        Node::new(FsEntity {
            name: "/".into(),
            size: 0,
        }),
        InsertBehavior::AsRoot,
    )?;

    for line in lines {
        match line {
            Line::Command(command) => match command {
                Command::Ls => {}
                Command::Cd(path) => match path.as_str() {
                    "/" => {
                        current = fs_tree
                            .root_node_id()
                            .ok_or_else(|| eyre!("should have root"))?
                            .clone();
                    }
                    ".." => {
                        current = fs_tree
                            .get(&current)?
                            .parent()
                            .ok_or_else(|| eyre!("should have parent"))?
                            .clone()
                    }
                    _ => {
                        let dir = FsEntity {
                            name: path,
                            size: 0,
                        };
                        current =
                            fs_tree.insert(Node::new(dir), InsertBehavior::UnderNode(&current))?;
                    }
                },
            },
            Line::Entry(entry) => match entry {
                Entry::Dir(_) => {}
                Entry::File(size, name) => {
                    let file = FsEntity { name, size };
                    fs_tree.insert(Node::new(file), InsertBehavior::UnderNode(&current))?;
                }
            },
        }
    }

    let mut s = String::new();
    fs_tree.write_formatted(&mut s)?;
    println!("{s}");

    let part_1 = fs_tree
        .traverse_post_order(
            fs_tree
                .root_node_id()
                .ok_or_else(|| eyre!("should have root"))?,
        )?
        .filter(|node| !node.children().is_empty())
        .flat_map(|node| total_size_at_node(&fs_tree, node))
        .filter(|&size| size <= 100_000)
        .sum::<usize>();

    println!("Part 1: {part_1}");

    let total_space = 70_000_000_usize;
    let used_space = total_size_at_node(
        &fs_tree,
        fs_tree.get(
            fs_tree
                .root_node_id()
                .ok_or_else(|| eyre!("should have root"))?,
        )?,
    )?;
    let free_space = total_space
        .checked_sub(used_space)
        .ok_or_else(|| eyre!("shouldn't underflow"))?;
    let needed_free_space = 30_000_000_usize;
    let size_needed = needed_free_space
        .checked_sub(free_space)
        .ok_or_else(|| eyre!("shouldn't underflow"))?;

    let part2 = fs_tree
        .traverse_post_order(
            fs_tree
                .root_node_id()
                .ok_or_else(|| eyre!("should have root"))?,
        )?
        .filter(|node| !node.children().is_empty())
        .flat_map(|node| total_size_at_node(&fs_tree, node))
        .filter(|&size| size >= size_needed)
        .min()
        .ok_or_else(|| eyre!("should have minimum"))?;

    println!("Part 2: {part2}");

    Ok(())
}

fn total_size_at_node(tree: &Tree<FsEntity>, node: &Node<FsEntity>) -> Result<usize> {
    let mut total = node.data().size;
    for child in node.children() {
        total += total_size_at_node(tree, tree.get(child)?)?;
    }
    Ok(total)
}

#[allow(dead_code)]
#[derive(Debug)]
struct FsEntity {
    name: Utf8PathBuf,
    size: usize,
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Entry(Entry),
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    alt((
        map(parse_entry, Line::Entry),
        map(parse_command, Line::Command),
    ))(input)
}

#[derive(Debug)]
enum Entry {
    Dir(Utf8PathBuf),
    File(usize, Utf8PathBuf),
}

fn parse_entry(input: &str) -> IResult<&str, Entry> {
    alt((parse_dir, parse_file))(input)
}

fn parse_file(input: &str) -> IResult<&str, Entry> {
    map(separated_pair(u64, tag(" "), parse_path), |(size, name)| {
        Entry::File(size as usize, name)
    })(input)
}

fn parse_dir(input: &str) -> IResult<&str, Entry> {
    preceded(tag("dir "), map(parse_path, Entry::Dir))(input)
}

#[derive(Debug)]
enum Command {
    Ls,
    Cd(Utf8PathBuf),
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    preceded(tag("$ "), alt((parse_ls, parse_cd)))(input)
}

fn parse_cd(input: &str) -> IResult<&str, Command> {
    preceded(tag("cd "), map(parse_path, Command::Cd))(input)
}

fn parse_path(input: &str) -> IResult<&str, Utf8PathBuf> {
    map(
        take_while1(|c: char| c.is_alphanumeric() || c == '.' || c == MAIN_SEPARATOR),
        Into::into,
    )(input)
}

fn parse_ls(input: &str) -> IResult<&str, Command> {
    map(tag("ls"), |_| Command::Ls)(input)
}
