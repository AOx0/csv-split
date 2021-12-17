#[cfg(test)]
mod file_manager {
    use std::env::current_dir;
    use spcsv::file_manager::*;

    #[test]
    fn count_lines_test() {
        let f : File = File::new(&format!("{}/tests/sample-1.csv", current_dir().unwrap().display())).unwrap();
        assert_eq!(1001, f.lines())
    }

    #[test]
    fn bad_count_lines_test() {
        let f : File = File::new(&format!("{}/tests/sample-2.csv", current_dir().unwrap().display())).unwrap();
        assert_ne!(1001, f.lines())
    }


    #[test]
    fn get_file_name() {
        let f : File = File::new(&format!("{}/tests/sample-2.csv", current_dir().unwrap().display())).unwrap();
        assert_eq!("sample-2.csv", f.name())
    }

    #[test]
    fn bad_get_file_name() {
        let f : File = File::new(&format!("{}/tests/sample-3.csv", current_dir().unwrap().display())).unwrap();
        assert_ne!("sample-2.csv", f.name())
    }

    #[test]
    fn get_file_base_name() {
        let f : File = File::new(&format!("{}/tests/sample-2.csv", current_dir().unwrap().display())).unwrap();
        assert_eq!("sample-2", f.base_name().unwrap())
    }

    #[test]
    fn bad_get_file_base_name() {
        let f : File = File::new(&format!("{}/tests/sample-3.csv", current_dir().unwrap().display())).unwrap();
        assert_ne!("sample-2", f.base_name().unwrap())
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
    use std::env::current_dir;
    use spcsv::file_manager::*;

    #[test]
    fn get_names() {
        let f : File = File::new(&format!("{}/tests/sample-3.csv", current_dir().unwrap().display())).unwrap();
        assert_eq!(gen_names(f, 3), Vec::<String>::from(["sample-3-1".to_string(), "sample-3-2".to_string(), "sample-3-3".to_string()]))
    }
}