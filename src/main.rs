use walkdir::WalkDir;
use std::env;
use std::process::Command;
use std::collections::HashMap;


/* command `sst_dump --file=name.sst --show_properties` on one file looks like this:
from [] to []
Process 1013115.sst
Sst file format: block-based
Table Properties:
------------------------------
  # data blocks: 3247
  # entries: 6425
  # range deletions: 0
  raw key size: 257000
  raw average key size: 40.000000
  raw value size: 97879517
  raw average value size: 15234.166070
  data block size: 67125545
  index block size (user-key? 0, delta-value? 0): 80959
  filter block size: 0
  (esddtimated) table size: 67206504
  filter policy name: N/A
  prefix extractor name: nullptr
  column family ID: 3
  column family name: col2
  comparator name: leveldb.BytewiseComparator
  merge operator name: nullptr
  property collectors names: []
  SST file compression algo: Snappy
  creation time: 1602909152
  time stamp of earliest key: 0
  # deleted keys: 0
  # merge operands: 0
Raw user collected properties
------------------------------
  # rocksdb.block.based.table.index.type: 0x00000000
  # rocksdb.block.based.table.prefix.filtering: 0x30
  # rocksdb.block.based.table.whole.key.filtering: 0x31
  # rocksdb.deleted.keys: 0x00
  # rocksdb.merge.operands: 0x00

*/

struct Data {
    col: String,
    data_blocks: i64,
    entries: i64,
    raw_key_size: i64,
    raw_value_size: i64,
    data_block_size: i64,
    index_block_size: i64,
    estimated_table_size: i64,

}

impl Data {
    pub fn new() -> Data {
        Data {
            col: String::new(),
            data_blocks: 0,
            entries: 0,
            raw_key_size: 0,
            raw_value_size: 0,
            data_block_size: 0,
            index_block_size: 0,
            estimated_table_size: 0,
        }
    }

    pub fn add(&mut self, other: &Data) {
        self.data_blocks += other.data_blocks;
        self.entries += other.entries;
        self.raw_key_size += other.raw_key_size;
        self.raw_value_size += other.raw_value_size;
        self.data_block_size += other.data_block_size;
        self.index_block_size += other.index_block_size;
        self.estimated_table_size += other.estimated_table_size;
    }

    pub fn print(&self) {
        println!("col:{}, data_blocks:{}, entries:{},
                 raw_key_size:{}, raw_value_size:{},
                 data_block_size:{}, index_block_size:{}, estimated_table_size:{}",
                 self.col, self.data_blocks, self.entries, 
                 self.raw_key_size, self.raw_value_size, self.data_block_size,
                 self.index_block_size, self.estimated_table_size); 
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut column_data : HashMap<String,Data> = HashMap::new();
    for entry in WalkDir::new(&args[1])
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok()) {
        let f_name = entry.file_name().to_string_lossy();

        if f_name.ends_with(".sst") {
            println!("{} path: {}", f_name, entry.path().to_str().unwrap());
            let output = Command::new("sst_dump")
                .arg(format!("--file={}",entry.path().to_str().unwrap()))
                .arg("--show_properties")
                .output()
                .expect("ls command failed to start");
            
            let mut data = Data::new();


            let out = String::from_utf8(output.stdout)?;
            for line in out.lines() {
                    let split_line : Vec<&str> = line.split(':').collect();
                    if split_line.len() < 2 {
                        continue;
                    }
                    let line = split_line[0].trim_start();
                    let value = split_line[1].trim();

                    if line.starts_with("column family name")      { data.col = value.to_string(); }
                    else if line.starts_with("# data_blocks") { data.data_blocks = value.parse::<i64>().unwrap(); }
                    else if line.starts_with("# entries") { data.entries = value.parse::<i64>().unwrap(); }
                    else if line.starts_with("raw value size") { data.raw_value_size = value.parse::<i64>().unwrap(); }
                    else if line.starts_with("raw key size") { data.raw_key_size = value.parse::<i64>().unwrap(); }
                    else if line.starts_with("data block size") { data.data_block_size = value.parse::<i64>().unwrap(); }
                    else if line.starts_with("index block size") { data.index_block_size = value.parse::<i64>().unwrap(); }
                    else if line.starts_with("(estimated) table size") { data.estimated_table_size = value.parse::<i64>().unwrap(); }
                    else                          {}


            }
            if let Some(x) = column_data.get_mut(&data.col) {
                x.add(&data);
            } else {
                column_data.insert(data.col.clone(),data);
            }

        }

    }
    for (k, v) in column_data.iter() {
        v.print();
    }

    Ok(())
}
