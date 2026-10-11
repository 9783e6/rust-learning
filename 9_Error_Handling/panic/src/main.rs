fn main() {

    loop {
        let mut input_str = String::new();

        std::io::stdin()
            .read_line(&mut input_str)
            .expect("Failed to read line");

        match input_str.trim() {
            "1" => {
                panic!("crash and burn");
            },
            "2" => {
                let v = vec![1, 2, 3];
                v[99];
            },
            _ => break,
        }



    }

    
}
