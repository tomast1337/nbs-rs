use core::fmt;
use std::fmt::{Formatter, Result};
use std::io::{self, Write};
use std::vec::Vec;

const CURRENT_NBS_VERSION: u8 = 5;

#[derive(Clone)]
pub struct Instrument {
    pub name: Vec<u8>,
    pub file: Vec<u8>,
    pub key: u8,
    pub press_key: u8,
}

impl std::fmt::Debug for Instrument {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Instrument {{ name: {:?}, file: {:?}, key: {}, press_key: {} }}",
            String::from_utf8(self.name.clone()).unwrap_or("Invalid UTF-8".into()),
            String::from_utf8(self.file.clone()).unwrap_or("Invalid UTF-8".into()),
            self.key,
            self.press_key
        )
    }
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
    pub fn new(
        tick: u16,
        layer: u16,
        instrument: u8,
        key: u8,
        velocity: Option<u8>,
        panning: Option<i8>,
        pitch: Option<i16>,
    ) -> Note {
        Note {
            tick,
            layer,
            instrument,
            key,
            velocity: velocity.unwrap_or(100), // default
            panning: panning.unwrap_or(100),   // default
            pitch: pitch.unwrap_or(0),         // default
        }
    }
}

#[derive(Clone)]
pub struct Layer {
    pub id: u16,
    pub name: Vec<u8>,
    pub lock: bool,
    pub volume: u8,
    pub panning: u8,
}

impl fmt::Debug for Layer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Layer {{ id: {}, name: {:?}, lock: {}, volume: {}, panning: {} }}",
            self.id,
            String::from_utf8(self.name.clone()).unwrap_or("Invalid UTF-8".into()),
            self.lock,
            self.volume,
            self.panning
        )
    }
}

#[derive(Clone)]
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

impl fmt::Debug for Header {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Header {{ version: {}, default_instruments: {}, song_length: {}, song_layers: {}, song_name: {:?}, song_author: {:?}, original_author: {:?}, description: {:?}, tempo: {}, auto_save: {}, auto_save_duration: {}, time_signature: {}, minutes_spent: {}, left_clicks: {}, right_clicks: {}, blocks_added: {}, blocks_removed: {}, song_origin: {:?}, loop_flag: {}, max_loop_count: {}, loop_start: {} }}",
            self.version,
            self.default_instruments,
            self.song_length,
            self.song_layers,
            String::from_utf8(self.song_name.clone()).unwrap_or("Invalid UTF-8".into()),
            String::from_utf8(self.song_author.clone()).unwrap_or("Invalid UTF-8".into()),
            String::from_utf8(self.original_author.clone()).unwrap_or("Invalid UTF-8".into()),
            String::from_utf8(self.description.clone()).unwrap_or("Invalid UTF-8".into()),
            self.tempo,
            self.auto_save,
            self.auto_save_duration,
            self.time_signature,
            self.minutes_spent,
            self.left_clicks,
            self.right_clicks,
            self.blocks_added,
            self.blocks_removed,
            String::from_utf8(self.song_origin.clone()).unwrap_or("Invalid UTF-8".into()),
            self.loop_flag,
            self.max_loop_count,
            self.loop_start
        )
    }
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

/// NbsFile is used to store the data from a .nbs file
/// It can be used to save the data to a file or to modify the data
/// and save it to a new file
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

/// NbsParser is used to parse the data from a .nbs file
/// The data is stored in the NbsFile struct
/// The parse method is used to parse the data from the file
/// An explanation of the NBS file format can be found here:
/// <https://opennbs.org/nbs>
pub struct NbsParser<'a> {
    data: &'a [u8],
}

impl<'a> NbsParser<'a> {
    pub fn new(file_data: &'a [u8]) -> NbsParser<'a> {
        NbsParser { data: file_data }
    }

    pub fn parse(&mut self) -> io::Result<NbsFile> {
        let header = self.parse_header()?;
        let notes = self.parse_notes()?;
        let layers = self.parse_layers(&header)?;
        let instruments = self.parse_instruments()?;
        Ok(NbsFile::new(header, notes, layers, instruments))
    }

    fn read<T: Default + Copy>(&mut self) -> Option<T> {
        let size = size_of::<T>();

        if self.data.len() < size {
            return None; // Avoid out-of-bounds access
        }

        let bytes = &self.data[..size];
        self.data = &self.data[size..];

        // SAFETY: Ensure T is plain-old-data (POD) and aligned
        let result = unsafe {
            let mut value: T = std::mem::zeroed();
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), &mut value as *mut T as *mut u8, size);
            value
        };

        Some(result)
    }

    fn read_string(&mut self) -> io::Result<Vec<u8>> {
        let len = self.read::<u32>().unwrap();
        let str = self.data[..len as usize].to_vec();
        self.data = &self.data[len as usize..];
        Ok(str)
    }

    fn parse_header(&mut self) -> io::Result<Header> {
        // remove the first 2 bytes
        self.data = &self.data[2..];
        self.data = &self.data[1..];
        Ok(Header {
            version: CURRENT_NBS_VERSION,
            default_instruments: self.read().unwrap(),
            song_length: self.read().unwrap(),
            song_layers: self.read().unwrap(),
            song_name: self.read_string()?,
            song_author: self.read_string()?,
            original_author: self.read_string()?,
            description: self.read_string()?,
            tempo: self.read().unwrap(),
            auto_save: self.read::<u8>().unwrap() != 0,
            auto_save_duration: self.read().unwrap(),
            time_signature: self.read().unwrap(),
            minutes_spent: self.read().unwrap(),
            left_clicks: self.read().unwrap(),
            right_clicks: self.read().unwrap(),
            blocks_added: self.read().unwrap(),
            blocks_removed: self.read().unwrap(),
            song_origin: self.read_string()?,
            loop_flag: self.read::<u8>().unwrap() != 0,
            max_loop_count: self.read().unwrap(),
            loop_start: self.read().unwrap(),
        })
    }

    fn parse_notes(&mut self) -> io::Result<Vec<Note>> {
        let mut notes = Vec::new();
        let mut current_tick = 0;

        while self.data.len() > 0 {
            /*
            The amount of "jumps" to the next tick with at least one note block in it.
            We start at tick -1. If the amount of jumps is 0, the program will stop reading and proceed to the next part.
            */
            let jump_ticks: u16 = self.read().unwrap();
            if jump_ticks == 0 {
                break;
            }

            current_tick += jump_ticks;

            while self.data.len() > 0 {
                /*
                The amount of "jumps" to the next tick with at least one note block in it.
                We start at tick -1. If the amount of jumps is 0, the program will stop reading and proceed to the next part.
                */
                let jump_layers: u16 = self.read().unwrap();
                if jump_layers == 0 {
                    break;
                }

                notes.push(Note {
                    tick: current_tick - 1,           // The tick of the note
                    layer: jump_layers - 1,           // The layer of the note
                    instrument: self.read().unwrap(), // The instrument of the note block. This is 0-15, or higher if the song uses custom instruments.
                    key: self.read().unwrap(), // The key of the note block, from 0-87, where 0 is A0 and 87 is C8. 33-57 is within the 2-octave limit.
                    velocity: self.read().unwrap(), // The velocity/volume of the note block, from 0% to 100%.
                    panning: self.read().unwrap(), // The stereo position of the note block, from 0-200. 0 is 2 blocks right, 100 is center, 200 is 2 blocks left.
                    pitch: self.read().unwrap(), // The fine pitch of the note block, from -32,768 to 32,767 cents (but the max in Note Block Studio is limited to -1200 and +1200). 0 is no fine-tuning. ±100 cents is a single semitone difference. After reading this, we go back to step 2.
                });
            }
        }
        Ok(notes)
    }

    fn parse_layers(&mut self, header: &Header) -> io::Result<Vec<Layer>> {
        let mut layers = Vec::new();

        for _ in 0..header.song_layers {
            let name = self.read_string()?;
            let lock = self.read::<u8>().unwrap() == 1; // Convert to bool
            let volume = self.read::<u8>().unwrap();
            let panning = self.read::<u8>().unwrap(); // Convert to signed (-100 to +100)
            layers.push(Layer {
                id: layers.len() as u16,
                name,
                lock,
                volume,
                panning,
            });
        }

        Ok(layers)
    }

    fn parse_instruments(&mut self) -> io::Result<Vec<Instrument>> {
        let mut instruments = Vec::new();

        let instrument_count = self.read::<u8>().unwrap();

        for _ in 0..instrument_count {
            let name = self.read_string()?;
            let file = self.read_string()?;
            let key = self.read::<u8>().unwrap();
            let press_key = self.read::<u8>().unwrap(); // 0 or 1

            instruments.push(Instrument {
                name,
                file,
                key,
                press_key,
            });
        }

        Ok(instruments)
    }
}

/// NbsWriter is used get the data from the NbsFile and write it to a Vec<u8>
/// The data can then be written to a file
pub struct NbsWriter {
    data: Vec<u8>,
}

impl NbsWriter {
    pub fn new() -> NbsWriter {
        NbsWriter { data: Vec::new() }
    }

    pub fn get_file_bytes(&mut self, file: &NbsFile) -> Vec<u8> {
        self.write_header(&file.header);
        self.write_notes(&file.notes);
        self.write_layers(&file.layers);
        self.write_instruments(&file.instruments);
        self.data.clone()
    }

    fn write<T: Default + Copy>(&mut self, data: T) -> &mut NbsWriter {
        let size = std::mem::size_of::<T>();
        let bytes = unsafe {
            let mut bytes = Vec::with_capacity(size);
            bytes.set_len(size);
            std::ptr::copy_nonoverlapping(&data as *const T as *const u8, bytes.as_mut_ptr(), size);
            bytes
        };
        self.data.write_all(&bytes).unwrap();
        self
    }

    fn write_string(&mut self, data: Vec<u8>) -> &mut NbsWriter {
        self.write::<u32>(data.len() as u32);
        self.data.write_all(&data).unwrap();
        self
    }

    fn write_header(&mut self, header: &Header) {
        // Write "New Format" marker (0 length)
        self.write::<u16>(0); 
        
        self.write::<u8>(header.version)
            .write::<u8>(header.default_instruments)
            .write::<u16>(header.song_length)
            .write::<u16>(header.song_layers)
            .write_string(header.song_name.clone())
            .write_string(header.song_author.clone())
            .write_string(header.original_author.clone())
            .write_string(header.description.clone())
            .write::<u16>(header.tempo)
            .write::<u8>(if header.auto_save { 1 } else { 0 })
            .write::<u8>(header.auto_save_duration)
            .write::<u8>(header.time_signature)
            .write::<u32>(header.minutes_spent)
            .write::<u32>(header.left_clicks)
            .write::<u32>(header.right_clicks)
            .write::<u32>(header.blocks_added)
            .write::<u32>(header.blocks_removed)
            .write_string(header.song_origin.clone())
            .write::<u8>(if header.loop_flag { 1 } else { 0 })
            .write::<u8>(header.max_loop_count)
            .write::<u16>(header.loop_start);
    }

    fn write_notes(&mut self, notes: &Vec<Note>) {
        if notes.is_empty() {
            self.write::<u16>(0); // End of notes
            return;
        }

        // 1. Sort notes by Tick, then by Layer
        let mut sorted_notes = notes.clone();
        sorted_notes.sort_by(|a, b| {
            if a.tick != b.tick {
                a.tick.cmp(&b.tick)
            } else {
                a.layer.cmp(&b.layer)
            }
        });

        let mut current_tick: i32 = -1;
        let mut current_layer: i32 = -1;

        for note in sorted_notes {
            let note_tick = note.tick as i32;
            let note_layer = note.layer as i32;

            // If we moved to a new tick
            if note_tick != current_tick {
                // If this isn't the very first tick processed, we need to close the 
                // previous tick's layer loop
                if current_tick != -1 {
                    self.write::<u16>(0); // End of layers for previous tick
                }

                // Write jump to this tick
                let jump_ticks = (note_tick - current_tick) as u16;
                self.write::<u16>(jump_ticks);

                current_tick = note_tick;
                current_layer = -1; // Reset layer for new tick
            }

            // Write jump to this layer
            let jump_layers = (note_layer - current_layer) as u16;
            self.write::<u16>(jump_layers);
            
            current_layer = note_layer;

            // Write Note Data
            self.write::<u8>(note.instrument)
                .write::<u8>(note.key)
                .write::<u8>(note.velocity)
                .write::<i8>(note.panning)
                .write::<i16>(note.pitch);
        }

        // Close the final tick's layer loop
        self.write::<u16>(0);
        // Close the note stream (0 jump ticks)
        self.write::<u16>(0);
    }

    fn write_layers(&mut self, layers: &Vec<Layer>) {
        for layer in layers {
            self.write_string(layer.name.clone())
                .write::<u8>(if layer.lock { 1 } else { 0 })
                .write::<u8>(layer.volume)
                .write::<u8>(layer.panning);
        }
    }

    fn write_instruments(&mut self, instruments: &Vec<Instrument>) {
        self.write::<u8>(instruments.len() as u8);
        for instrument in instruments {
            self.write_string(instrument.name.clone())
                .write_string(instrument.file.clone())
                .write::<u8>(instrument.key)
                .write::<u8>(instrument.press_key);
        }
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

        assert_eq!(file.notes.len(), 1021);
        assert_eq!(file.instruments.len(), 3);
    }

    #[test]
    fn write_nbs_file() {
        // read nyan_cat.nbs
        let file_data = include_bytes!("../assets/nyan_cat.nbs");
        let mut parser = NbsParser::new(file_data);
        let file = parser.parse().unwrap();

        // write nyan_cat.nbs
        let mut writer = NbsWriter::new();
        let data = writer.get_file_bytes(&file);
        // if the size of the data is the same it means the serialization is successful
        assert_eq!(data.len(), file_data.len());
    }
}
