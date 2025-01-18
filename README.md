# threplay

**threplay** is a Rust library which lets you interface with Touhou replay files. Below are the supported Touhou versions that you can use:

| Touhou Version | Supported |
| --- | --- |
| 11 | Yes |
| 15 | Soon |

It may be used like this:
```rust
let replay = threplay::read_replay(&data, 11);
println!("{}'s score was {} while playing with {}",
        replay.name, replay.score, replay.character);
// => Vic's score was 12345 while playing with ReimuC
```

The data that the `replay` struct holds currently is:

- `name` (the player's name)
- `score` (the player's score)
- `character` (reimu, marisa)*
- `rank` (easy, lunatic, etc.)
- `stage` (the stage that the player made it to)

*: the character value might be something like ReimuA or MarisaC. The letter at the end corresponds to which weapon you selected.

An important thing to note is that, if you're reading directly from a replay file, you're going to want to use a crate like `encoding_rs`, that way it won't raise any encoding errors. Touhou likes to use a weird encoding format for its replays, so that's why that happens.

This is the code I use to read the files, and it seems to work well:

```rust
let mut file = File::open("th11_10.rpy").unwrap();
let mut file_contents = Vec::new();
file.read_to_end(&mut file_contents).unwrap();

let (decoded, _, _) = encoding_rs::WINDOWS_1252.decode(&file_contents);
let decoded_str = decoded.to_string();
```

I hope you enjoy using `threplay`, and if you're interested, please do contribute by looking out for errors or bugs, or helping out with implementing new Touhou replay versions. Some Touhou games use the same formats, so we won't need to add a case for each version.
