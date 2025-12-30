const MAX_NODES: usize = 26 * 26 * 26;

// 26^3 possible nodes, so store nodes as indices into a vector
type Input = Vec<Vec<usize>>;

pub fn parse(input: &str) -> Input {
    let mut graph = vec![vec![]; MAX_NODES];

    for line in input.lines() {
        let (from, to_many) = line.split_at(3);

        let from_idx = encode(from);

        for to in to_many[2..].split_ascii_whitespace() {
            let to_idx = encode(to);

            graph[from_idx].push(to_idx);
        }
    }

    graph
}

pub fn part1(input: &Input) -> u64 {
    num_paths(input, encode("you"), encode("out")).unwrap()
}

pub fn part2(input: &Input) -> u64 {
    let svr = encode("svr");
    let fft = encode("fft");
    let dac = encode("dac");
    let out = encode("out");

    num_paths(input, svr, fft).unwrap()
        * num_paths(input, fft, dac).unwrap()
        * num_paths(input, dac, out).unwrap()
}

fn encode(node: &str) -> usize {
    node.bytes().fold(0, |p, c| (c - b'a') as usize + 26 * p)
}

pub fn num_paths(input: &Input, src: usize, dst: usize) -> Option<u64> {
    let mut cache = vec![u64::MAX; MAX_NODES];
    dfs(input, src, dst, &mut cache)
}

pub fn dfs(
    input: &Input,
    src: usize,
    dst: usize,
    cache: &mut [u64],
) -> Option<u64> {
    if src == dst {
        Some(1)
    } else if cache[src] != u64::MAX {
        Some(cache[src])
    } else {
        let num = input[src].iter().try_fold(0, |sum, &child| {
            Some(dfs(input, child, dst, cache)? + sum)
        });
        cache[src] = num?;
        num
    }
}

// Answers for my input:
// Part 1: 494
// Part 2: 296006754704850
