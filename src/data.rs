use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Launch {
    pub name: String,
    pub date_utc: String,
    pub success: Option<bool>,
    pub rocket: String,
    pub launchpad: String,
}

pub async fn fetch_spacex_launches() -> Result<Vec<Launch>, reqwest::Error> {
    let url = "https://api.spacexdata.com/v4/launches";
    let response = reqwest::get(url).await?;
    let launches: Vec<Launch> = response.json().await?;
    Ok(launches)
}
