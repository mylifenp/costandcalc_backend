pub mod health;

use async_graphql::MergedObject;
use crate::graphql::query::health::BillingHealthQuery;


#[derive(Default, MergedObject)]
pub struct Query(BillingHealthQuery);