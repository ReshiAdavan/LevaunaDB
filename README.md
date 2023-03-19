# LevaunaDB

Developed in Rust and implemented by a Gremlin inspired traversal language.

### Inspiration

Graph architectures have become increasingly popular in software design - databases, query languages, machine learning, etc. Neo4j and GraphQL are great examples. I wanted to see the hype myself but instead by implementing it myself and seeing how things work behind the scenes.

I'll probably do the same for relational databases via B+ trees since im on the topic of creating small-scale databases.

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
