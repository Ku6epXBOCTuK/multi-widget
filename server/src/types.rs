use serde::{Deserialize, Serialize};
use shared::{Activity, ActivityId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SseEvent {
    CreateActivity(Activity),
    UpdateActivity(Activity),
    DeleteActivity(ActivityId),
    // TODO: do i need lagged???
    Lagged,
}
