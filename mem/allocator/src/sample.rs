use argh::FromArgs;

#[derive(FromArgs)]
#[argh(subcommand, name = "sample", description = "sampling")]
pub struct Sample {}

impl Sample {
    pub fn run(self) {
        self.read_records();
    }

    fn read_records(&self) {
        use serde::Deserialize;

        #[derive(Deserialize)]
        struct Record {
            #[allow(unused)]
            city: String,
            #[allow(unused)]
            state: String,
        }

        use std::fs::File;
        let f = File::open("cities.json").unwrap();
        unsafe { crate::ALLOCATOR.set_active(true); }
        let records: Vec<Record> = serde_json::from_reader(f).unwrap();
        unsafe { crate::ALLOCATOR.set_active(false); }
        println!("Read {} records", records.len());
    }
}
