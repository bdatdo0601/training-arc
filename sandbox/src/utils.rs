use std::{
    fs::File,
    io::{BufReader, Error, ErrorKind},
    path::PathBuf,
}; // path buffer, to construct paths

pub fn parse_json_file<T: serde::de::DeserializeOwned>(path: &PathBuf) -> Result<T, Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let schedule =
        serde_json::from_reader(reader).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

    Ok(schedule)
}
