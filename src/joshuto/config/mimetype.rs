extern crate toml;
extern crate xdg;

use std::fmt;
use std::fs;
use std::collections::HashMap;
use std::process;

#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize)]
pub enum ExecType {
    forking
}

#[derive(Debug, Deserialize)]
pub struct JoshutoMimetypeEntry {
    pub program: String,
    pub args: Option<Vec<String>>,
    pub exec_type: Option<String>,
}

impl std::fmt::Display for JoshutoMimetypeEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        let mut fmt_result = f.write_str(self.program.as_str());
        match self.args.as_ref() {
            Some(s) => {
                for arg in s {
                    fmt_result = write!(f, "{} ", arg);
                }
            },
            None => {},
        }
        match self.exec_type.as_ref() {
            Some(s) => {
                fmt_result = write!(f, "\t({})", s);
            },
            None => {},
        }
        fmt_result
    }
}

#[derive(Debug, Deserialize)]
pub struct JoshutoRawMimetype {
    mimetypes: Option<HashMap<String, Vec<JoshutoMimetypeEntry>>>,
}

impl JoshutoRawMimetype {
    #[allow(dead_code)]
    pub fn new() -> Self
    {
        JoshutoRawMimetype {
            mimetypes: None,
        }
    }

    pub fn flatten(self) -> JoshutoMimetype
    {
        let mimetypes = match self.mimetypes {
            Some(s) => s,
            None => HashMap::new(),
            };

        JoshutoMimetype {
            mimetypes,
        }
    }
}

#[derive(Debug)]
pub struct JoshutoMimetype {
    pub mimetypes: HashMap<String, Vec<JoshutoMimetypeEntry>>,
}

impl JoshutoMimetype {

    pub fn new() -> Self
    {
        JoshutoMimetype {
            mimetypes: HashMap::new(),
        }
    }

    fn read_config() -> Option<JoshutoRawMimetype>
    {
        match xdg::BaseDirectories::with_profile(::PROGRAM_NAME, "") {
            Ok(dirs) => {
                let config_path = dirs.find_config_file(::MIMETYPE_FILE)?;
                match fs::read_to_string(&config_path) {
                    Ok(config_contents) => {
                        match toml::from_str(&config_contents) {
                            Ok(config) => {
                                Some(config)
                            },
                            Err(e) => {
                                eprintln!("{}", e);
                                process::exit(1);
                            },
                        }
                    },
                    Err(e) => {
                        eprintln!("{}", e);
                        None
                    },
                }
            },
            Err(e) => {
                eprintln!("{}", e);
                None
            },
        }
    }

    pub fn get_config() -> Self
    {
        match Self::read_config() {
            Some(config) => {
                config.flatten()
            }
            None => {
                Self::new()
            }
        }
    }
}
