use axum::{extract::Path, response::IntoResponse, http::StatusCode, Json};


use crate::error::ExternalExecutionError;

use super::model::game::StateResponse;


pub async fn state(Path(id): Path<String>)  ->  Result<impl IntoResponse, ExternalExecutionError> {
    let res  = catan_core::load(id.as_str(), -1);
    let game = match res {
        Ok(val) => val,
        Err(err) => return Err(ExternalExecutionError{ step: err.step, message: err.message })
    };
    Ok((StatusCode::OK, Json(StateResponse::from(game))))
}


#[cfg(test)]
mod test {
    use crate::api::{self, model::game::StateResponse};
    use anyhow::{Result, Ok};
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


        let response = app.oneshot(
            Request::builder().uri(path).body(Body::empty()).unwrap()
        ).await
        .unwrap();

        assert_eq!(StatusCode::OK, response.status());

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: StateResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.tiles.len(), 4);

        Ok(())
    }
}