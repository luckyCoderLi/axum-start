use crate::{
    Result,
    model::{ModelController, Ticket, TicketForCreate},
};
use axum::extract::{Path, State};
use axum::routing::{delete, get, post};
use axum::{Json, Router};

// #[derive(Clone,FromRef)]
// struct AppState {
//     mc: ModelController,
// }

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/tickets/create", post(api_create_ticket))
        .route("/tickets/list", get(api_list_tickets))
        .route("/tickets/delete/{id}", delete(api_delete_ticket))
        .with_state(mc)
}

// create
async fn api_create_ticket(
    State(mc): State<ModelController>,
    Json(payload): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - api_create_ticket", "HANDLER");
    let ticket = mc.create_ticket(payload).await?;
    Ok(Json(ticket))
}

// list
async fn api_list_tickets(State(mc): State<ModelController>) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<12} - api_list_tickets", "HANDLER");
    let tickets = mc.list_tickets().await?;
    Ok(Json(tickets))
}

// delete
async fn api_delete_ticket(
    State(mc): State<ModelController>,
    Path(id): Path<i64>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - api_delete_ticket", "HANDLER");
    let ticket = mc.delete_ticket(id).await?;
    Ok(Json(ticket))
}
