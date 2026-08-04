pub mod health;

use async_graphql::MergedObject;
use crate::graphql::query::health::AdminHealthQuery;


#[derive(Default, MergedObject)]
pub struct Query(AdminHealthQuery);