#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct HttpHandler {
    client: reqwest::Client,
}

impl HttpHandler {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }
    pub async fn post<T, U>(&self, url: &str, request: &T) -> reqwest::Result<U>
    where
        T: serde::Serialize,
        U: serde::de::DeserializeOwned,
    {
        let response = self
            .client
            .post(url)
            .json(request)
            .send()
            .await?
            .error_for_status()?;

        response.json::<U>().await
    }
}

pub fn get_client() -> reqwest::Result<reqwest::Client> {
    reqwest::Client::builder().build()
}
