use std::io::{self, Write};
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

    fn read<T: Default + Copy>(&mut self) -> T {
        let size = std::mem::size_of::<T>();
        let bytes = &self.current_data[..size];
        self.current_data = &self.current_data[size..];
        let mut result = T::default();
        unsafe {
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), &mut result as *mut T as *mut u8, size);
        }
        result
    }

    fn read_string(&mut self) -> io::Result<Vec<u8>> {
        let len = self.read::<u32>() as usize;
        let str = self.current_data[..len].to_vec();
        self.current_data = &self.current_data[len..];
        Ok(str)
    }

    fn parse_header(&mut self) -> io::Result<Header> {
        // remove the first 2 bytes
        self.current_data = &self.current_data[2..];
        self.current_data = &self.current_data[1..];
        Ok(Header {
            version: CURRENT_NBS_VERSION,
            default_instruments: self.read(),
            song_length: self.read(),
            song_layers: self.read(),
            song_name: self.read_string()?,
            song_author: self.read_string()?,
            original_author: self.read_string()?,
            description: self.read_string()?,
            tempo: self.read(),
            auto_save: self.read::<u8>() != 0,
            auto_save_duration: self.read(),
            time_signature: self.read(),
            minutes_spent: self.read(),
            left_clicks: self.read(),
            right_clicks: self.read(),
            blocks_added: self.read(),
            blocks_removed: self.read(),
            song_origin: self.read_string()?,
            loop_flag: self.read::<u8>() != 0,
            max_loop_count: self.read(),
            loop_start: self.read(),
        })
    }

    fn parse_notes(&mut self) -> io::Result<Vec<Note>> {
        let mut notes = Vec::new();
        let mut current_tick: u16 = 0;

        while self.current_data.len() > 0 {
            let jump_ticks: u16 = self.read();
            if jump_ticks == 0 {
                break;
            }

            current_tick += jump_ticks;

            while self.current_data.len() > 0 {
                let jump_layers: u16 = self.read();
                if jump_layers == 0 {
                    break;
                }

                notes.push(Note {
                    tick: current_tick,
                    layer: jump_layers,
                    instrument:self.read(),
                    key:self.read(),
                    velocity:self.read(),
                    panning:self.read(),
                    pitch:self.read(),
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

            layers.push(Layer {
                id: layer_id,
                name,
                lock:self.read::<u8>() != 0,
                volume:self.read::<u8>(),
                panning:self.read::< i8>(),
            });

            layer_id += 1;
        }

        Ok(layers)
    }

    fn parse_instruments(&mut self) -> io::Result<Vec<Instrument>> {
        let mut instruments = Vec::new();
        // next u8 is the number of instruments
        let num_instruments = self.read::<i8>();

        for _ in 0..num_instruments {
            let name = self.read_string()?;
            let sound_file = self.read_string()?;
            let sound_key = self.read::<u8>();
            let press_key = self.read::<u8>();

            let mut instrument = Instrument::new(sound_key, name, sound_file, press_key);
            instrument.press_key = press_key;
            instruments.push(instrument);
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

    pub fn write_file(&mut self, file: &NbsFile) -> Vec<u8> {
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

    fn write_header(&mut self, header:&Header) {
        // add 2 bytes for the version
        self.write::<u8>(0).write::<u8>(0);
        // write the header
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
    fn write_notes(&mut self, notes:&Vec<Note>) {
        let mut current_tick: u16 = 0;
        for note in notes {
            let jump_ticks = note.tick - current_tick;
            self.write::<u16>(jump_ticks);
            current_tick = note.tick;

            self.write::<u16>(note.layer)
            .write::<u8>(note.instrument)
            .write::<u8>(note.key)
            .write::<u8>(note.velocity)
            .write::<i8>(note.panning)
            .write::<i16>(note.pitch);
        }
    }
    fn write_layers(&mut self, layers:&Vec<Layer>) {
        
        for layer in layers {
            self.write_string(layer.name.clone())
            .write::<u8>(if layer.lock { 1 } else { 0 })
            .write::<u8>(layer.volume)
            .write::<i8>(layer.panning);
        }
    }
    fn write_instruments(&mut self, instruments:&Vec<Instrument>) {
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
    }

    #[test]
    fn save_nbs_file() {
        let file_data = include_bytes!("../assets/nyan_cat.nbs");
        let mut parser = NbsParser::new(file_data);
        let file = parser.parse().unwrap();

        let mut writer = NbsWriter::new();
        let new_data = writer.write_file(&file);

        // save new data to a file
        let mut new_file = std::fs::File::create("./assets/nyan_cat_new.nbs").unwrap();
        new_file.write_all(new_data.as_slice()).unwrap();
    
        assert_eq!(file_data, new_data.as_slice());
    }
}
