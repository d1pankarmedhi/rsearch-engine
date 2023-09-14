extern crate rust_stemmers;
use rust_stemmers::{Stemmer, Algorithm};
use std::collections::{HashMap, HashSet};
use std::f64;

// BM25Retriever struct
pub struct BM25Retriever {
    k1: f64,
    b: f64,
    document_term_freq_collection: Vec<HashMap<String, usize>>,
    document_lengths: Vec<usize>,
    term_document_frequencies: HashMap<String, usize>,
}

// methods
impl BM25Retriever {
    pub fn new(k1: f64, b: f64) -> Self {
        BM25Retriever {
            k1,
            b,
            document_term_freq_collection: Vec::new(),
            document_lengths: Vec::new(),
            term_document_frequencies: HashMap::new(),
        }
    }

    // Tokenization and stemming of document
    fn tokenize_and_stem(document: &str) -> HashSet<String> {
        let stemmer = Stemmer::create(Algorithm::English);
        let tokens= document
            .to_lowercase()
            .split_whitespace()
            .map(|word| stemmer.stem(&word).to_string())
            .collect();
        return tokens;
    }

    // adding document for search
    pub fn add_document(&mut self, document: &str) {
        let tokens = BM25Retriever::tokenize_and_stem(document); // get tokens
        let mut term_frequency: HashMap<String, usize> = HashMap::new(); 

        for token in &tokens {
            *term_frequency.entry(token.to_string()).or_insert(0) += 1;  // term counts in a document
            *self.term_document_frequencies.entry(token.to_string()).or_insert(0) += 1; // document counts with the term
        } 

        let document_length = tokens.len();
        self.document_lengths.push(document_length);  // push the token length 
        self.document_term_freq_collection.push(term_frequency.clone());  // arr of term freq in each document

    }

    pub fn search(&self, query: &str, top_k: usize) -> Vec<(String, f64)> {
        let mut query_scores: Vec<(String, f64)> = Vec::new();
        let average_document_length: f64 = self.document_lengths.iter().sum::<usize>() as f64 / self.document_lengths.len() as f64; // avg length of doc
        let total_documents = self.document_lengths.len() as f64; // total number of documents

        // Tokenize and stem the query
        let query_tokens = BM25Retriever::tokenize_and_stem(query); // tokenize the query

        for (i, document_term_frequency) in self.document_term_freq_collection.iter().enumerate() {
            let mut score = 0.0;

            for query_token in &query_tokens {

                if let (Some(&tf), Some(&df)) = ( self.term_document_frequencies.get(query_token), document_term_frequency.get(query_token),
                ) {
                    let idf = (((total_documents - df as f64 + 0.5) / (df as f64 + 0.5)) + 1.0).ln();
                    let tf = tf as f64;
                    let document_length = self.document_lengths[i] as f64;
                    let numerator = (self.k1 + 1.0) * tf;
                    let denominator = self.k1 * (1.0 - self.b + self.b * (document_length / average_document_length)) + tf;
                    score += idf * (numerator / denominator);
                }
                
            }

            query_scores.push((i.to_string(), score));
        }
        // Sort documents by query scores in descending order
        query_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        println!("Query scores: {:?}", query_scores);

        // Return the top-k relevant documents
        query_scores.into_iter().take(top_k).collect()
    }
}
