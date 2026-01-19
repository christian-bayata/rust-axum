use crate::{Error, Result, ctx::Ctx};
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Serialize)]
// model
pub struct Ticket {
    pub id: u64,
    pub title: String,
    pub creator_id: String
}

// dto
#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String
}

#[derive(Clone)]
// This is a shared in-memory data store.
pub struct ModelController {
    ticket_stores: Arc<Mutex<Vec<Option<Ticket>>>>
}

// Constructor
impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            ticket_stores: Arc::default()
        })
    }
}

// CRUD Implementation 
impl ModelController {
    /* Create a ticket */
    pub async fn create_ticket(&self, ctx: Ctx, tc: TicketForCreate) -> Result<Ticket> {
        let mut store = self.ticket_stores.lock().unwrap();

        let id = store.len() as u64;

        let ticket: Ticket = Ticket {
            id, 
            creator_id: ctx.user_id().to_string(),
            title: tc.title
        }; 

        store.push(Some(ticket.clone()));

        Ok(ticket)
    }

    /* Retrieve all tickets */
    pub async fn list_tickets(&self, _ctx: Ctx) -> Result<Vec<Ticket>> {
        let store = self.ticket_stores.lock().unwrap();

        let tickets = store.iter().filter_map(|t| t.clone()).collect();

        Ok(tickets)
    }

    /* Delete a ticket */
    pub async fn delete_ticket(&self,_ctx: Ctx, id: u64) -> Result<Ticket> {
        let mut store = self.ticket_stores.lock().unwrap();
  
        let ticket = store.get_mut(id as usize).and_then(|t| t.take());

        ticket.ok_or(Error::TicketDeleteFailIdNotFound { id })
    }
}