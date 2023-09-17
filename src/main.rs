mod bm25;
use bm25::BM25Retriever;

use serde::{Serialize, Deserialize};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::fs::File;
use std::io::{BufRead, BufReader, Result};


#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    document: String,
    score: f64,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    documents: Vec<Document>,
}


#[derive(Deserialize)]
struct Search{
    query: String,
}


// load file content
fn etl_txt_file(file_path: &str) -> Result<Vec<String>>{
    let mut data = Vec::new();
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();        
        data.push(line);
    }
    Ok(data)
}


// search on local file
fn file_search(documents: &Vec<String>, query: &str, top_k: usize) -> Vec<(std::string::String, f64)>{
    // BM25 parameters (k1 and b)
    let k1 = 1.5;
    let b = 0.75;

    // Create a BM25Retriever instance
    let mut bm25_retriever = BM25Retriever::new(k1, b);
    // Add documents to the BM25Retriever
    for document in documents {
        bm25_retriever.add_document(&document);
    }
    let relevant_documents = bm25_retriever.search(query, top_k);

    return relevant_documents;
}



// search route
#[post("/search")]
async fn search(search: web::Query<Search>) -> Result<HttpResponse> {
    // let query = "BFS DFS";
    let top_k = 3;
    let query = search.query.clone();
    let documents = etl_txt_file("search.txt").unwrap();
    // println!("Documents ------ \n{:?}", &documents);
    let relevant_documents = file_search(&documents, &query, top_k);

    // for (document_idx, score) in &relevant_documents {
    //     let document = &documents[document_idx.parse::<usize>().unwrap()];
    //     println!("Document: {}\nBM25 Score: {}\n", document, score);
    // }

    let output: Vec<Document>= relevant_documents.into_iter().map(|(i, score)| Document{
        document: documents[i.parse::<usize>().unwrap()].clone(),
        score: score,
    }).collect();
    let response = ApiResponse{
        documents: output,
    };
    let json_response = serde_json::to_string(&response).unwrap();

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(json_response))
}




#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        println!("Server starting !");
        App::new()
        .service(search)
    })
    
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
