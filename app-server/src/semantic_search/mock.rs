use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;

use super::semantic_search_grpc::{
    index_request::Datapoint, CalculateSimilarityScoresResponse, CreateCollectionResponse,
    DeleteCollectionsResponse, DeleteEmbeddingsResponse, IndexResponse, QueryResponse,
};

use super::SemanticSearchTrait;

#[derive(Clone)]
pub struct MockSemanticSearch {}

#[async_trait]
impl SemanticSearchTrait for MockSemanticSearch {
    async fn query(
        &self,
        _: &str,
        _: String,
        _: u32,
        _: f32,
        _: Vec<HashMap<String, String>>,
    ) -> Result<QueryResponse> {
        Ok(QueryResponse::default())
    }

    async fn delete_embeddings(
        &self,
        _: &str,
        _: Vec<HashMap<String, String>>,
    ) -> Result<DeleteEmbeddingsResponse> {
        Ok(DeleteEmbeddingsResponse::default())
    }

    async fn index(&self, _: Vec<Datapoint>, _: String) -> Result<IndexResponse> {
        Ok(IndexResponse::default())
    }

    async fn create_collection(&self, _: String) -> Result<CreateCollectionResponse> {
        Ok(CreateCollectionResponse::default())
    }

    async fn delete_collections(&self, _: String) -> Result<DeleteCollectionsResponse> {
        Ok(DeleteCollectionsResponse::default())
    }

    async fn calculate_similarity_scores(
        &self,
        _: Vec<String>,
        _: Vec<String>,
    ) -> Result<CalculateSimilarityScoresResponse> {
        Ok(CalculateSimilarityScoresResponse::default())
    }
}
