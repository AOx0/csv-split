use lazy_static::*;
use spcsv::file_manager::File;
use std::env::current_dir;

lazy_static! {
    static ref SAMPLE1: File = File::new(
        &format!("{}/tests/sample-1.csv", current_dir().unwrap().display()),
        true
    )
    .unwrap();
    static ref SAMPLE2: File = File::new(
        &format!("{}/tests/sample-2.csv", current_dir().unwrap().display()),
        true
    )
    .unwrap();
    static ref SAMPLE3: File = File::new(
        &format!("{}/tests/sample-3.csv", current_dir().unwrap().display()),
        true
    )
    .unwrap();
}

#[cfg(test)]
mod file_manager {
    use crate::*;
    use std::env::current_dir;

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
        let f: Option<File> = File::new(
            &format!("{}/tests/sample-3.csv", current_dir().unwrap().display()),
            true,
        );
        assert_eq!(true, f.is_some())
    }

    #[test]
    fn bad_file_path() {
        let f: Option<File> = File::new(
            &format!("{}/tests/sample-3", current_dir().unwrap().display()),
            true,
        );
        assert_eq!(true, f.is_none())
    }
}

#[cfg(test)]
mod lib {
    use crate::*;
    use spcsv::misc::{gen_names, lines_per_file};

    #[test]
    fn get_names() {
        assert_eq!(
            gen_names(&SAMPLE3, 3),
            Vec::<String>::from([
                "sample-3_1.csv".to_string(),
                "sample-3_2.csv".to_string(),
                "sample-3_3.csv".to_string()
            ])
        )
    }

    #[test]
    fn valid_lines_per_file() {
        assert!(lines_per_file(&SAMPLE1, 50).is_some())
    }

    #[test]
    fn invalid_lines_per_file() {
        assert!(lines_per_file(&SAMPLE2, 150).is_none())
    }

    #[test]
    fn n_lines_per_file_sample1() {
        let (each, remain) = lines_per_file(&SAMPLE1, 13).unwrap();
        assert_eq!(each, 76);
        assert_eq!(remain, 12);
    }

    #[test]
    fn n_lines_per_file_sample2() {
        let (each, remain) = lines_per_file(&SAMPLE2, 19).unwrap();
        assert_eq!(each, 5);
        assert_eq!(remain, 5);
    }
}
