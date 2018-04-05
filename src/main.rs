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
    use std::collections::HashMap;
    use std::hash::Hash;

    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct State<T>
    {
        pub cost: usize,
        pub pos: T
    }

    pub trait Context<T>
    {
        fn cost(&self, pos: T) -> usize;
        fn update_cost(&mut self, s: &State<T>);
        fn nexts(&self, s: &State<T>) -> Vec<State<T>>;
    }

    impl<T> Ord for State<T>
    where T: Copy+Ord {
        fn cmp(&self, other: &State<T>) -> Ordering {
            let c = other.cost.cmp(&self.cost);

            if c == Ordering::Equal {
                self.pos.cmp(&other.pos)
            } else {
                c
            }
        }
    }

    impl<T: Copy+Ord> PartialOrd for State<T> {
        fn partial_cmp(&self, other: &State<T>) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    #[allow(dead_code)]
    pub fn run<T>(context: &mut Context<T>, start: State<T>, end: T) -> Option<usize>
    where T: Copy+Ord {
        let mut heap = BinaryHeap::new();
        heap.push(start);

        while let Some(State {cost, pos}) = heap.pop() {
            if pos == end {
                return Some(cost);
            }

            if cost > context.cost(pos) { continue; }

            for next in context.nexts(&State {cost: cost, pos: pos}) {
                if next.cost < context.cost(next.pos) {
                    context.update_cost(&next);
                    heap.push(next);
                }
            }
        }

        None
    }

    pub struct DefaultContext<'a, T: 'a>
    {
        pub table: HashMap<T, usize>,
        pub f: &'a Fn(&State<T>) -> Vec<State<T>>
    }


    impl<'a, T> Context<T> for DefaultContext<'a, T>
    where T: Copy+Clone+Eq+Hash
    {
        fn cost(&self, pos: T) -> usize {
            self.table.get(&pos).map_or(::std::usize::MAX, |c| *c)
        }

        fn update_cost(&mut self, s: &State<T>) {
            self.table.insert(s.pos, s.cost);
        }

        fn nexts(&self, s: &State<T>) -> Vec<State<T>> {
            (self.f)(s)
        }
    }

    #[allow(dead_code)]
    pub fn create_default_context<'a, T, F>(f: &'a F) -> DefaultContext<'a, T>
    where T: Eq + Hash, F: Fn(&State<T>) -> Vec<State<T>> {
        DefaultContext{ table: HashMap::new(), f: f}
    }

}

use dijkstra::*;

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
    let next_state = |s: &State<(usize, usize)>| {
        let mut res = Vec::new();

        for offset in vec![(-1,0), (1,0), (0,1), (0,-1)] {
            if is_reachable(s.pos, offset, &ss) {
                let next_r = (s.pos.0 as i32 +offset.0) as usize;
                let next_c = (s.pos.1 as i32 +offset.1) as usize;

                let next = State{ cost: s.cost + 1, pos: (next_r, next_c)};
                res.push(next);
            }
        }

        res
    };

    let mut context = create_default_context(&next_state);

    dijkstra::run(&mut context, State{cost:1, pos:(0,0)}, (ss.len()-1, ss[0].len()-1))
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
