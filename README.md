
# Okapi BM25 search 

A rust implementation of the BM25 search algorithm for document search, backed by an actix-web server.

> BM25 is a bag-of-words retrieval function that ranks a set of documents based on the query terms appearing in each document, regardless of their proximity within the document - Wikipedia

![okapi-bm25](https://wikimedia.org/api/rest_v1/media/math/render/svg/8624885ce5cd14936807927801f6d29c315d3828)

## Project set-up

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites
Make sure you have `rust` installed. If not, check-out the official rust [documentation](https://www.rust-lang.org/tools/install).

### Installing

A step by step series of examples that tell you how to get a development env running

1. Clone the repo and `cd` into it.

```bash
$ git clone https://github.com/d1pankarmedhi/search_bm25.git
$ cd search_bm25
```

2. Build the project to add the dependencies and build the project.
```
$ cargo build
```

3. Run the project 

```bash
$ cargo run
```

4. The server should be running on `localhost:8080`. Make a POST request and check if there's any response using curl.
If yes, then the project is set-up successfully.
   
```bash 
$ curl -X POST http://localhost:8080/search?query=breadth
```
```bash
# Response
{
  "documents": [
    {
      "document": "Document 5: Breadth-First Search",
      "score": 6.627193106486171
    },
    {
      "document": "A* search is a popular heuristic search algorithm that combines elements of breadth-first search and best-first search. It uses a heuristic function to estimate the cost of reaching the goal from a particular node. A* search intelligently explores the most promising paths first, improving efficiency in finding optimal solutions.",
      "score": 3.558679160608629
    },
    {
      "document": "Breadth-first search (BFS) is another graph traversal algorithm that explores all the vertices of a graph level by level. It starts at a specific vertex and visits all its neighbors before moving to the next level of vertices. BFS is commonly used to find the shortest path between two vertices or to solve puzzles with minimum moves.",
      "score": 3.2969824201070472
    }
  ]
}
```


## Built With

* [Rust](https://www.rust-lang.org/) programming language
* [Actix-web](https://actix.rs/) - a web framework for rust


## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details
