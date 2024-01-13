#[cfg(test)]
mod test {
    use std::fs;
    
    use anyhow::{Ok, Result};
    use api::routes::create::{CreateNewGameRequest, PlayerRequest };
    use http_body_util::BodyExt; // for `collect`

    use axum::{
        body::Body,
        http::{self, request, Request, StatusCode},
    };
    use serde_json::json;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_create_new_game() -> Result<()> {
        let create_event = json!(CreateNewGameRequest {
            npc: vec![
                PlayerRequest {
                    name: "npc1".to_string(),
                    color: "RED".to_string()
                },
                PlayerRequest {
                    name: "npc2".to_string(),
                    color: "BLUE".to_string()
                },
            ],
            player: vec![
                PlayerRequest {
                    name: "pc1".to_string(),
                    color: "WHITE".to_string()
                },
                PlayerRequest {
                    name: "pc2".to_string(),
                    color: "ORANGE".to_string()
                },
            ],
            extentiosns: vec![]
        }).to_string();

        let path = "/game/test_new_game/create";
        let app = api::create_main_rounter();


        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(create_event)
                    .unwrap(),
            )
            .await
            .unwrap();
        fs::remove_file("../.storage/test_new_game.jsonl").unwrap();

        assert_eq!(response.status(), 200);


        Ok(())
    }
}
