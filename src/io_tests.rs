mod io_tests {

    use std::env;
    use std::fs::File;
    use std::io::{self, prelude::*, BufReader};
    use std::path::Path;

    #[test]
    pub fn test_reading_trees_from_text_file() {
        let path_file: &String = &String::from("./resources/trees.txt");
        let path_exists = Path::new(path_file).exists();
        assert_eq!(path_exists, true);
        let file = File::open(path_file);
        let file = match file {
            Ok(f) => f, 
            Err(e) => panic!("Something went wrong!")
        }; 
        let reader = BufReader::new(file);
        for line in reader.lines() {
            match line {
                Ok(l) => println!("{}", l), 
                Err(e) => panic!("Again something wrong!")
            }
        }
    }
}
