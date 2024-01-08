use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// Tiekets
#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketsForCreate {
    pub title: String,
}

// Controler
#[derive(Clone)]
pub struct ModelController {
    ticket_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController {
    pub async fn new() -> Result<ModelController> {
        Ok(ModelController {
            ticket_store: Arc::default(),
        })
    }
}

impl ModelController {
    pub async fn create(&self, ticket: TicketsForCreate) -> Result<Ticket> {
        let mut store = self.ticket_store.lock().unwrap();
        let id = store.len() as u64;

        let ticket = Ticket {
            id: id,
            title: ticket.title,
        };

        store.push(Some(ticket.clone()));

        Ok(ticket)
    }

    pub async fn list(&self) -> Result<Vec<Ticket>> {
        let store = self.ticket_store.lock().unwrap();

        let tickets = store.iter().filter_map(|t| t.clone()).collect();

        Ok(tickets)
    }

    pub async fn delete(&self, id: u64) -> Result<Ticket> {
        let mut store = self.ticket_store.lock().unwrap();

        let ticket = store.get_mut(id as usize).and_then(|t| t.take());

        ticket.ok_or(Error::TicketDeleteFaildNotFound { id: id })
    }
}
