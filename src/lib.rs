use std::io::{self};
use std::vec::Vec;

const CURRENT_NBS_VERSION: u8 = 5;

#[derive(Debug, Clone)]
pub struct Instrument {
    pub name: Vec<u8>,
    pub file: Vec<u8>,
    pub key: u8,
    pub press_key: u8,
}

impl Instrument {
    pub fn new(key: u8, name: Vec<u8>, file: Vec<u8>, press_key: u8) -> Instrument {
        Instrument {
            name,
            file,
            key,
            press_key,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Note {
    pub tick: u16,
    pub layer: u16,
    pub instrument: u8,
    pub key: u8,
    pub velocity: u8,
    pub panning: i8,
    pub pitch: i16,
}

impl Note {
    pub fn new(tick: u16, layer: u16, instrument: u8, key: u8) -> Note {
        Note {
            tick,
            layer,
            instrument,
            key,
            velocity: 100, // default
            panning: 0,    // default
            pitch: 0,      // default
        }
    }
}

#[derive(Debug, Clone)]
pub struct Layer {
    pub id: u16,
    pub name: Vec<u8>,
    pub lock: bool,
    pub volume: u8,
    pub panning: i8,
}

impl Layer {
    pub fn new(id: u16) -> Layer {
        Layer {
            id,
            name: Vec::new(),
            lock: false, // default
            volume: 100, // default
            panning: 0,  // default
        }
    }
}

#[derive(Debug, Clone)]
pub struct Header {
    pub version: u8,
    pub default_instruments: u8,
    pub song_length: u16,
    pub song_layers: u16,
    pub song_name: Vec<u8>,
    pub song_author: Vec<u8>,
    pub original_author: Vec<u8>,
    pub description: Vec<u8>,
    pub tempo: u16,
    pub auto_save: bool,
    pub auto_save_duration: u8,
    pub time_signature: u8,
    pub minutes_spent: u32,
    pub left_clicks: u32,
    pub right_clicks: u32,
    pub blocks_added: u32,
    pub blocks_removed: u32,
    pub song_origin: Vec<u8>,
    pub loop_flag: bool,
    pub max_loop_count: u8,
    pub loop_start: u16,
}

impl Header {
    pub fn new() -> Header {
        Header {
            version: CURRENT_NBS_VERSION,
            default_instruments: 16,
            song_length: 0,
            song_layers: 0,
            song_name: Vec::new(),
            song_author: Vec::new(),
            original_author: Vec::new(),
            description: Vec::new(),
            tempo: 0,
            auto_save: false,
            auto_save_duration: 10,
            time_signature: 4,
            minutes_spent: 0,
            left_clicks: 0,
            right_clicks: 0,
            blocks_added: 0,
            blocks_removed: 0,
            song_origin: Vec::new(),
            loop_flag: false,
            max_loop_count: 0,
            loop_start: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NbsFile {
    pub header: Header,
    pub notes: Vec<Note>,
    pub layers: Vec<Layer>,
    pub instruments: Vec<Instrument>,
}

impl NbsFile {
    pub fn new(
        header: Header,
        notes: Vec<Note>,
        layers: Vec<Layer>,
        instruments: Vec<Instrument>,
    ) -> NbsFile {
        NbsFile {
            header,
            notes,
            layers,
            instruments,
        }
    }

    pub fn update_header(&mut self, version: u8) {
        self.header.version = version;
        if !self.notes.is_empty() {
            self.header.song_length = self.notes.last().unwrap().tick;
        }
        self.header.song_layers = self.layers.len() as u16;
    }

    pub fn save(&self, _filename: &str, _version: u8) -> io::Result<()> {
        // TODO: Implement file writing logic
        Ok(())
    }
}

pub struct NbsParser<'a> {
    current_data: &'a [u8],
}

impl<'a> NbsParser<'a> {
    pub fn new(file_data: &'a [u8]) -> NbsParser<'a> {
        NbsParser {
            current_data: file_data,
        }
    }

    pub fn parse(&mut self) -> io::Result<NbsFile> {
        let header = self.parse_header()?;
        let notes = self.parse_notes()?;
        let layers = self.parse_layers()?;
        let instruments = self.parse_instruments()?;
        Ok(NbsFile::new(header, notes, layers, instruments))
    }

    fn read_u16(&mut self) -> u16 {
        let val = u16::from_le_bytes([self.current_data[0], self.current_data[1]]);
        self.current_data = &self.current_data[2..];
        val
    }

    fn read_u8(&mut self) -> u8 {
        let val = self.current_data[0];
        self.current_data = &self.current_data[1..];
        val
    }

    fn read_i16(&mut self) -> i16 {
        let val = i16::from_le_bytes([self.current_data[0], self.current_data[1]]);
        self.current_data = &self.current_data[2..];
        val
    }

    fn read_u32(&mut self) -> u32 {
        let val = u32::from_le_bytes([
            self.current_data[0],
            self.current_data[1],
            self.current_data[2],
            self.current_data[3],
        ]);
        self.current_data = &self.current_data[4..];
        val
    }

    fn read_string(&mut self) -> io::Result<Vec<u8>> {
        let len = self.read_u32() as usize;
        let str = self.current_data[..len].to_vec();
        self.current_data = &self.current_data[len..];
        Ok(str)
    }

    fn parse_header(&mut self) -> io::Result<Header> {
        // remove the first 2 bytes
        self.current_data = &self.current_data[2..];
        let version = CURRENT_NBS_VERSION;
        self.current_data = &self.current_data[1..];

        let default_instruments = self.read_u8();
        let song_length = self.read_u16();
        let song_layers = self.read_u16();
        let song_name = self.read_string()?;
        let song_author = self.read_string()?;
        let original_author = self.read_string()?;
        let description = self.read_string()?;
        let tempo = self.read_u16();
        let auto_save = self.read_u8() != 0;
        let auto_save_duration = self.read_u8();
        let time_signature = self.read_u8();
        let minutes_spent = self.read_u32();
        let left_clicks = self.read_u32();
        let right_clicks = self.read_u32();
        let blocks_added = self.read_u32();
        let blocks_removed = self.read_u32();
        let song_origin = self.read_string()?;
        let loop_flag = self.read_u8() != 0;
        let max_loop_count = self.read_u8();
        let loop_start = self.read_u16();

        Ok(Header {
            version,
            default_instruments,
            song_length,
            song_layers,
            song_name,
            song_author,
            original_author,
            description,
            tempo,
            auto_save,
            auto_save_duration,
            time_signature,
            minutes_spent,
            left_clicks,
            right_clicks,
            blocks_added,
            blocks_removed,
            song_origin,
            loop_flag,
            max_loop_count,
            loop_start,
        })
    }

    fn parse_notes(&mut self) -> io::Result<Vec<Note>> {
        let mut notes = Vec::new();
        let mut current_tick: u16 = 0;

        while self.current_data.len() > 0 {
            let jump_ticks = self.read_u16();
            if jump_ticks == 0 {
                break;
            }

            current_tick += jump_ticks;

            while self.current_data.len() > 0 {
                let jump_layers = self.read_u16();
                if jump_layers == 0 {
                    break;
                }

                let instrument = self.read_u8();
                let key = self.read_u8();
                let velocity = self.read_u8();
                let panning = self.read_u8() as i8 - 100;
                let pitch = self.read_i16();

                notes.push(Note {
                    tick: current_tick,
                    layer: jump_layers,
                    instrument,
                    key,
                    velocity,
                    panning,
                    pitch,
                });
            }
        }

        Ok(notes)
    }

    fn parse_layers(&mut self) -> io::Result<Vec<Layer>> {
        let mut layers = Vec::new();
        let mut layer_id: u16 = 0;

        while self.current_data.len() > 0 {
            let name = self.read_string()?;
            if name.is_empty() {
                break;
            }

            let lock = self.read_u8() != 0;
            let volume = self.read_u8();
            let panning = self.read_u8() as i8 - 100;

            layers.push(Layer {
                id: layer_id,
                name,
                lock,
                volume,
                panning,
            });

            layer_id += 1;
        }

        Ok(layers)
    }

    fn parse_instruments(&mut self) -> io::Result<Vec<Instrument>> {
        let mut instruments = Vec::new();
        // next u8 is the number of instruments
        let num_instruments = self.read_u8();

        for _ in 0..num_instruments {
            let name = self.read_string()?;
            let sound_file = self.read_string()?;
            let sound_key = self.read_u8();
            let press_key = self.read_u8();

            let mut instrument = Instrument::new(sound_key, name, sound_file, press_key);
            instrument.press_key = press_key;
            instruments.push(instrument);
        }

        Ok(instruments)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_nbs_file() {
        let file_data = include_bytes!("../assets/nyan_cat.nbs");
        let mut parser = NbsParser::new(file_data);

        let file = parser.parse().unwrap();

        assert_eq!(file.header.version, 5);
        assert_eq!(file.header.song_name, b"Nyan Cat");
        assert_eq!(file.header.song_author, b"chenxi050402");
        assert_eq!(
            file.header.description,
            b"\"Nyan Cat\" recreated in note blocks by chenxi050402."
        );
        assert_eq!(file.header.original_author, b"");
        assert_eq!(file.header.song_origin, b"");

        assert_eq!(file.header.auto_save, false);
        assert_eq!(file.header.loop_flag, true);

        assert_eq!(file.header.default_instruments, 16);
        assert_eq!(file.header.auto_save_duration, 10);
        assert_eq!(file.header.time_signature, 8);
        assert_eq!(file.header.max_loop_count, 0);

        assert_eq!(file.header.song_length, 670);
        assert_eq!(file.header.song_layers, 36);
        assert_eq!(file.header.tempo, 1893);
        assert_eq!(file.header.loop_start, 160);

        assert_eq!(file.header.minutes_spent, 32);
        assert_eq!(file.header.left_clicks, 1207);
        assert_eq!(file.header.right_clicks, 32);
        assert_eq!(file.header.blocks_added, 212);
        assert_eq!(file.header.blocks_removed, 27);
    }
}
