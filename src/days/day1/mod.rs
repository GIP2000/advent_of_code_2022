pub fn get_n_maxes<const N: usize>(input: &str) -> u32 {
    // O(N + nN + N)
    let mut maxes = [0; N];
    for chunk in input.split("\n\n") {
        let current = chunk.lines().flat_map(str::parse::<u32>).sum::<u32>();
        for (i, &max) in maxes.iter().enumerate() {
            // if we found a new max replace it
            if current > max {
                maxes[i..].rotate_right(1);
                maxes[i] = current;
                break;
            }
        }
    }
    return maxes.into_iter().sum::<u32>();
}

pub fn get_slow_n_maxes(input: &str, n: usize) -> u32 {
    //O(nlogn + N) N = input n n = size of input
    let mut v = input
        .split("\n\n")
        .map(|chunk| chunk.lines().flat_map(str::parse::<u32>).sum::<u32>())
        .collect::<Vec<_>>();
    v.sort_by(|a, b| b.cmp(a));
    return v[0..n].into_iter().sum::<u32>();
}

#[cfg(test)]
mod test {
    use super::*;
    use std::time::Instant;
    const INPUT: &'static str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_day1() {
        assert_eq!(get_n_maxes::<1>(INPUT), 24000);
        assert_eq!(get_n_maxes::<3>(INPUT), 45000);
        assert_eq!(get_n_maxes::<1>(INPUT), get_slow_n_maxes(INPUT, 1));
        assert_eq!(get_n_maxes::<3>(INPUT), get_slow_n_maxes(INPUT, 3));
    }

    #[test]
    fn bench() {
        let before = Instant::now();
        get_n_maxes::<3>(INPUT);
        println!("fast: {:?}", before.elapsed());
        let before = Instant::now();
        get_slow_n_maxes(INPUT, 3);
        println!("slow: {:?}", before.elapsed());
    }
}
