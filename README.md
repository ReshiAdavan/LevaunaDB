# LevaunaDB

An Extensible In-Memory Graph Database implemented in Rust that leverages a Gremlin inspired traversal language.

### Inspiration

I found it so interesting that some non-relational database architectures use graphs to draw relationships between data. I thought architectures limited to document data stores (JSON), columnar data stores and key-value stores, all of which essentially leverage hashmaps/hashtables.

Databases like Neo4j are what caught my attention and inspired me to create something similar.

I'll probably try to build a relational database via a B+ tree at some point in the near future while on the topic of creating small-scale databases.

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

Will include testing files for the DB in a test directory

### Skills

Small disclaimer this DB does use hashmaps BUT for the purpose of creating a graph.

- Rust
- Graphs/Hashmaps, Traversals
- Gremlin (Traversal Query Languages)

### Dependencies

- maplit = "0.1.4"

### For Your Info

Project still in progress - Functionality, formatting, and other changes TBD.
