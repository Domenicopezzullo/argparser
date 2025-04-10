use std::{collections::HashMap, env};

pub struct Option {
    pub usage: String,
    pub name: String,
    pub default: String,
}
pub struct Flag {
    pub usage: String,
    pub name: String,
    pub default: String,
    pub flag_type: FlagType,
}
pub struct Parser {
    pub flags: Vec<Flag>,
    pub options: Vec<Option>,
}

pub enum FlagType {
    STRING,
    INT,
    CHAR,
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser {
    pub fn new() -> Self {
        let flags: Vec<Flag> = Vec::new();
        let options: Vec<Option> = Vec::new();
        Parser { flags, options }
    }
    pub fn add_option(&mut self, name: String, usage: String, default: String) {
        self.options.push(Option {
            name,
            usage,
            default,
        });
    }
    pub fn add_flag(&mut self, name: String, usage: String, default: String, flag_type: FlagType) {
        self.flags.push(Flag {
            name,
            usage,
            default,
            flag_type,
        });
    }
    pub fn parse(self) -> Result<HashMap<String, String>, String> {
        let mut results = HashMap::new();

        for option in &self.options {
            results.insert(option.name.clone(), option.default.clone());
        }
        for flag in &self.flags {
            results.insert(flag.name.clone(), flag.default.clone());
        }

        let args: Vec<String> = env::args().collect();
        let mut i = 1;

        while i < args.len() {
            let arg = &args[i];

            if arg.starts_with("-") {
                let name = arg.trim_start_matches('-');

                if let Some(flag) = self.flags.iter().find(|f| f.name == name) {
                    if i + 1 < args.len() {
                        let value = &args[i + 1];
                        match flag.flag_type {
                            FlagType::INT => {
                                if value.parse::<i32>().is_err() {
                                    return Err(format!(
                                        "Value for flag {} must be an integer",
                                        name
                                    ));
                                }
                            }
                            FlagType::CHAR => {
                                if value.len() != 1 {
                                    return Err(format!(
                                        "Value for flag {} must be a single character",
                                        name
                                    ));
                                }
                            }
                            _ => {}
                        }

                        results.insert(name.to_string(), value.clone());
                        i += 1; // Skip the value in the next iteration
                    } else {
                        return Err(format!("Missing value for flag: {}", name));
                    }
                } else if let Some(option) = self.options.iter().find(|o| o.name == name) {
                    results.insert(option.name.clone(), "true".to_string());
                } else {
                    return Err(format!("Unknown flag or option: {}", name));
                }
            }

            i += 1;
        }

        Ok(results)
    }
}
