use std::{ops::Range, error::Error};

pub fn scrape_sites(id_range: Range<u32>) -> Result<(), Box<dyn Error>> {
    for idx in id_range {
        let go_str = format!("https://collections.lib.uwm.edu/digital/download/collection/san/id/{}/size/full", idx);
        let mut file = std::fs::File::create(format!("src/img/{}.png", idx)).unwrap();
        let mut request = reqwest::blocking::get(go_str.clone()).unwrap();

        if request.text().unwrap().contains("html") {
            println!("Html :(");
        } else{
            println!("Beginning download {}", idx);
            request = reqwest::blocking::get(go_str).unwrap();
            request.copy_to(&mut file).unwrap();
        }
    }
    Ok(())
}
