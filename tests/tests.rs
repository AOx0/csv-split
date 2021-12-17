#[cfg(test)]
mod file_manager {
    use std::env::current_dir;
    use spcsv::file_manager::*;

    #[test]
    fn count_lines_test() {
        let f : File = File::new(&format!("{}/tests/sample-1.csv", current_dir().unwrap().display()));
        assert_eq!(1001, f.lines())
    }
}