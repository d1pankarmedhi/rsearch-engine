mod bm25;
use bm25::BM25Retriever;

fn main() {
    // BM25 parameters (k1 and b)
    let k1 = 1.5;
    let b = 0.75;

    // Create a BM25Retriever instance
    let mut bm25_retriever = BM25Retriever::new(k1, b);

    // Sample documents
    let documents = vec![
        "This is the the first document.",
        "This document is the second document.",
        "And this is the third one.",
        "Is this the first document?",
    ];

    // Add documents to the BM25Retriever
    for document in &documents {
        bm25_retriever.add_document(document);
    }

    // Search query
    let query = "second third";
    let top_k = 2; // Specify the number of top relevant documents to retrieve

    // Perform BM25-based search on documents
    let relevant_documents = bm25_retriever.search(query, top_k);

    // Display the relevant documents
    for (document_idx, score) in relevant_documents {
        let document = &documents[document_idx.parse::<usize>().unwrap()];
        println!("Document: {}\nBM25 Score: {}\n", document, score);
    }
}
