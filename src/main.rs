use bronzeville_scrape::uwm::scrape::scrape_sites;
use std::{thread, time};

fn main() {
    for i in 0..100 {
        let range = (i*10)..(i+1)*10;
        scrape_sites(range).unwrap();
        let time = time::Duration::from_millis(5000);
        thread::sleep(time);
    }
}
