macro_rules! open_font {
    ($path:expr) => ({
        use std::fs::File;
        use std::io::BufReader;
        use std::io::Read;

        let file = File::open($path)
            .expect("failed to open test font");
        let mut buf = BufReader::new(file);
        let mut contents = Vec::new();
        buf.read_to_end(&mut contents)
            .expect("failed to read test font");

        contents
    })
}

mod types;
mod font;
