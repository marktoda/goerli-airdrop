use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn load_creator_map(file_name: &str) -> HashMap<String, usize> {
    let path = Path::new(file_name);
    if path.exists() {
        // Open the file in read-only mode with buffer.
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        // Read the JSON contents of the file as an instance of `User`.
        serde_json::from_reader(reader).unwrap()
    } else {
        HashMap::new()
    }
}
