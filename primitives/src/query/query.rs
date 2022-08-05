use {
    super::QueryRequest,
    crate::{types::BlockReference, utils::generate_random_string},
};

#[derive(Clone, Debug)]
pub struct Query {
    pub query_id: String,
    pub block_reference: BlockReference,
    pub request: QueryRequest,
}

impl Query {
    pub fn new(block_reference: BlockReference, request: QueryRequest) -> Self {
        Query {
            query_id: generate_random_string(10),
            block_reference,
            request,
        }
    }
}
