use std::fs::File;

fn main() {
    let filename = "hello.txt";
    let file_result = File::open(filename);

    let file = match file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"), 
    };
    
}
