pub mod health;

use async_graphql::MergedObject;
use crate::graphql::query::health::MediaHealthQuery;


#[derive(Default, MergedObject)]
pub struct Query(MediaHealthQuery);