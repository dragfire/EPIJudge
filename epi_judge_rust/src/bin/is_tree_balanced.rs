fn is_tree_balanced(s: String) -> bool {
    true
}

fn main() {
    epi_judge_rust::run_tests(
        "is_tree_balanced.tsv",
        |data| -> epi_judge_rust::Result<()> {
            let s = data[0].to_owned();
            let expected = serde_json::from_str::<bool>(&data[1]).unwrap();
            let actual = is_tree_balanced(s);

            epi_judge_rust::try_assert!(actual, expected)
        },
    );
}
