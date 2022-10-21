pub struct Lib;
pub struct Song {
    name: String,
    path: String,
}

use json::*;
use std::fs;

impl Lib {
    pub fn find_music(path: String) -> String {
        println!("Current Reading Directory : {:?}", path);
        let mut song = object!{
            songs: []
        };
        let mut counter = 0;
        let x = fs::read_dir(path).unwrap();

        for entry in x {
            let entry = entry.unwrap();
            println!("{:?}", entry.path());

            song["songs"][counter] = object! {
                name : "Test",
                path : entry.path().into_os_string().into_string().unwrap(),
            };
            counter += 1;
        }
        // Return the song variable to JS
        // println!("{}",stringify(song));
        return stringify(song);
    }
}