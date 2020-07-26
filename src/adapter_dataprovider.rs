// use crate::port_dataprovider::DataProvider;
// use std::fs;
// use std::io::Write;
// use std::error::Error;
// use std::fs::File;
//
// pub struct DataFile {
//     pub file: File,
// }
//
// impl DataProvider for DataFile {
//     fn get_data(&self) -> String {
//         fs::read_to_string(&self.file).expect("Something went wrong reading the file")
//     }
//
//
// }
