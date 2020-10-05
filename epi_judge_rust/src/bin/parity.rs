fn parity(mut x: u64) -> u64 {
    let mut result = 0;
    while x != 0 {
        result ^= x & 1;
        x >>= 1;
    }
    result
}

fn main() {
    epi_judge_rust::run_tests("parity.tsv", |data| -> epi_judge_rust::Result<()> {
        let input = serde_json::from_str::<u64>(&data[0]).unwrap();
        let expected = serde_json::from_str::<u64>(&data[1]).unwrap();
        let actual = parity(input);

        epi_judge_rust::try_assert!(actual, expected)
    });
}
