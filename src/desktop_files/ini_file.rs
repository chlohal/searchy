use std::{collections::HashMap, fs, io::Error, path::PathBuf};

pub type IniFile = HashMap<String, HashMap<String, String>>;
    

const DEFAULT_SECTION_NAME: &str = "";


pub fn parse_ini_file(filename: &PathBuf) -> Result<IniFile, Error> {
    let mut ini = IniFile::default();

    let mut current_section_name;

    //Insert the default section to start
    ini.insert(DEFAULT_SECTION_NAME.into(), HashMap::default());
    let mut current_section = ini.get_mut(DEFAULT_SECTION_NAME.into()).unwrap();


    for line in fs::read_to_string(filename)?.split('\n') {
        let trimmed = line.trim();

        match trimmed.chars().next() {
            Some(first_char) => match first_char {
                '#' => continue, // Comments
                '[' => {
                    current_section_name = &trimmed[1..trimmed.len() - 1];
                    
                    if !ini.contains_key(current_section_name) {
                        ini.insert(current_section_name.into(), Default::default());
                    }

                    current_section = ini.get_mut(current_section_name).unwrap();
                },
                _ => {
                    let equals_index = trimmed.find('=');
                    match equals_index {
                        Some(i) => {
                            let variable_name = trimmed[..i].trim();
                            let value = trimmed[i+1..].trim();

                            current_section.insert(variable_name.into(), value.into());
                        }
                        None => return Err(std::io::ErrorKind::InvalidData.into())
                    }
                }
            },
            None => continue
        }
    }

    return Ok(ini);
}
