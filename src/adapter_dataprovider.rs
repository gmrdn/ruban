use crate::port_dataprovider::DataProvider;
use std::fs;

pub struct DataFile {
    pub filepath: String,
}

impl DataProvider for DataFile {
    fn get_data(&self) -> String {
        fs::read_to_string(&self.filepath).expect("Something went wrong reading the file")
    }
}
