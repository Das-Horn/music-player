pub struct Lib;

use json::*;
use std::fs;

impl Lib {
    pub fn find_music(path: String) -> String {
        let mut song = object!{
            songs: []
        }; // Init JSON return
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
        return stringify(song);
    }

    pub fn save_def_conf() {
        // TODO : ADD default config save
    }

    pub fn read_conf() {
        // TODO : Read config File
    }

    pub fn write_conf() {
        // TODO : Write new config
    }

    pub fn get_song_info() {
        // TODO : Read Song Metadata
    }
}