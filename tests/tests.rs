#[cfg(test)]
mod file_manager {
    use std::env::current_dir;
    use spcsv::file_manager::*;

    #[test]
    fn count_lines_test_1() {
        let f : File = File::new(&format!("{}/tests/sample-1.csv", current_dir().unwrap().display()));
        assert_eq!(1001, f.lines())
    }

    #[test]
    fn count_lines_test_2() {
        let f : File = File::new(&format!("{}/tests/sample-2.csv", current_dir().unwrap().display()));
        assert_eq!(101, f.lines())
    }

    #[test]
    fn count_lines_test_3() {
        let f : File = File::new(&format!("{}/tests/sample-3.csv", current_dir().unwrap().display()));
        assert_eq!(10001, f.lines())
    }
}