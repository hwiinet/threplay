mod replay;

pub use replay::{Replay, read_replay};

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};
    use super::replay;

    #[test]

    fn read_replay() {
        let mut file = File::open("th11_10.rpy").unwrap();
        let mut file_contents = Vec::new();
        file.read_to_end(&mut file_contents).unwrap();
    
        let (decoded, _, _) = encoding_rs::WINDOWS_1252.decode(&file_contents);
        let decoded_str = decoded.to_string();
    
        let replay = replay::read_replay(&decoded_str, 11);
        println!("{}'s score was {} while playing with {}",
        replay.name, replay.score, replay.character);
    }
}
