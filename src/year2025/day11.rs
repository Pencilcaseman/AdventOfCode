const MAX_NODES: usize = 26 * 26 * 26;

// 26^3 possible nodes, so store nodes as indices into a vector
type Input = Vec<Vec<usize>>;

pub fn parse(input: &str) -> Input {
    let mut graph = vec![vec![]; MAX_NODES];

    for line in input.lines() {
        let mut iter = line.split_ascii_whitespace();
        let from = encode(&iter.next().unwrap()[..3]);
        graph[from].extend(iter.map(encode));
    }

    graph
}

pub fn part1(input: &Input) -> u64 {
    num_paths(input, "you", "out")
}

pub fn part2(input: &Input) -> u64 {
    let fft_to_dac = num_paths(input, "fft", "dac");

    if fft_to_dac != 0 {
        // fft comes before dac
        num_paths(input, "svr", "fft")
            * fft_to_dac
            * num_paths(input, "dac", "out")
    } else {
        // dac comes before fft
        num_paths(input, "svr", "dac")
            * num_paths(input, "dac", "fft")
            * num_paths(input, "fft", "out")
    }
}

fn encode(node: &str) -> usize {
    node.bytes().fold(0, |p, c| (c - b'a') as usize + 26 * p)
}

pub fn num_paths(input: &Input, src: &str, dst: &str) -> u64 {
    let mut cache = vec![u64::MAX; MAX_NODES];
    dfs(input, encode(src), encode(dst), &mut cache)
}

pub fn dfs(input: &Input, src: usize, dst: usize, cache: &mut [u64]) -> u64 {
    if src == dst {
        // Target found
        1
    } else if cache[src] != u64::MAX {
        // Cache hit
        cache[src]
    } else {
        // No cached value, so iterate over all children and recurse
        let num =
            input[src].iter().map(|&child| dfs(input, child, dst, cache)).sum();
        cache[src] = num;
        num
    }
}

// Answers for my input:
// Part 1: 494
// Part 2: 296006754704850
