pub fn count_slice_from_raw_parts(input: &String) -> (String, f64) {
    (
        "slice_from_raw_parts".to_string(),
        input.matches("slice::from_raw_parts").count() as f64,
    )
}
