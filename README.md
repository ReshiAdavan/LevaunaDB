# LevaunaDB

An Extensible In-Memory Graph Database implemented in Rust that leverages a Gremlin inspired traversal language.

### Inspiration

Honestly, I thought it was so cool that non-relational database architectures could use graphs and some way to traverse them (aka a traversal query language). This whole time I believed non-relational architectures limited to document data stores (JSON), columnar data stores and key-value stores, which essentially leverage hashmaps/hashtables.

I saw DBs like Neo4j, Dgraph, and Kibana and I just had to see whats up.

I'll probably try to build a relational database via a B+ tree at some point and probably in a language I never used before as well.

### Sample

```
let mut graph = Graph::new();

let v1 = graph.add_vertex(hashmap!{
    "name".into() => Value::String("foo".into()),
    "type".into() => Value::String("banana".into())
}).unwrap();

let v2 = graph.add_vertex(hashmap!{
    "name".into() => Value::String("bar".into()),
    "type".into() => Value::String("orange".into())
}).unwrap();

graph.add_edge(v1, v2, "fruitier".into(), hashmap!{});

let mut q = Query::new(&graph, VertexFilter::Id(v1));
let out = q.out(EdgeFilter::None).run();

assert_eq!(out, vec![QueryResult::Vertex(v2)]);
```

I included testing files for the DB in `test/test_asgard.rs`.

### Dependencies

- maplit = "0.1.4"

### For Your Info

Project still in progress - Functionality, formatting, and other changes TBD.
