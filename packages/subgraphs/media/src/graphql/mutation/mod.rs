use async_graphql::MergedObject;

use crate::graphql::mutation::file::FileMutation;

pub mod file;

#[derive(Default, MergedObject)]
pub struct Mutation(FileMutation);
