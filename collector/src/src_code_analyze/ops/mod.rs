use crate::src_code_analyze::ops::count_slice_from_raw_parts::count_slice_from_raw_parts;

mod count_slice_from_raw_parts;

pub fn analyze_ops() -> Vec<Box<dyn Fn(&String) -> (String, f64)>> {
    vec![Box::new(count_slice_from_raw_parts)]
}
