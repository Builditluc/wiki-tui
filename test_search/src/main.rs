use wiki_api::{search::{Search, SearchRequest}, languages::Language, Endpoint};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Testing wiki-api search functionality...");
    
    let endpoint = Endpoint::parse("https://en.wikipedia.org/w/api.php")?;
    let language = Language::English;
    
    println!("Searching for 'rust programming language'...");
    
    let search_request = Search::builder()
        .query("rust programming language")
        .endpoint(endpoint)
        .language(language)
        .limit(5);
    
    match search_request.search().await {
        Ok(search) => {
            println!("Search successful!");
            println!("Total hits: {:?}", search.info.total_hits);
            println!("Query: {}", search.info.query);
            println!("Language: {}", search.info.language.name());
            println!("Number of results: {}", search.results.len());
            
            for (i, result) in search.results.iter().enumerate() {
                println!("\nResult {}:", i + 1);
                println!("  Title: {}", result.title);
                println!("  Namespace: {:?}", result.namespace);
                println!("  Page ID: {}", result.pageid);
                if let Some(snippet) = &result.snippet {
                    println!("  Snippet: {}", result.cleaned_snippet());
                }
                if let Some(size) = result.size {
                    println!("  Size: {} bytes", size);
                }
            }
        }
        Err(e) => {
            println!("Search failed: {}", e);
            return Err(e);
        }
    }
    
    Ok(())
}