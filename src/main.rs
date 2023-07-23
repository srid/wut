use std::path::Path;

use anyhow::Result;
use clap::Parser;
use indradb;

#[derive(Parser, Debug)]
#[clap(author = "Sridhar Ratnakumar", version, about)]
/// Application configuration
struct Args {
    /// whether to be verbose
    #[arg(short = 'v')]
    verbose: bool,

    #[arg(default_value = "/tmp/wutdb")]
    db_file: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("DEBUG {args:?}");

    let db: indradb::Database<indradb::MemoryDatastore> =
        if Path::new(args.db_file.as_str()).exists() {
            indradb::MemoryDatastore::read_msgpack_db(args.db_file)?
        } else {
            indradb::MemoryDatastore::create_msgpack_db(args.db_file)
        };

    // Create a couple of vertices
    let out_v = indradb::Vertex::new(indradb::Identifier::new("person")?);
    let in_v = indradb::Vertex::new(indradb::Identifier::new("movie")?);
    db.create_vertex(&out_v)?;
    db.create_vertex(&in_v)?;

    // Add an edge between the vertices
    let edge = indradb::Edge::new(out_v.id, indradb::Identifier::new("likes")?, in_v.id);
    db.create_edge(&edge)?;

    // Query for the edge
    let output: Vec<indradb::QueryOutputValue> =
        db.get(indradb::SpecificEdgeQuery::single(edge.clone()))?;
    // Convenience function to extract out the edges from the query results
    let e = indradb::util::extract_edges(output).unwrap();
    assert_eq!(e.len(), 1);
    assert_eq!(edge, e[0]);
    println!("edge: {:?}", e[0]);
    db.sync()?;
    Ok(())
}
