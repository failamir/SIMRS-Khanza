// BPJS Service placeholder
// Implementasi BPJS API (SEP, Rujukan, dll) akan ditambahkan di sini

pub struct BpjsService {
    base_url: String,
    consumer_id: String,
    consumer_secret: String,
}

impl BpjsService {
    pub fn new(base_url: &str, consumer_id: &str, consumer_secret: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            consumer_id: consumer_id.to_string(),
            consumer_secret: consumer_secret.to_string(),
        }
    }

    // TODO: Implement BPJS API methods
}