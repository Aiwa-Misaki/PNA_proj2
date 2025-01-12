use crate::error::{KvsError, Result};
use buffered_offset_reader::{BufOffsetReader, OffsetReadMut};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::{read_to_string, File, OpenOptions};
use std::io::{LineWriter, Write};
use std::ops::Add;
use std::path::{Path, PathBuf};

/// KvStore as a memory store struct
pub struct KvStore {
    filepath: PathBuf,
    // value is a tuple of (offset, length(Byte))
    map: HashMap<String, (u64, usize)>,
    // old values in log file
    // when overwrite or remove key, redundant will incr 1
    // when redundant / total size > 0.3, compaction is triggered
    redundant: i32,
}

impl KvStore {
    /// set map[key]=value; if key in map, then overwrite
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let (offset, length) =
            self.write_value_to_file(self.filepath.clone(), key.clone(), value.clone())?;

        if self.map.insert(key, (offset, length)) != None {
            self.redundant += 1;
        };

        self.compact_log();

        Ok(())
    }

    /// get map[key]; if key not in map, then None
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        // println!("222{:?}", self.filepath.clone());
        if self.map.contains_key(&key) {
            let offset = self.map[&key].0;
            let length = self.map[&key].1;

            self.get_value_from_file(offset, length)
        } else {
            Ok(None)
        }
    }

    /// remove k-v pair from map
    pub fn remove(&mut self, key: String) -> Result<()> {
        if !self.map.contains_key(&key) {
            return Err(KvsError::RmKeyError("Key not found".to_string()));
        }

        let log = Log {
            op: OpType::Remove,
            key: key.clone(),
            value: "".to_string(),
        };
        let mut serialized = serde_json::to_string(&log)?;
        let data_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.filepath.as_path())?;

        let mut data_file = LineWriter::new(data_file);

        // Write to a file
        serialized += "\n";
        data_file.write(serialized.as_bytes())?;
        data_file.flush()?;

        self.map.remove(&key);

        self.redundant += 1;

        self.compact_log();

        Ok(())
    }

    /// function to read log file and load to hash map in memory
    /// used both in open and get(?)
    fn read_to_map(&mut self) -> Result<()> {
        self.map.clear();
        if !self.filepath.exists() {
            return Ok(());
        }

        let mut result = Vec::new();
        for line in read_to_string(self.filepath.clone())?.lines() {
            result.push(line.to_string())
        }

        let mut offset = 0;

        for log in result.iter() {
            let deserialized: Log = serde_json::from_str(&log)?;
            let length = log.len();
            match deserialized.op {
                OpType::Set => {
                    self.map
                        .insert(deserialized.key.clone(), (offset, length + 1));
                }
                OpType::Remove => {
                    self.map.remove(&deserialized.key);
                }
            }
            offset = offset.add(length as u64 + 1);
        }
        Ok(())
    }

    /// open a file in disk as KvStore
    /// now only record value in map
    pub fn open(path: &Path) -> Result<Self> {
        let filename = "store.log";
        let mut filepath = path.to_path_buf();
        if filepath.is_dir() {
            filepath.push(filename);
        }
        let mut kv = KvStore {
            filepath: filepath.clone(),
            map: Default::default(),
            redundant: 0,
        };

        kv.read_to_map()?;

        Ok(kv)
    }

    fn write_value_to_file(
        &self,
        filepath: PathBuf,
        key: String,
        value: String,
    ) -> Result<(u64, usize)> {
        let log = Log {
            op: OpType::Set,
            key: key.clone(),
            value: value.clone(),
        };

        let mut serialized = serde_json::to_string(&log)?;
        let data_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(filepath.as_path())?;
        let offset = data_file.metadata()?.len();
        let mut file = LineWriter::new(data_file);

        // Write to a file
        serialized += "\n";
        file.write(serialized.as_bytes())?;
        file.flush()?;

        file.get_ref().sync_data()?;

        let length = serialized.len();

        Ok((offset, length))
    }

    fn get_value_from_file(&self, offset: u64, length: usize) -> Result<Option<String>> {
        let file = OpenOptions::new()
            .read(true)
            .open(self.filepath.as_path())?;
        let mut r = BufOffsetReader::new(file);
        let mut buf = vec![0; length];

        r.read_at(&mut buf, offset)?; // read length bytes at offset

        let string = String::from_utf8(buf)
            .expect("Bytes not valid utf8")
            .trim()
            .to_string();

        let deserialized: Log = serde_json::from_str(string.as_str())?;

        Ok(Some(deserialized.value))
    }

    /// compact
    pub fn compact_log(&mut self) {
        if (self.redundant as f32 / self.map.clone().len() as f32) <= 0.3 {
            return;
        }
        // create a new file when compacting
        let mut old_path = self.filepath.clone();
        self.read_to_map().unwrap();
        old_path.pop();
        old_path.push("store_new.log".to_string());

        File::create(old_path.clone()).expect("create failed");

        let mut map_new: HashMap<String, (u64, usize)> = HashMap::new();

        for (key, (offset, length)) in self.map.clone() {
            let value = self.get_value_from_file(offset, length).unwrap();
            let (o, l) = self
                .write_value_to_file(old_path.clone(), key.clone(), value.unwrap())
                .expect("TODO: panic message");

            map_new.insert(key, (o, l));
        }

        let mut new_name = old_path.clone();
        new_name.pop();
        new_name.push("store.log");

        fs::remove_file(self.filepath.clone()).unwrap();

        fs::rename(old_path, new_name).unwrap();

        self.map = map_new;

        self.redundant = 0;
    }
}

// used to represent operation in log
#[derive(Serialize, Deserialize, Debug)]
enum OpType {
    Set,
    Remove,
}

// used to store log in memory
#[derive(Serialize, Deserialize, Debug)]
struct Log {
    op: OpType,
    key: String,
    value: String,
}
