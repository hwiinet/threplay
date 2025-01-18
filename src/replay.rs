pub struct Replay {
    pub score: u64,
    pub character: String,
    pub name: String,
    pub rank: String,
    pub stage: u64,
}

#[allow(dead_code)]
pub fn read_replay(replay: &str, touhou_version: u32) -> Replay {
    if touhou_version == 11 {
        let lines: Vec<&str> = replay.lines().rev().take(8).collect();
        let extract_last_word = |line: &&str| line.split_whitespace().last().unwrap().to_string();

        let score = lines.iter().find(|&&line| line.starts_with("Score"))
                         .map(|line| line.split_whitespace().last().unwrap().parse::<u64>().unwrap()).unwrap();
        let character = lines.iter().find(|&&line| line.starts_with("Chara"))
                             .map(extract_last_word).unwrap();
        let name = lines.iter().find(|&&line| line.starts_with("Name"))
                        .map(extract_last_word).unwrap();
        let rank = lines.iter().find(|&&line| line.starts_with("Rank"))
                        .map(extract_last_word).unwrap();
        let stage = lines.iter().find(|&&line| line.starts_with("Stage"))
                         .map(|line| line.split_whitespace().last().unwrap().parse::<u64>().unwrap()).unwrap();

        Replay {
            score,
            character,
            name,
            rank,
            stage,
        }
    } else {
        panic!("Unsupported Touhou version");
    }
}
