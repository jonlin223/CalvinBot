use scraper::{Html, Selector};

/// Get date string corresponding to today's comic
pub async fn get_today() -> Option<String> {
    // Get HTML as string
    let url = "https://www.gocomics.com/calvinandhobbes/";
    let res = reqwest::get(url).await.ok()?.text().await.ok()?;
    let doc = Html::parse_document(&res);

    // Parse for href to today's comic
    let selector = Selector::parse(r#"a[data-link="comics"]"#).ok()?;
    let href = doc
        .select(&selector)
        .map(|x| x.value().attr("href"))
        .next()??;

    // Get date
    let date = href.strip_prefix("/calvinandhobbes/")?;

    Some(date.to_owned())
}

pub async fn scrape(url: &str) -> Option<String> {
    // Get HTML in String
    let res = reqwest::get(url).await.ok()?.text().await.ok()?;

    let doc = Html::parse_document(&res);
    let selector = Selector::parse("picture.item-comic-image>img").ok()?;
    let img = doc
        .select(&selector)
        .map(|x| x.value().attr("src"))
        .next()??;

    Some(img.to_owned())
}
