use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Default)]
pub struct RerankRequest<'input> {
    /// The search query.
    pub query: &'input str,
    /// A list of document strings to rerank.
    pub documents: &'input [String],
    /// The model to use.
    pub model: RerankModel,
    /// The number of results to return, defaults to the length of the documents.
    pub top_n: Option<u64>,
    // The maximum number of chunks to derive from each document.
    pub max_chunks_per_doc: Option<u64>,
}

#[derive(strum_macros::Display, Serialize, Debug, Default)]
pub enum RerankModel {
    /// A model that allows for re-ranking English Language documents and semi-structured data (JSON).
    /// This model has a context length of 4096 tokens.
    #[strum(serialize = "rerank-english-v3.0")]
    #[serde(rename = "rerank-english-v3.0")]
    EnglishV3,
    /// A model for documents and semi-structured data (JSON) that are not in English.
    /// Supports the same languages as embed-multilingual-v3.0. This model has a context length of 4096 tokens.
    #[strum(serialize = "rerank-multilingual-v3.0")]
    #[serde(rename = "rerank-multilingual-v3.0")]
    MultilingualV3,
    /// A model that allows for re-ranking English language documents.
    /// This model has a context length of 512 tokens.
    #[strum(serialize = "rerank-english-v2.0")]
    #[serde(rename = "rerank-english-v2.0")]
    #[default]
    EnglishV2,
    /// A model for documents that are not in English.
    /// Supports the same languages as embed-multilingual-v3.0. This model has a context length of 512 tokens.
    #[strum(serialize = "rerank-multilingual-v2.0")]
    #[serde(rename = "rerank-multilingual-v2.0")]
    MultilingualV2,
    /// Custom model
    Custom(String),
}

#[derive(Deserialize, Debug)]
pub(crate) struct RerankResponse {
    /// List of ranked documents
    pub results: Vec<RerankResult>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct RerankResult {
    /// The index of the input document
    pub index: u64,
    /// A relevance score assigned to the ranking
    pub relevance_score: f64,
}
