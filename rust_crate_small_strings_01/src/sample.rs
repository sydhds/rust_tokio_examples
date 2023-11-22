use std::fs::File;

use argh::FromArgs;
use parse_display::{Display, FromStr};
use serde::Deserialize;
use smartstring;

#[derive(FromArgs)]
#[argh(description = "Run sample code")]
#[argh(subcommand, name = "sample")]
pub struct Sample {
    #[argh(option)]
    #[argh(description = "which string lib to use: Std, Smol, SmartString?")]
    lib: Lib,
}

#[derive(Display, FromStr)]
#[display(style = "snake_case")]
enum Lib {
    Std,
    Smol,
    Smart,
}

impl Sample {
    pub fn run(self) {
        // todo!();
        // self.read_records();
        match self.lib {
            Lib::Std => self.read_records::<String>(),
            Lib::Smol => self.read_records::<smol_str::SmolStr>(),
            Lib::Smart => {
                self.read_records::<smartstring::alias::String>();
            }
        }
    }

    fn read_records<S>(&self)
    where
        S: serde::de::DeserializeOwned,
    {
        #[derive(Deserialize)]
        struct Record<S> {
            #[allow(unused)]
            city: S,
            #[allow(unused)]
            state: S,
        }

        let f = File::open("cities.json").unwrap();
        // Activate allocator 'log' only for json deserialisation
        crate::ALLOCATOR.set_active(true);
        let records: Vec<Record<S>> = serde_json::from_reader(f).unwrap();
        crate::ALLOCATOR.set_active(false);
        println!("Read {} record", records.len());
    }
}
