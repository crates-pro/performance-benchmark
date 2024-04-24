use std::path::PathBuf;

pub fn compare_stat(stats_a: &PathBuf, stats_b: &PathBuf, metric: &String, out_path: PathBuf) {
    unimplemented!()
}

#[cfg(test)]
mod test_compare_stat {
    /// test for compare_stat
    ///
    /// Step1. compare stats of metric `instructions` in `test/compare_stat/stat`.
    ///
    /// Step2. verify length of compare result.
    ///
    /// Step3. plot the compare result.
    ///
    /// Step4. clean up.
    #[test]
    fn test_compare_stat() {}
}
