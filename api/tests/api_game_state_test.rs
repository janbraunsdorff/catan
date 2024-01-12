#[cfg(test)]
mod test {
    use anyhow::{Ok, Result};
    use api::routes::state::StateResponse;
    use http_body_util::BodyExt; // for `collect`

    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;


    #[tokio::test]
    async fn test_get_game_state_empty_game() -> Result<()> {
        let path = "/game/002_fill_board_2/state";
        let app = api::create_main_rounter();

        let response = app
            .oneshot(Request::builder().uri(path).body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(StatusCode::OK, response.status());

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: StateResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.tiles.len(), 4);

        Ok(())
    }
}
