use axum::extract::{Path, State};
use axum::routing::{delete, post};
use axum::{Json, Router};

use crate::model::{ModelController, Ticket, TicketsForCreate};
use crate::Result;

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_tickets))
        .with_state(mc)
}

async fn create_ticket(
    State(mc): State<ModelController>,
    Json(ticket): Json<TicketsForCreate>,
) -> Result<Json<Ticket>> {
    println!("--> {:<12} - create_ticket", "HANDLER");

    let ticket = mc.create(ticket).await?;

    Ok(Json(ticket))
}

async fn list_tickets(State(mc): State<ModelController>) -> Result<Json<Vec<Ticket>>> {
    println!("--> {:<12} - create_list", "HANDLER");

    let ticket = mc.list().await?;

    Ok(Json(ticket))
}

async fn delete_tickets(
    State(mc): State<ModelController>,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    println!("--> {:<12} - create_list", "HANDLER");

    let ticket = mc.delete(id).await?;

    Ok(Json(ticket))
}
