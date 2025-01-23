#[cfg(test)]
mod tests {
    use crate::api::get_latest_quotes;
    use mockito;
    use mockito::Server;
    use serde_json::json;

    #[tokio::test]
    async fn test_get_latest_quotes_success() {
        let mut server = Server::new_async().await;

        let mock_server = server
            .mock("GET", "/v1/cryptocurrency/quotes/latest")
            .match_query(mockito::Matcher::UrlEncoded("convert".into(), "USD".into()))
            .match_query(mockito::Matcher::UrlEncoded(
                "symbol".into(),
                "BTC,ETH".into(),
            ))
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
                    "data": {
                        "BTC": {
                            "id": 1,
                            "name": "Bitcoin",
                            "symbol": "BTC",
                            "slug": "bitcoin",
                            "is_active": 1,
                            "is_fiat": null,
                            "circulating_supply": 19310906,
                            "total_supply": 19310906,
                            "max_supply": 21000000,
                            "date_added": "2013-04-28T00:00:00.000Z",
                            "num_market_pairs": 10107,
                            "cmc_rank": 1,
                            "last_updated": "2023-01-23T09:52:00.000Z",
                            "tags": [
                                "mineable",
                                "pow",
                                "sha-256",
                                "store-of-value",
                                "state-channels",
                                "coinbase-ventures-portfolio",
                                "three-arrows-capital-portfolio",
                                "polychain-capital-portfolio",
                                "binance-labs-portfolio",
                                "blockchain-capital-portfolio",
                                "boostvc-portfolio",
                                "cms-holdings-portfolio",
                                "dcg-portfolio",
                                "dragonfly-capital-portfolio",
                                "electric-capital-portfolio",
                                "fabric-ventures-portfolio",
                                "framework-ventures-portfolio",
                                "galaxy-digital-portfolio",
                                "huobi-capital-portfolio",
                                "alameda-research-portfolio",
                                "a16z-portfolio",
                                "1confirmation-portfolio",
                                "winklevoss-capital-portfolio",
                                "usv-portfolio",
                                "placeholder-ventures-portfolio",
                                "pantera-capital-portfolio",
                                "multicoin-capital-portfolio",
                                "paradigm-portfolio"
                            ],
                            "platform": null,
                            "self_reported_circulating_supply": null,
                            "self_reported_market_cap": null,
                            "quote": {
                                "USD": {
                                    "price": 22984.17476055732,
                                    "volume_24h": 23481937739.27893,
                                    "volume_change_24h": -19.4461,
                                    "percent_change_1h": -0.09774376,
                                    "percent_change_24h": 0.49182722,
                                    "percent_change_7d": 10.14722419,
                                    "percent_change_30d": 39.19000321,
                                    "market_cap": 445218773946.8293,
                                    "market_cap_dominance": 42.3839,
                                    "fully_diluted_market_cap": 48266767,
                                    "last_updated": "2023-01-23T09:52:00.000Z"
                                }
                            }
                        },
                        "ETH": {
                            "id": 1027,
                            "name": "Ethereum",
                            "symbol": "ETH",
                            "slug": "ethereum",
                            "is_active": 1,
                            "is_fiat": null,
                            "circulating_supply": 122373866.219,
                            "total_supply": 122373866.219,
                            "max_supply": null,
                            "date_added": "2015-08-07T00:00:00.000Z",
                            "num_market_pairs": 6329,
                            "cmc_rank": 2,
                            "last_updated": "2023-01-23T09:52:00.000Z",
                            "tags": [
                                "mineable",
                                "pow",
                                "smart-contracts",
                                "ethereum-ecosystem",
                                "coinbase-ventures-portfolio",
                                "three-arrows-capital-portfolio",
                                "polychain-capital-portfolio",
                                "binance-labs-portfolio",
                                "blockchain-capital-portfolio",
                                "boostvc-portfolio",
                                "cms-holdings-portfolio",
                                "dcg-portfolio",
                                "dragonfly-capital-portfolio",
                                "electric-capital-portfolio",
                                "fabric-ventures-portfolio",
                                "framework-ventures-portfolio",
                                "hashkey-capital-portfolio",
                                "kenetic-capital-portfolio",
                                "huobi-capital-portfolio",
                                "alameda-research-portfolio",
                                "a16z-portfolio",
                                "1confirmation-portfolio",
                                "winklevoss-capital-portfolio",
                                "usv-portfolio",
                                "placeholder-ventures-portfolio",
                                "pantera-capital-portfolio",
                                "multicoin-capital-portfolio",
                                "paradigm-portfolio"
                            ],
                            "platform": null,
                            "self_reported_circulating_supply": null,
                            "self_reported_market_cap": null,
                            "quote": {
                                "USD": {
                                    "price": 1634.2580643568523,
                                    "volume_24h": 8311937726.710532,
                                    "volume_change_24h": -24.9079,
                                    "percent_change_1h": -0.15884522,
                                    "percent_change_24h": 0.98925777,
                                    "percent_change_7d": 7.43766666,
                                    "percent_change_30d": 32.69572748,
                                    "market_cap": 200495789576.48135,
                                    "market_cap_dominance": 19.0833,
                                    "fully_diluted_market_cap": 200495789576.48135,
                                    "last_updated": "2023-01-23T09:52:00.000Z"
                                }
                            }
                        }
                    }
                })
                .to_string(),
            )
            .create();

        // Use the mock server's URL for testing
        let base_url = server.url();

        // Call the function with a test API key and base URL
        let result = get_latest_quotes("test_api_key", "BTC,ETH", &base_url).await;

        // Assert the result
        if let Err(e) = &result {
            eprintln!("Error: {:?}", e); // Print the error to stderr
        }
        assert!(result.is_ok());
        let api_response = result.unwrap();

        // Add more assertions based on the expected data
        assert_eq!(api_response.data.currencies.len(), 2);
        assert!(api_response.data.currencies.contains_key("BTC"));
        assert!(api_response.data.currencies.contains_key("ETH"));

        // Verify that the mock was called
        mock_server.assert();
    }
}
