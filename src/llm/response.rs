/// LLM 返回的响应。
pub struct LlmResponse {
    pub commit_message: String,
    pub usage_tokens: Option<usize>,
}
