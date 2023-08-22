use reqwest::Error;

pub async fn scrape(date: &str) -> Result<String, Error>{

    // Get HTML in String
    let url = "https://www.gocomics.com/calvinandhobbes/".to_owned() + date;
    let res = reqwest::get(url).await?.text().await?;

    let doc = scraper::Html::parse_document(&res);
    let selector = scraper::Selector::parse("picture.item-comic-image>img").unwrap();
    let img = doc.select(&selector).map(|x| x.value().attr("src")).next().unwrap().unwrap();

    Ok(img.to_owned())
}