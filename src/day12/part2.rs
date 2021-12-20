use std::{io::{stdin, Read}, hash::Hash};

use indexmap::{IndexMap, IndexSet};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Cave {
    Start,
    End,
    Small(String),
    Big(String),
}

fn string_to_cave(str: &str) -> Cave {
    match str {
        "start" => Cave::Start,
        "end" => Cave::End,
        name => {
            if name.chars().all(|char| char.is_ascii_lowercase()) {
                Cave::Small(name.to_string())
            } else {
                Cave::Big(name.to_string())
            }
        }
    }
}

fn build_map(edges: Vec<(Cave, Cave)>) -> IndexMap<Cave, Vec<Cave>> {
    edges
        .into_iter()
        .fold(IndexMap::new(), |mut map, (end1, end2)| {
            map.entry(end1.clone()).or_insert_with(Vec::new).push(end2.clone());
            map.entry(end2).or_insert_with(Vec::new).push(end1);
            map
        })
}

fn process_input(input: &str) -> IndexMap<Cave, Vec<Cave>> {
    let edges = input
        .lines()
        .flat_map(|line| {
            line.split('-').map(string_to_cave).tuples()
        })
        .collect();

    build_map(edges)
}

fn traverse_map(map: &IndexMap<Cave, Vec<Cave>>, mut visited: IndexSet<Cave>, mut visited_twice: bool, curr_cave: Cave) -> Vec<Vec<Cave>> {
    match curr_cave {
        Cave::End => vec!(vec!(Cave::End)),
        _ => {
            if let Cave::Small(_) = curr_cave {
                visited_twice |= !visited.insert(curr_cave.clone());  // If this is the second time we are inserting a cave, note it
            }

            // Valid next caves to move to
            map[&curr_cave]
                .to_owned()
                .into_iter()
                .filter(|neighbour| *neighbour != Cave::Start && (!visited.contains(neighbour) || !visited_twice))
                .fold(Vec::new(), |mut acc, next_cave| {
                    let mut paths = traverse_map(map, visited.clone(), visited_twice, next_cave);
                    for path in paths.iter_mut() {
                        path.push(curr_cave.clone());
                    }

                    acc.extend(paths);
                    acc
                })
        },
    }
}

fn count_paths(map: IndexMap<Cave, Vec<Cave>>) -> usize {
    let paths = traverse_map(&map, IndexSet::new(), false, Cave::Start);
    paths.len()
}

fn main() {
    let mut buffer = String::new();
    stdin()
        .read_to_string(&mut buffer)
        .expect("Cannot read input");
    let map = process_input(&buffer);

    let num_paths = count_paths(map);

    println!("{}", num_paths);
}
