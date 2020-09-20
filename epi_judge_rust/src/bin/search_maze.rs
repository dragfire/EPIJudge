use serde::Deserialize;
use serde_repr::*;
use std::cmp::{Eq, PartialEq};
use std::collections::VecDeque;

#[derive(Deserialize, Eq, PartialEq, Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
enum Color {
    White = 0,
    Black = 1,
}

fn search_maze(maze: Vec<Vec<Color>>, s: Coordinate, e: Coordinate) -> bool {
    let mut queue: VecDeque<Coordinate> = VecDeque::new();
    queue.push_back(s);
    let m = maze.len();
    let n = maze[0].len();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; n]; m];

    while !queue.is_empty() {
        let u = queue.pop_front().unwrap();
        visited[u.x as usize][u.y as usize] = true;

        if u == e {
            return true;
        }

        let dirs: Vec<[i32; 2]> = vec![[0, 1], [1, 0], [0, -1], [-1, 0]];

        for dir in dirs {
            let x = u.x + dir[0];
            let y = u.y + dir[1];

            if x >= 0
                && (x as usize) < m
                && y >= 0
                && (y as usize) < n
                && !visited[x as usize][y as usize]
                && maze[x as usize][y as usize] != Color::Black
            {
                queue.push_back(Coordinate { x, y });
            }
        }
    }

    false
}

fn main() {
    epi_judge_rust::run_tests("search_maze.tsv", |data| -> epi_judge_rust::Result<()> {
        let maze = serde_json::from_str::<Vec<Vec<Color>>>(&data[0]).unwrap();
        let s = serde_json::from_str::<Coordinate>(&data[1]).unwrap();
        let e = serde_json::from_str::<Coordinate>(&data[2]).unwrap();
        let expected = serde_json::from_str::<bool>(&data[3]).unwrap();
        let actual = search_maze(maze, s, e);

        epi_judge_rust::try_assert!(actual, expected)
    });
}
