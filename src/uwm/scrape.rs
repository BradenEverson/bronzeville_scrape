use std::{ops::Range, error::Error};
use rayon::prelude::*;

pub fn scrape_sites(id_range: Range<u32>) -> Result<(), Box<dyn Error>> {
    let mut iterated_file_download = id_range.into_par_iter()
        .map(|a| {
            let go_str = format!("https://collections.lib.uwm.edu/digital/download/collection/san/id/{}/size/full", a);
            let mut file = std::fs::File::create(format!("src/img/{}.png", a)).unwrap();
            let mut request = reqwest::blocking::get(go_str.clone()).unwrap();

            request.copy_to(&mut file);

            return file;
        });
    let mut res = iterated_file_download.collect::<Vec<_>>();
    for i in 0..res.len() {
        println!("{:?}", res[i]);
    }
    Ok(())
}
