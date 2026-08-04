pub mod health;

use async_graphql::MergedObject;
use crate::graphql::query::health::WorkHealthQuery;


#[derive(Default, MergedObject)]
pub struct Query(WorkHealthQuery);