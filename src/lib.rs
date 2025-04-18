use std::collections::HashMap;
use std::env;

#[derive(Default, Debug)]
pub struct ArgumentParser<'a> {
    flags: Vec<Flag<'a>>,
    options: Vec<Option<'a>>,
}

#[derive(Debug)]
pub struct Flag<'a> {
    pub name: &'a str,
    pub usage: &'a str,
}

#[derive(Debug)]
pub struct Option<'a> {
    pub name: &'a str,
    pub usage: &'a str,
    pub default: &'a str,
}

impl<'a> ArgumentParser<'a> {
    pub fn add_option(&mut self, name: &'a str, usage: &'a str, default: &'a str) {
        self.options.push(Option {
            name,
            usage,
            default,
        });
    }

    pub fn add_flag(&mut self, name: &'a str, usage: &'a str) {
        self.flags.push(Flag { name, usage })
    }

    pub fn parse(&self) -> Result<HashMap<&'a str, String>, String> {
        let mut results: HashMap<&'a str, String> = HashMap::new();
        let args: Vec<String> = env::args().collect();
        let mut i = 1;
        while i < args.len() {
            if let Some(option) = self
                .options
                .iter()
                .find(|o| o.name == args[i].trim_start_matches("-"))
            {
                if i + 1 < args.len() {
                    results.insert(option.name, args[i + 1].clone());
                    i += 2;
                } else {
                    results.insert(option.name, option.default.to_string());
                    i += 1;
                }
            } else if let Some(flag) = self
                .flags
                .iter()
                .find(|f| f.name == args[i].trim_start_matches("-"))
            {
                results.insert(flag.name, "true".to_string());
                i += 1;
            }
        }
        Ok(results)
    }
}
