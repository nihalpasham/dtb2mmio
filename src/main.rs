use std::fs;
use std::io::Read;

mod dt;

use crate::dt::NodeItems;
use crate::dt::{patch::parse_raw_node, Reader, StructItem};

fn remove_trailing_path_component(url: &str) -> &str {
    let mut last_slash = 0;
    for (i, c) in url.trim_end_matches("/").char_indices().rev() {
        if c == '/' {
            last_slash = i;
            break;
        }
    }
    &url[..last_slash + 1]
}

fn bytes_to_u32(bytes: &[u8]) -> Result<Vec<u32>, &'static str> {
    if bytes.len() % 4 != 0 {
        return Err("invalid 'reg' entry: must be a multiple of 4");
    }
    let mut reg_addrs = Vec::new();
    for chunk in bytes.chunks(4) {
        let addr = u32::from_be_bytes(chunk.try_into().unwrap());
        reg_addrs.push(addr);
    }
    Ok(reg_addrs)
}

fn main() {
    let mut buf = Vec::new();

    let ph = std::env::args().nth(2).expect("please provide a peripheral name, example: i2c, spi etc.");
    let mut file = fs::File::open(
        std::env::args()
            .nth(1)
            .expect("Need path to DTB file as argument"),
    )
    .unwrap();
    file.read_to_end(&mut buf).unwrap();
    let reader = Reader::read(buf.as_slice()).unwrap();

    let _ = log_init();

    let mut node_string = String::new();
    let mut node_depth = 0usize;

    for entry in reader.struct_items() {
        match entry {
            StructItem::BeginNode { name } => {
                node_depth += 1;

                node_string.push_str(name);
                node_string.push_str("/");

                if node_string.contains(ph.as_str()) {
                    // only supports nodes with upto 500 properties.
                    let parsed_node = parse_raw_node::<500>(&reader, node_string.as_str(), &buf);
                    match parsed_node {
                        Ok(val) => {
                            for item in val {
                                if item.0 == "reg" {
                                    match item.1 {
                                        NodeItems::RawPropertyConstructor(property) => {
                                            let reg_addrs = bytes_to_u32(property.prop_val());
                                            match reg_addrs {
                                                Ok(addr) => {
                                                    print!(
                                                        "node-depth: {:?}, {}: ",
                                                        node_depth, node_string,
                                                    );
                                                    print!("<");
                                                    addr.iter().for_each(|x| print!("0x{:x}, ", x));
                                                    print!("\u{8}\u{8}>\n");
                                                }
                                                Err(e) => {
                                                    println!("{}", e)
                                                }
                                            }
                                        }
                                        _ => {
                                            unreachable!()
                                        }
                                    }
                                }
                            }
                        }
                        Err(e) => println!("{:?}", e),
                    }
                }
            }
            StructItem::EndNode => {
                if node_depth > 0 {
                    node_depth -= 1;
                    node_string = remove_trailing_path_component(node_string.as_str()).to_string();
                } else {
                }
            }
            StructItem::Property { name: _, value: _ } => {}
            _ => {
                panic!("invalid device tree blob")
            }
        }
    }
}

use log::{Level, Metadata, Record};
use log::{LevelFilter, SetLoggerError};

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("\x1b[93m[{}]\x1b[0m  {}", record.level(), record.args());
            match (record.module_path(), record.line()) {
                (Some(file), Some(line)) => {
                    println!("\t \u{2319} {} @ line:{}", file, line);
                }
                (_, None) => {
                    println!("... ")
                }
                (_, Some(line)) => println!("\t  \u{2a3d} {} @ line:{}", record.target(), line),
                // (Some(file), None) => println!("\t  \u{2a3d} @ {}", file),
            }
        }
    }

    fn flush(&self) {}
}

pub fn log_init() -> core::result::Result<(), SetLoggerError> {
    log::set_boxed_logger(Box::new(SimpleLogger)).map(|()| log::set_max_level(LevelFilter::Info))
}
