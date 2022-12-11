use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric0, newline, not_line_ending},
    combinator::map,
    multi::separated_list0,
    IResult,
};
use std::collections::HashMap;

pub fn parse_input(input: &str) -> IResult<&str, HashMap<String, u64>> {
    let (_, lines) = separated_list0(newline, terminal_line)(input)?;

    let mut size_map: HashMap<String, u64> = HashMap::new();
    let mut current_path: Vec<&str> = vec![];

    for line in lines {
        match line {
            Terminal::File(file_size) => {
                current_path
                    .iter()
                    .fold(String::new(), |mut path, new_dir| {
                        path.push_str(new_dir);
                        if new_dir != &"/" {
                            path.push('/');
                        }
                        size_map
                            .entry(path.clone())
                            .and_modify(|s| *s += file_size)
                            .or_insert(file_size);

                        path
                    });
            }
            Terminal::Command(command) => match command {
                Command::ChangeDirectory(new_dir) => {
                    current_path.push(new_dir);
                }

                Command::PreviousDirectoryCommand => {
                    current_path.pop();
                }
                _ => {}
            },
            _ => {}
        }
    }
    Ok((input, size_map))
}

fn terminal_line(input: &str) -> IResult<&str, Terminal> {
    alt((command, file, directory))(input)
}

fn command(input: &str) -> IResult<&str, Terminal> {
    let (input, _) = tag("$ ")(input)?;
    let (input, command) = alt((
        previous_directory_command,
        change_directory_command,
        list_command,
    ))(input)?;
    Ok((input, Terminal::Command(command)))
}

fn file(input: &str) -> IResult<&str, Terminal> {
    let (input, size) = nom::character::complete::u64(input)?;
    let (input, _) = not_line_ending(input)?;
    Ok((input, Terminal::File(size)))
}

fn directory(input: &str) -> IResult<&str, Terminal> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = alphanumeric0(input)?;
    Ok((input, Terminal::Directory(name)))
}

fn list_command(input: &str) -> IResult<&str, Command> {
    map(tag("ls"), |_| Command::List)(input)
}

fn change_directory_command(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("cd ")(input)?;
    let (input, dir_string) = not_line_ending(input)?;

    Ok((input, Command::ChangeDirectory(dir_string)))
}

fn previous_directory_command(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("cd ..")(input)?;
    Ok((input, Command::PreviousDirectoryCommand))
}

#[derive(Debug, Clone, Copy)]
enum Command<'a> {
    ChangeDirectory(&'a str),
    List,
    PreviousDirectoryCommand,
}
#[derive(Debug, Clone, Copy)]
enum Terminal<'a> {
    File(u64),
    Command(Command<'a>),
    Directory(&'a str),
}
