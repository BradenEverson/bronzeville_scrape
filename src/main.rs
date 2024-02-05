use bronzeville_scrape::uwm::scrape::scrape_sites;

fn main() {
    scrape_sites(0..1000).unwrap();
}
