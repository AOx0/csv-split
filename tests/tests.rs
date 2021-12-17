use spcsv::args::Args;

#[cfg(test)]
mod file_manager {
    use std::env::current_dir;
    use spcsv::file_manager::*;

    #[test]
    fn count_lines_test() {
        let f : File = File::new(&format!("{}/tests/sample-1.csv", current_dir().unwrap().display()));
        assert_eq!(1001, f.lines())
    }

    #[test]
    fn bad_count_lines_test() {
        let f : File = File::new(&format!("{}/tests/sample-2.csv", current_dir().unwrap().display()));
        assert_ne!(1001, f.lines())
    }


    #[test]
    fn get_file_name() {
        let f : File = File::new(&format!("{}/tests/sample-2.csv", current_dir().unwrap().display()));
        assert_eq!("sample-2.csv", f.name())
    }

    #[test]
    fn bad_get_file_name() {
        let f : File = File::new(&format!("{}/tests/sample-3.csv", current_dir().unwrap().display()));
        assert_ne!("sample-2.csv", f.name())
    }
}