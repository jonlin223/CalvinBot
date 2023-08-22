use reqwest::Error;

pub async fn scrape(date: &str) -> Result<String, Error>{

    let url = "https://www.gocomics.com/calvinandhobbes/".to_owned() + date;
    println!("{url}");
    let res = reqwest::get(url).await?.text().await?;

    Ok(res)
}