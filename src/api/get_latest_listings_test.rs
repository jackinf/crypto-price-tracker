#[cfg(test)]
mod tests {
    use crate::api::get_latest_listings;
    use mockito;
    use mockito::Server;
    use serde_json::json;

    #[tokio::test]
    async fn test_get_latest_listings_success() {
        let mut server = Server::new_async().await;

        let mock_server = server
            .mock("GET", "/v1/cryptocurrency/listings/latest")
            .match_query(mockito::Matcher::UrlEncoded("start".into(), "1".into()))
            .match_query(mockito::Matcher::UrlEncoded("limit".into(), "5000".into()))
            .match_query(mockito::Matcher::UrlEncoded("convert".into(), "USD".into()))
            .match_header("X-CMC_PRO_API_KEY", "test_api_key")
            .match_header("Accept", "application/json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "status": {
                        "timestamp": "2023-01-23T00:00:00.000Z",
                        "error_code": 0,
                        "error_message": null,
                        "elapsed": 10,
                        "credit_count": 1,
                        "notice": null
                    },
                    "data": [
                        {
                            "name": "Bitcoin",
                            "symbol": "BTC",
                            "quote": {
                                "USD": {
                                    "price": 23000.0
                                }
                            }
                        },
                        {
                            "name": "Ethereum",
                            "symbol": "ETH",
                            "quote": {
                                "USD": {
                                    "price": 1600.0
                                }
                            }
                        }
                    ]
                })
                .to_string(),
            )
            .create();

        // Use the mock server's URL for testing
        let base_url = server.url();

        // Call the function with a test API key and base URL
        let result = get_latest_listings("test_api_key", &base_url).await;

        // Assert the result
        assert!(result.is_ok());
        let api_response = result.unwrap();

        // Add more assertions based on the expected data
        assert_eq!(api_response.data.len(), 2);
        assert_eq!(api_response.data[0].name, "Bitcoin");
        assert_eq!(api_response.data[0].symbol, "BTC");
        assert_eq!(api_response.data[0].quote.usd.price, 23000.0);
        assert_eq!(api_response.data[1].name, "Ethereum");

        // Verify that the mock was called
        mock_server.assert();
    }
}
