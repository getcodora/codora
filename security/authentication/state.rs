use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    redirect_url: String,
}
