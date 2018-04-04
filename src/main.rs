#[allow(unused_macros)]
macro_rules! parse_line {
    ( $t:ty ) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            iter.next().unwrap().parse::<$t>().unwrap()
        }
    );

    ( $( $t:ty), +) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            ( $(iter.next().unwrap().parse::<$t>().unwrap()),* )
        }
    );
}

#[allow(unused_macros)]
macro_rules! read_line {
    () => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            line.pop();
            line
        }
    )
}

#[allow(unused_macros)]
macro_rules! parse_vec {
    ( $t:ty ) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let iter = line.split_whitespace();
            iter.map(|v| v.parse::<$t>().unwrap()).collect::<Vec<_>>()
        }
    )
}

#[allow(dead_code)]
fn parse_table(rows: usize) -> Vec<Vec<char>> {
    (0..rows).map(|_| read_line!().chars().collect()).collect::<Vec<Vec<_>>>()
}

mod dijkstra {
    use std::cmp::Ordering;
    use std::collections::BinaryHeap;

    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct State<T: Copy+Clone+Eq+PartialEq+Ord> {
        pub cost: usize,
        pub pos: T
    }

    impl<T: Copy+Clone+Eq+PartialEq+Ord> Ord for State<T> {
        fn cmp(&self, other: &State<T>) -> Ordering {
            let c = other.cost.cmp(&self.cost);

            if c == Ordering::Equal {
                self.pos.cmp(&other.pos)
            } else {
                c
            }
        }
    }

    impl<T: Copy+Clone+Eq+PartialEq+Ord> PartialOrd for State<T> {
        fn partial_cmp(&self, other: &State<T>) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    pub trait Context<T: Copy+Clone+Eq+PartialEq+Ord> {
        fn dist(&self, pos: T) -> usize;
        fn nexts(&self, s: &State<T>) -> Vec<State<T>>;
        fn update_dist(&mut self, s: &State<T>);
    }

    #[allow(dead_code)]
    pub fn run<T: Copy+Clone+Eq+PartialEq+Ord>(context: &mut Context<T>, start: State<T>, end: T) -> Option<usize> {
        let mut heap = BinaryHeap::new();
        heap.push(start);

        while let Some(State {cost, pos}) = heap.pop() {
            if pos == end {
                return Some(cost);
            }

            if cost > context.dist(pos) { continue; }

            for next in context.nexts(&State {cost: cost, pos: pos}) {
                if next.cost < context.dist(next.pos) {
                    context.update_dist(&next);
                    heap.push(next);
                }
            }
        }

        None
    }
}

struct Context<'a> {
    ss: &'a Vec<Vec<char>>,
    dist: Vec<Vec<usize>>,
}

use dijkstra::State;

impl<'a> dijkstra::Context<(usize, usize)> for Context<'a> {
    fn dist(&self, pos: (usize, usize)) -> usize {
        self.dist[pos.0][pos.1]
    }

    fn nexts(&self, s: &State<(usize, usize)>) -> Vec<State<(usize, usize)>> {
        let mut res = Vec::new();

        for offset in vec![(-1,0), (1,0), (0,1), (0,-1)] {
            if is_reachable(s.pos, offset, self.ss) {
                let next_r = (s.pos.0 as i32 +offset.0) as usize;
                let next_c = (s.pos.1 as i32 +offset.1) as usize;

                let next = State{ cost: s.cost + 1, pos: (next_r, next_c)};
                res.push(next);
            }
        }

        res
    }

    fn update_dist(&mut self, s: &State<(usize, usize)>) {
        self.dist[s.pos.0][s.pos.1] = s.cost;
    }
}


fn is_reachable(pos: (usize, usize), offset: (i32, i32), ss: &Vec<Vec<char>>) -> bool {
    let h = ss.len();
    let w = ss[0].len();

    if (offset.0<0 && pos.0 < offset.0.abs() as usize) ||
        (offset.1<0 && pos.1 < offset.1.abs() as usize) ||
        (pos.0 as i32 + offset.0 >= h as i32) ||
        (pos.1 as i32 + offset.1 >= w as i32) {
        return false;
    }

    if ss[pos.0][pos.1] == '#' { return false; }

    true
}

fn _solve(ss: &Vec<Vec<char>>) -> Option<usize> {
    use dijkstra::State;
    let h = ss.len();
    let w = ss[0].len();
    let goal = (h-1, w-1);

    let mut dist = vec![vec![std::usize::MAX; w]; h];
    let mut heap = std::collections::BinaryHeap::new();
    heap.push(State{cost: 1, pos: (0,0)});

    while let Some(State {cost, pos}) = heap.pop() {
        if pos == goal {
            return Some(cost);
        }

        if cost > dist[pos.0][pos.1] { continue; }

        for offset in vec![(-1,0), (1,0), (0,1), (0,-1)] {
            if is_reachable(pos, offset, &ss) {
                let next_r = (pos.0 as i32 +offset.0) as usize;
                let next_c = (pos.1 as i32 +offset.1) as usize;

                let next = State{ cost: cost + 1, pos: (next_r, next_c)};

                if next.cost < dist[next_r][next_c] {
                    heap.push(next);
                    dist[next_r][next_c] = next.cost;
                }
            }
        }
    }

    None
}

fn solve(ss: Vec<Vec<char>>) -> Option<usize> {
    let white_num = ss.iter().map(|line| line.iter().fold(0, |sum, c| if *c == '.' { sum+1 } else {sum})).fold(0, |sum, n| sum + n);

    _solve(&ss).map(|n| white_num - n)
}

fn main() {
    
}

#[test]
fn solve_test() {
    assert_eq!(solve(vec!["..#".chars().collect(),
                           "#..".chars().collect(),
                           "...".chars().collect()]), Some(2));

    let test2 = 
r#".....................................
...#...####...####..###...###...###..
..#.#..#...#.##....#...#.#...#.#...#.
..#.#..#...#.#.....#...#.#...#.#...#.
.#...#.#..##.#.....#...#.#.###.#.###.
.#####.####..#.....#...#..##....##...
.#...#.#...#.#.....#...#.#...#.#...#.
.#...#.#...#.##....#...#.#...#.#...#.
.#...#.####...####..###...###...###..
.....................................
"#.to_string().split_whitespace().map(|s| s.chars().collect()).collect::<Vec<Vec<_>>>();

    assert_eq!(solve(test2), Some(209));

}
