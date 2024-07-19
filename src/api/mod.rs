use serde::Serialize;

pub mod chat;
pub mod classify;
pub mod detect_language;
pub mod detokenize;
pub mod embed;
pub mod generate;
pub mod rerank;
pub mod summarize;
pub mod tokenize;

#[derive(strum_macros::Display, Serialize, Debug)]
pub enum Truncate {
    #[strum(serialize = "NONE")]
    #[serde(rename = "NONE")]
    None,
    #[strum(serialize = "START")]
    #[serde(rename = "START")]
    Start,
    #[strum(serialize = "END")]
    #[serde(rename = "END")]
    End,
}

#[derive(strum_macros::Display, Serialize, Debug)]
pub enum EmbedModel {
    /// A model that allows for text to be classified or turned into embeddings. English only.
    /// Embedding vector of size 1024.
    /// Context length: 512
    /// Similarity: Cosine Similarity
    #[strum(serialize = "embed-english-v3.0")]
    #[serde(rename = "embed-english-v3.0")]
    EnglishV3,
    /// A smaller, faster version of embed-english-v3.0. Almost as capable, but a lot faster.
    /// English only with an embedding vector of size 384.
    /// Context length: 512
    /// Similarity: Cosine Similarity
    #[strum(serialize = "embed-english-light-v3.0")]
    #[serde(rename = "embed-english-light-v3.0")]
    EnglishLightV3,
    /// Provides multilingual classification and embedding support with an embedding vector of size 1024.
    /// Context length: 512
    /// Similarity: Cosine Similarity
    #[strum(serialize = "embed-multilingual-v3.0")]
    #[serde(rename = "embed-multilingual-v3.0")]
    MultilingualV3,
    /// A smaller, faster version of embed-multilingual-v3.0. Almost as capable, but a lot faster.
    /// Supports multiple languages with an embedding vector of size 384.
    /// Context length: 512
    /// Similarity: Cosine Similarity
    #[strum(serialize = "embed-multilingual-light-v3.0")]
    #[serde(rename = "embed-multilingual-light-v3.0")]
    MultilingualLightV3,
    /// Older embeddings model that allows for text to be classified or turned into embeddings.
    /// English only with an embedding vector of size 4096.
    /// Context length: 512
    /// Similarity: Cosine Similarity
    #[strum(serialize = "embed-english-v2.0")]
    #[serde(rename = "embed-english-v2.0")]
    EnglishV2,
    /// A smaller, faster version of embed-english-v2.0. Almost as capable, but a lot faster.
    /// English only with an embedding vector of size 1024.
    /// Context length: 512
    /// Similarity: Cosine Similarity
    #[strum(serialize = "embed-english-light-v2.0")]
    #[serde(rename = "embed-english-light-v2.0")]
    EnglishLightV2,
    /// Provides multilingual classification and embedding support with an embedding vector of size 768.
    /// Context length: 256
    /// Similarity: Dot Product Similarity
    #[strum(serialize = "embed-multilingual-v2.0")]
    #[serde(rename = "embed-multilingual-v2.0")]
    MultilingualV2,
    /// Custom model
    Custom(String),
}

#[derive(strum_macros::Display, Serialize, Debug, Default)]
pub enum GenerateModel {
    /// Command R+ is an instruction-following conversational model that performs language tasks at a higher quality, more reliably, and with a longer context than previous models.
    /// It is best suited for complex RAG workflows and multi-step tool use.
    /// Context length: 128k
    /// Maximum output tokens: 4k
    #[strum(serialize = "command-r-plus")]
    #[serde(rename = "command-r-plus")]
    #[default]
    CommandRPlus,
    /// Command R is an instruction-following conversational model that performs language tasks at a higher quality, more reliably, and with a longer context than previous models.
    /// It can be used for complex workflows like code generation, retrieval augmented generation (RAG), tool use, and agents.
    /// Context length: 128k
    /// Maximum output tokens: 4k
    #[strum(serialize = "command-r")]
    #[serde(rename = "command-r")]
    CommandR,
    /// An instruction-following conversational model that performs language tasks with high quality, more reliably, and with a longer context than our base generative models.
    /// Context length: 4k
    /// Maximum output tokens: 4k
    #[strum(serialize = "command")]
    #[serde(rename = "command")]
    Command,
    /// To reduce the time between major releases, we put out nightly versions of command models. For command, that is command-nightly.
    /// Be advised that command-nightly is the latest, most experimental, and (possibly) unstable version of its default counterpart. Nightly releases are updated regularly, without warning, and are not recommended for production use.
    /// Context length: 128k
    /// Maximum output tokens: 128k
    #[strum(serialize = "command-nightly")]
    #[serde(rename = "command-nightly")]
    CommandNightly,
    /// A smaller, faster version of command. Almost as capable, but a lot faster.
    /// Context length: 4k
    /// Maximum output tokens: 4k
    #[strum(serialize = "command-light")]
    #[serde(rename = "command-light")]
    CommandLight,
    /// To reduce the time between major releases, we put out nightly versions of command models. For command-light, that is command-light-nightly.
    /// Be advised that command-light-nightly is the latest, most experimental, and (possibly) unstable version of its default counterpart. Nightly releases are updated regularly, without warning, and are not recommended for production use.
    /// Context length: 4k
    /// Maximum output tokens: 4k
    #[strum(serialize = "command-light-nightly")]
    #[serde(rename = "command-light-nightly")]
    CommandLightNightly,
    /// Custom model
    Custom(String),
}
