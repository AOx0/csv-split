use lazy_static::*;
use std::env::current_dir;
use spcsv::file_manager::File;

lazy_static! {
    static ref SAMPLE1: File = File::new(&format!("{}/tests/sample-1.csv", current_dir().unwrap().display())).unwrap();
    static ref SAMPLE2: File = File::new(&format!("{}/tests/sample-2.csv", current_dir().unwrap().display())).unwrap();
    static ref SAMPLE3: File = File::new(&format!("{}/tests/sample-3.csv", current_dir().unwrap().display())).unwrap();
}

#[cfg(test)]
mod file_manager {
    use std::env::current_dir;
    use spcsv::file_manager::*;
    use crate::*;

    #[test]
    fn count_lines_test() {
        assert_eq!(1001, SAMPLE1.lines())
    }

    #[test]
    fn bad_count_lines_test() {
        assert_ne!(1001, SAMPLE2.lines())
    }

    #[test]
    fn get_file_name() {
        assert_eq!("sample-2.csv", SAMPLE2.name())
    }

    #[test]
    fn bad_get_file_name() {
        assert_ne!("sample-2.csv", SAMPLE3.name())
    }

    #[test]
    fn get_file_base_name() {
        assert_eq!("sample-2", SAMPLE2.base_name().unwrap())
    }

    #[test]
    fn bad_get_file_base_name() {
        assert_ne!("sample-2", SAMPLE3.base_name().unwrap())
    }

    #[test]
    fn file_path() {
        let f : Option<File> = File::new(&format!("{}/tests/sample-3.csv", current_dir().unwrap().display()));
        assert_eq!(true, f.is_some())
    }

    #[test]
    fn bad_file_path() {
        let f : Option<File> = File::new(&format!("{}/tests/sample-3", current_dir().unwrap().display()));
        assert_eq!(true, f.is_none())
    }
}

#[cfg(test)]
mod lib {
    use spcsv::gen_names;
    use crate::*;

    #[test]
    fn get_names() {
        assert_eq!(gen_names(&SAMPLE3, 3), Vec::<String>::from(["sample-3_1.csv".to_string(), "sample-3_2.csv".to_string(), "sample-3_3.csv".to_string()]))
    }

    #[test]
    fn calc_files_to_create() {

    }
}