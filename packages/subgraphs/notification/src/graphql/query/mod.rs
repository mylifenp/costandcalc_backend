pub mod health;

use async_graphql::MergedObject;
use crate::graphql::query::health::NotificationHealthQuery;


#[derive(Default, MergedObject)]
pub struct Query(NotificationHealthQuery);