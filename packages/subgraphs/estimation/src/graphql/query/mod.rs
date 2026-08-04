pub mod health;

use async_graphql::MergedObject;
use crate::graphql::query::health::EstimationHealthQuery;


#[derive(Default, MergedObject)]
pub struct Query(EstimationHealthQuery);