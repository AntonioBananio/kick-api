use crate::error::{KickApiError, Result};
use crate::models::{Livestream, LivestreamSort, LivestreamStats};
use reqwest;

/// Livestreams API - query live streams and global stats
pub struct LivestreamsApi<'a> {
    client: &'a reqwest::Client,
    token: &'a Option<String>,
    base_url: &'a str,
}

impl<'a> LivestreamsApi<'a> {
    pub(crate) fn new(
        client: &'a reqwest::Client,
        token: &'a Option<String>,
        base_url: &'a str,
    ) -> Self {
        Self {
            client,
            token,
            base_url,
        }
    }

    /// Get currently live streams
    ///
    /// Works with both User Access Tokens and App Access Tokens (no special scope required).
    ///
    /// # Parameters
    /// - `broadcaster_user_ids`: Filter by specific broadcaster IDs (max 50)
    /// - `category_id`: Filter by category
    /// - `language`: Filter by language code (e.g., "en")
    /// - `sort`: Sort order (defaults to `ViewerCount`)
    /// - `limit`: Number of results (1-100, defaults to 25)
    ///
    /// # Example
    /// ```no_run
    /// // Get top live streams
    /// let streams = client.livestreams().get(None, None, None, None, None).await?;
    ///
    /// // Get specific broadcasters
    /// let streams = client.livestreams().get(
    ///     Some(vec![12345, 67890]),
    ///     None, None, None, None,
    /// ).await?;
    /// ```
    pub async fn get(
        &self,
        broadcaster_user_ids: Option<Vec<u64>>,
        category_id: Option<u32>,
        language: Option<&str>,
        sort: Option<LivestreamSort>,
        limit: Option<u32>,
    ) -> Result<Vec<Livestream>> {
        super::require_token(self.token)?;

        let url = format!("{}/livestreams", self.base_url);
        let mut request = self
            .client
            .get(&url)
            .header("Accept", "*/*")
            .bearer_auth(self.token.as_ref().unwrap());

        if let Some(ids) = broadcaster_user_ids {
            for id in ids {
                request = request.query(&[("broadcaster_user_id", id)]);
            }
        }
        if let Some(cat) = category_id {
            request = request.query(&[("category_id", cat)]);
        }
        if let Some(lang) = language {
            request = request.query(&[("language", lang)]);
        }
        if let Some(s) = sort {
            request = request.query(&[("sort", s.as_str())]);
        }
        if let Some(l) = limit {
            request = request.query(&[("limit", l)]);
        }

        let response = crate::http::send_with_retry(self.client, request).await?;
        if response.status().is_success() {
            let body = response.text().await?;

            #[derive(serde::Deserialize)]
            struct LivestreamsResponse {
                data: Vec<Livestream>,
            }

            let resp: LivestreamsResponse = serde_json::from_str(&body)
                .map_err(|e| KickApiError::ApiError(format!("JSON parse error: {}", e)))?;

            Ok(resp.data)
        } else {
            Err(KickApiError::ApiError(format!(
                "Failed to get livestreams: {}",
                response.status()
            )))
        }
    }

    /// Get global livestream statistics
    ///
    /// Returns the total number of live streams on Kick.
    /// Works with both User Access Tokens and App Access Tokens.
    ///
    /// # Example
    /// ```no_run
    /// let stats = client.livestreams().stats().await?;
    /// println!("Total live streams: {}", stats.total_count);
    /// ```
    pub async fn stats(&self) -> Result<LivestreamStats> {
        super::require_token(self.token)?;

        let url = format!("{}/livestreams/stats", self.base_url);
        let request = self
            .client
            .get(&url)
            .header("Accept", "*/*")
            .bearer_auth(self.token.as_ref().unwrap());

        let response = crate::http::send_with_retry(self.client, request).await?;
        if response.status().is_success() {
            let body = response.text().await?;

            #[derive(serde::Deserialize)]
            struct StatsResponse {
                data: LivestreamStats,
            }

            let resp: StatsResponse = serde_json::from_str(&body)
                .map_err(|e| KickApiError::ApiError(format!("JSON parse error: {}", e)))?;

            Ok(resp.data)
        } else {
            Err(KickApiError::ApiError(format!(
                "Failed to get livestream stats: {}",
                response.status()
            )))
        }
    }
}
