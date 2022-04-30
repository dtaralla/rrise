/*
 * Copyright (c) 2022 Contributors to the Rrise project
 */

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::BTreeMap;
use std::env;
use std::ffi::OsStr;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq, Ord, PartialOrd)]
enum ResourceType {
    Event,
    DialogueEvent,
    SwitchGroup,
    Switch,
    StateGroup,
    State,
    Rtpc,
    Trigger,
    Bus,
    AuxBus,
    AudioDevices,
    Skip,
}

impl ResourceType {
    fn from(s: &str) -> Self {
        match s {
            "Event" => Self::Event,
            "Dialogue Event" => Self::DialogueEvent,
            "Switch Group" => Self::SwitchGroup,
            "Switch" => Self::Switch,
            "State Group" => Self::StateGroup,
            "State" => Self::State,
            "Game Parameter" => Self::Rtpc,
            "Trigger" => Self::Trigger,
            "Audio Bus" => Self::Bus,
            "Auxiliary Bus" => Self::AuxBus,
            "Audio Devices" => Self::AudioDevices,
            _ => Self::Skip,
        }
    }

    fn is_grouped(&self) -> bool {
        match self {
            Self::State | Self::Switch => true,
            _ => false,
        }
    }
}

impl Display for ResourceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceType::Event => write!(f, "ev"),
            ResourceType::DialogueEvent => write!(f, "d_ev"),
            ResourceType::SwitchGroup | ResourceType::Switch => write!(f, "sw"),
            ResourceType::StateGroup | ResourceType::State => write!(f, "st"),
            ResourceType::Rtpc => write!(f, "rtpc"),
            ResourceType::Trigger => write!(f, "trg"),
            ResourceType::Bus => write!(f, "bus"),
            ResourceType::AuxBus => write!(f, "xbus"),
            ResourceType::AudioDevices => write!(f, "dev"),
            other => panic!("Resource type is not exportable: {:?}!", other),
        }
    }
}

fn main() {
    if env::var("DOCS_RS").is_ok() {
        return;
    }

    println!("cargo:rerun-if-env-changed=BANK_PATH");

    let mut bank_path =
        PathBuf::from(env::var("BANK_PATH").expect("env variable BANK_PATH not found"));

    if !bank_path.is_dir() {
        panic!(
            "BANK_PATH doesn't exist or is not a directory: {}",
            bank_path.to_str().unwrap()
        );
    }

    #[cfg(not(windows))]
    {
        bank_path = bank_path.join("Linux");
    }

    #[cfg(windows)]
    {
        bank_path = bank_path.join("Windows");
    }

    if !bank_path.is_dir() {
        // Maybe they don't care about platforms and gave the folder of the only platform they care about
        bank_path = bank_path.parent().unwrap().to_path_buf();
    };

    let init_bank_path = bank_path.join("Init.txt");
    let mut soundbank_paths = vec![init_bank_path];

    // Find localized dir if it exists
    let any_language_dir;
    #[allow(unused_must_use)]
    {
        let mut it = WalkDir::new(bank_path.clone()).max_depth(1).into_iter();
        it.next().unwrap(); // First entry is always the '.' directory

        // All languages dir have a copy of the bank for the resources we're interested in. Pick
        // the first one and go with it.
        any_language_dir = it.next();
    }

    match any_language_dir {
        Some(Ok(dir)) => {
            for fr in WalkDir::new(dir.path()).max_depth(1) {
                match fr {
                    Ok(f) => {
                        if let Some(ext) = f.path().extension() {
                            if ext == OsStr::new("txt") {
                                soundbank_paths.push(f.into_path());
                            }
                        }
                    }
                    Err(e) => panic!("Couldn't walk {} - {}", dir.path().to_str().unwrap(), e),
                };
            }
        }
        Some(Err(e)) => panic!("Couldn't walk {} - {}", bank_path.to_str().unwrap(), e),
        None => (),
    };

    // ---- COLLECT DATA
    type Resource = (String, u32);
    let mut resources: BTreeMap<ResourceType, Vec<Resource>> = BTreeMap::new();
    let mut grouped_resources: BTreeMap<String, BTreeMap<String, Vec<Resource>>> = BTreeMap::new();
    for bank_file in soundbank_paths.iter() {
        println!("cargo:rerun-if-changed={}", bank_file.to_str().unwrap());

        let bank = File::open(bank_file)
            .unwrap_or_else(|_| panic!("Not found: {}", bank_file.to_str().unwrap()));

        let mut current_type = ResourceType::Skip;
        for line in BufReader::new(bank).lines() {
            let line = line.unwrap();
            if line.is_empty() {
                continue;
            }

            // Header line
            if !line.starts_with('\t') {
                let end_type_index = line.find('\t').unwrap_or_else(|| {
                    panic!(
                        "File '{}' malformed at line '{}'",
                        bank_file.to_str().unwrap(),
                        line
                    )
                });
                current_type = ResourceType::from(&line[0..end_type_index]);
            }
            // Resource line
            else if current_type != ResourceType::Skip {
                #[allow(clippy::single_char_pattern)]
                let mut columns = line.split("\t");
                columns.next().unwrap(); // skip first column: always empty

                let id = columns.next().unwrap_or_else(|| {
                    panic!(
                        "File '{}' malformed at line '{}' (missing ID)",
                        bank_file.to_str().unwrap(),
                        line
                    )
                });
                let id = id.parse::<u32>().unwrap_or_else(|e| {
                    panic!(
                        "File '{}' malformed at line '{}' (ID is not a u32: {})",
                        bank_file.to_str().unwrap(),
                        line,
                        e,
                    )
                });

                let name = columns.next().unwrap_or_else(|| {
                    panic!(
                        "File '{}' malformed at line '{}' (missing ID)",
                        bank_file.to_str().unwrap(),
                        line
                    )
                });
                let name = to_rust_name(name);

                if current_type.is_grouped() {
                    let group_name = columns.next().unwrap_or_else(|| {
                        panic!(
                            "File '{}' malformed at line '{}' (missing group name)",
                            bank_file.to_str().unwrap(),
                            line
                        )
                    });
                    let group_name = to_rust_name(group_name);
                    grouped_resources
                        .entry(current_type.to_string())
                        .or_default()
                        .entry(group_name)
                        .or_default()
                        .push((name, id));
                } else {
                    resources.entry(current_type).or_default().push((name, id));
                }
            }
        }
    }

    // ---- WRITE HEADER FILE
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut out =
        File::create(out_path.join("rr.rs")).expect("Couldn't open headers file for writing");

    out.write_fmt(format_args!(
        "/* automatically generated by {} {} */\n",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    ))
    .expect("Failed to write headers");

    out.write_all("pub mod bnk {\n".as_bytes())
        .expect("Failed to write headers");
    for bnk in soundbank_paths.iter() {
        let bnk = bnk.file_stem().unwrap().to_str().unwrap();
        out.write_fmt(format_args!(
            "\tpub const {}: &str = \"{}.bnk\";\n",
            to_rust_name(bnk),
            bnk
        ))
        .expect("Failed to write headers");
    }
    out.write_all("}\n".as_bytes())
        .expect("Failed to write headers");

    for (res_type, res) in resources.iter_mut() {
        out.write_fmt(format_args!("\npub mod {} {{\n", res_type))
            .expect("Failed to write headers");

        res.sort_by(|(n1, _), (n2, _)| n1.cmp(n2));

        for (res_name, res_id) in res.iter() {
            out.write_fmt(format_args!(
                "\tpub const {}: u32 = {};\n",
                res_name, res_id
            ))
            .expect("Failed to write headers");

            if let Some(groups_of_type) = grouped_resources.get_mut(&res_type.to_string()) {
                if let Some(group) = groups_of_type.get_mut(res_name) {
                    out.write_fmt(format_args!("\n\tpub mod {} {{\n", res_name))
                        .expect("Failed to write headers");

                    group.sort_by(|(n1, _), (n2, _)| n1.cmp(n2));

                    for (res_name, res_id) in group.iter() {
                        out.write_fmt(format_args!(
                            "\t\tpub const {}: u32 = {};\n",
                            res_name, res_id
                        ))
                        .expect("Failed to write headers");
                    }

                    out.write_all("\t}\n\n".as_bytes())
                        .expect("Failed to write headers");
                }
            }
        }

        out.write_all("}\n".as_bytes())
            .expect("Failed to write headers");
    }
}

fn to_rust_name(name: &str) -> String {
    lazy_static! {
        static ref RE_INVALID_CHARS: Regex = Regex::new(r"\P{XID_CONTINUE}+").unwrap();
    }

    // Replace invalid chars sequences with _
    let mut rust_name = RE_INVALID_CHARS.replace_all(name, "_").to_string();

    // _ is not an identifier; becomes __
    if name == "_" {
        rust_name = "__".to_string();
    }
    // identifiers can't start with X in [0-9]; X... becomes _X...
    else if name.starts_with(char::is_numeric) {
        rust_name = "_".to_string() + &rust_name;
    }

    rust_name
}
