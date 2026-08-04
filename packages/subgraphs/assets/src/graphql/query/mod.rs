pub mod health;

use async_graphql::MergedObject;
use crate::graphql::query::health::AssetsHealthQuery;


#[derive(Default, MergedObject)]
pub struct Query(AssetsHealthQuery);