# nbs-rs

<img width="256" height="256" alt="nbs-rs" src="https://github.com/user-attachments/assets/f46c482e-a6ae-4b00-9399-b84e5813d13f" />

**nbs-rs** is a lightweight, zero-dependency Rust library for parsing and serializing **OpenNBS** (`.nbs`) files, the format used by [Open Note Block Studio](https://opennbs.org/).

This library focuses exclusively on data manipulation and format compliance. It allows you to convert raw byte arrays into structured Rust types and back again, giving you full control over the file implementation logic.

## Features

  * **Complete NBS Support:** Fully supports NBS Version 5, including Headers, Notes, Layers, and Custom Instruments.
  * **I/O Agnostic:** The library does not perform file system operations. It operates strictly on in-memory buffers (`&[u8]` and `Vec<u8>`), allowing it to be used in generic contexts (files, network streams, embedded environments).
  * **Delta-Encoding Abstraction:** Automatically handles the complexity of NBS "tick jumps" and "layer jumps," exposing a flat, easy-to-use vector of `Note` structs.

## Installation

Add the library to your `Cargo.toml`:

```toml
[dependencies]
nbs-rs = { git = "https://github.com/tomast1337/nbs-rs" }
```

## Usage

### Philosophy: Byte-in, Byte-out

**nbs-rs** does not assume how you store or retrieve your files.

1.  **To Parse:** You provide a byte slice `&[u8]`.
2.  **To Write:** The library returns a `Vec<u8>`.

### Parsing an .nbs file

To parse a file, read the bytes using standard filesystem tools, then pass them to the parser.

```rust
use std::fs;
use nbs_rs::NbsParser;

fn main() -> std::io::Result<()> {
    // 1. User handles I/O
    let file_bytes = fs::read("./assets/song.nbs")?;

    // 2. Library handles parsing
    let mut parser = NbsParser::new(&file_bytes);
    let nbs_file = parser.parse()?;

    // Access metadata
    let song_name = String::from_utf8_lossy(&nbs_file.header.song_name);
    println!("Parsed Song: {}", song_name);
    println!("Total Notes: {}", nbs_file.notes.len());

    Ok(())
}
```

### Modifying and Saving

You can modify the `NbsFile` struct directly and then serialize it back to bytes.

```rust
use std::fs;
use nbs_rs::NbsWriter;

fn main() -> std::io::Result<()> {
    // ... assume `nbs_file` is an existing NbsFile struct ...

    // Modify the song (e.g., change the author)
    nbs_file.header.song_author = b"New Author".to_vec();

    // 1. Library handles serialization
    let mut writer = NbsWriter::new();
    let out_bytes = writer.get_file_bytes(&nbs_file);

    // 2. User handles I/O
    fs::write("./assets/modified_song.nbs", out_bytes)?;

    Ok(())
}
```

## Data Structures

The library exposes four main structs that map directly to the NBS specification:

1.  **`NbsFile`**: The root container holding the Header, Notes, Layers, and Instruments.
2.  **`Header`**: Metadata including song length, tempo, author, and save settings.
3.  **`Note`**: Represents a specific sound event.
      * *Note:* The library calculates absolute ticks internally. You do not need to manually calculate jump values.
4.  **`Layer`**: Information regarding UI layers (volume, lock status, name).
5.  **`Instrument`**: Custom instrument definitions embedded in the file.

## Contributing

Contributions are welcome. Please ensure that any changes to the parser or writer logic maintain compatibility with the official [OpenNBS specification](https://opennbs.org/nbs).
