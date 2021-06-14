#[tokio::main]
async fn main() {
    let url = String::from("https://www.immigration.govt.nz/about-us/covid-19/border-closures-and-exceptions/entry-to-new-zealand/critical-purpose-reasons-you-can-travel-to-new-zealand");

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on( nz::reap(&url));
    //let results = nz::health(&url).await?;
    //nz::reap(&url);
    //Ok(());
    
}
extern crate scraper;

mod nz {
    use scraper::{Html, Selector};
    #[tokio::main]
    pub async fn health(url: &str) -> Result<(), reqwest::Error> {
        let res = reqwest::get(url).await?;
        println!("Status: {}", res.status());
        println!("Headers: {:?}", res.headers());
        Ok(())
    }

    pub async fn reap(url: &str) {
        println!("Reaping...");
        let reg = String::from("inz_note inz_note_tip"); //inz_note inz_note_tip
                                                         // partners-and-dependent-children-of-temporary-visa-holders
        let resp = reqwest::get(url).await.unwrap();
        let body = resp.text().await.unwrap();
        let fragment = Html::parse_document(&body);
        let reasons = Selector::parse(&reg).unwrap();

        for reason in fragment.select(&reasons) {
            let reason_txt = reason.text().collect::<Vec<_>>();
            println!("{:?}", reason_txt);
        }
        //Ok(())
    }
}
