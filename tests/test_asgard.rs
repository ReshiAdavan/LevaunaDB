#[macro_use] extern crate maplit;

use std::collections::HashMap;
use levauna_db::*;

#[test]
fn asgard() {
    /////////////////////////////
    // 'Construct a graph'
    /////////////////////////////

    // should build an empty graph
    let mut graph = Graph::new();
    assert_eq!(graph.vertices.len(), 0);
    assert_eq!(graph.edges.len(), 0);

    // should add the Aesir
    let aesir = vec![ ["Auðumbla", "F"], ["Ymir", "M"], ["Þrúðgelmir", "M"], ["Bergelmir", "M"], ["Búri", "M"], ["Borr", "M"]
                    , ["Bölþorn", "M"], ["Bestla", "F"], ["Odin", "M"], ["Vili", "M"], ["Vé", "M"]
                    , ["Hœnir", "M"], ["Fjörgynn", "M"], ["Frigg", "F"], ["Annar", "M"]
                    , ["Jörð", "F"], ["Nepr", "M"], ["Gríðr", "F"], ["Forseti", "M"]
                    , ["Rindr", "F"], ["Dellingr", "M"], ["Nótt", "F"], ["Nanna", "F"], ["Baldr", "M"]
                    , ["Höðr", "M"], ["Hermóðr", "M"], ["Bragi", "M"], ["Iðunn", "F"], ["Víðarr", "M"]
                    , ["Váli", "M"], ["Gefjon", "F"], ["Ullr", "M"], ["Týr", "M"], ["Dagr", "M"]
                    , ["Thor", "M"], ["Sif", "F"], ["Járnsaxa", "F"], ["Nörfi", "M"]
                    , ["Móði", "M"], ["Þrúðr", "F"], ["Magni", "M"]
                    , ["Ægir", "M"], ["Rán", "F"], ["Nine sisters", "F"], ["Heimdallr", "M"]
                ];

    let mut name_idx = HashMap::new();

    for x in &aesir {
        let id = graph.add_vertex(hashmap!{
            "species".into() => Value::String("Aesir".into()),
            "name".into() => Value::String(x[0].into()),
            "gender".into() => Value::String(if x[1] == "M" { "male".into() } else { "female".into() })
        }).unwrap();

        name_idx.insert(x[0], id);
    }

    assert_eq!(graph.vertices.len(), aesir.len());
    assert_eq!(graph.edges.len(), 0);
    

    // should add the Vanir
    let vanir = vec![ "Alvaldi", "Þjazi", "Iði", "Gangr", "Fárbauti", "Nál", "Gymir", "Aurboða", "Njörðr", "Skaði"
                  , "Sigyn", "Loki", "Angrboða", "Býleistr", "Helblindi", "Beli", "Gerðr", "Freyr", "Freyja"
                  , "Óðr", "Vali", "Narfi", "Hyrrokkin", "Fenrir", "Jörmungandr", "Hel", "Fjölnir"
                  , "Hnoss", "Gersemi", "Hati Hróðvitnisson", "Sköll", "Mánagarmr"];

    for x in &vanir {
        let id = graph.add_vertex(hashmap!{
            "species".into() => Value::String("Vanir".into()),
            "name".into() => Value::String(x.to_string())
        }).unwrap();

        name_idx.insert(x, id);
    }

    assert_eq!(graph.vertices.len(), vanir.len() + aesir.len());
    assert_eq!(graph.edges.len(), 0);

    // should add some edges
    let relationships = vec![  
           ["Ymir", "Þrúðgelmir"]
        ,  ["Þrúðgelmir", "Bergelmir"]
        ,  ["Bergelmir", "Bölþorn"]
        ,  ["Bölþorn", "Bestla"]
        ,  ["Bestla", "Odin"]
        ,  ["Bestla", "Vili"]
        ,  ["Bestla", "Vé"]

        ,  ["Auðumbla", "Búri"]
        ,  ["Búri", "Borr"]
        ,  ["Borr", "Odin"]
        ,  ["Borr", "Vili"]
        ,  ["Borr", "Vé"]

        ,  ["Ægir", "Nine sisters"]
        ,  ["Rán", "Nine sisters"]
        ,  ["Nine sisters", "Heimdallr"]

        ,  ["Fjörgynn", "Frigg"]
        ,  ["Frigg", "Baldr"]
        ,  ["Odin",  "Baldr"]
        ,  ["Nepr",  "Nanna"]
        ,  ["Nanna", "Forseti"]
        ,  ["Baldr", "Forseti"]

        ,  ["Nörfi", "Nótt"]
        ,  ["Nótt", "Dagr"]
        ,  ["Nótt", "Jörð"]
        ,  ["Annar", "Jörð"]
    
        ,  ["Jörð", "Thor"]
        ,  ["Odin", "Thor"]
        ,  ["Thor", "Móði"]
        ,  ["Thor", "Þrúðr"]
        ,  ["Sif",  "Móði"]
        ,  ["Sif",  "Þrúðr"]
        ,  ["Thor", "Magni"]
        ,  ["Járnsaxa", "Magni"]
    ];

    for x in &relationships {
        graph.add_edge(*name_idx.get(x[1]).unwrap(), *name_idx.get(x[0]).unwrap(), "parent".into(), hashmap!{});
    }

    assert_eq!(graph.vertices.len(), vanir.len() + aesir.len());
    assert_eq!(graph.edges.len(), relationships.len());

    /////////////////////////////
    // Queries from the chapter
    /////////////////////////////
    
    // g.v('Thor') should be Thor
    let mut q = Query::new(&graph, VertexFilter::Id(name_idx.get("Thor").unwrap().clone()));
    let out = q.run();

    let result: Vec<&Box<Vertex>> = out.iter().filter_map(|r| graph.get_vertex(match r {QueryResult::Vertex(id) => id, _ => &0u64})).collect();
    
    assert_eq!(result[0].properties[&"name".to_string()], Value::String("Thor".to_string()));
    assert_eq!(result[0].properties[&"species".to_string()], Value::String("Aesir".to_string()));

    // g.v('Thor', 'Odin') should be Thor and Odin
    let mut q = Query::new(&graph, VertexFilter::Ids(vec![*name_idx.get("Thor").unwrap(), *name_idx.get("Odin").unwrap()]));
    let out = q.run();

    assert_eq!(out.len(), 2);
    assert!(out.contains(&QueryResult::Vertex(*name_idx.get("Thor").unwrap())));
    assert!(out.contains(&QueryResult::Vertex(*name_idx.get("Odin").unwrap())));

    // g.v({species: 'Aesir'}) should be all Aesir
    let mut q = Query::new(&graph, VertexFilter::Props(hashmap!{"species".to_string() => Value::String("Aesir".to_string())}));
    let out = q.run();

    assert_eq!(out.len(), aesir.len());

    // g.v() should be all Aesir and Vanir
    let mut q = Query::new(&graph, VertexFilter::None);
    let out = q.run();

    assert_eq!(out.len(), aesir.len() + vanir.len());

    // g.v('Thor').in().out() should contain several copies of Thor, and his wives
    let mut q = Query::new(&graph, VertexFilter::Id(*name_idx.get("Thor").unwrap()));
    let out1 = q.r#in(EdgeFilter::None).out(EdgeFilter::None).run();

    assert!(out1.contains(&QueryResult::Vertex(*name_idx.get("Járnsaxa").unwrap())));
    assert!(out1.contains(&QueryResult::Vertex(*name_idx.get("Sif").unwrap())));
    assert!(out1.contains(&QueryResult::Vertex(*name_idx.get("Thor").unwrap())));

    let mut q = Query::new(&graph, VertexFilter::Id(*name_idx.get("Thor").unwrap()));
    let out2 = q.r#in(EdgeFilter::None).out(EdgeFilter::None).unique().run();

    assert!(out2.contains(&QueryResult::Vertex(*name_idx.get("Thor").unwrap())));

    println!("{:?} {:?}", out1.len(), out2.len());
    assert!((out1.len() - out2.len()) > 0);

    // g.v('Thor').in().in().out().out() should be the empty array, 
    // because we don't know Thor's grandchildren
    let mut q = Query::new(&graph, VertexFilter::Id(*name_idx.get("Thor").unwrap()));
    let out = q.r#in(EdgeFilter::None).r#in(EdgeFilter::None).out(EdgeFilter::None).out(EdgeFilter::None).run();

    assert_eq!(out.len(), 0);

    // g.v('Thor').out().in() should contain several copies of Thor, and his sibling
    let mut q = Query::new(&graph, VertexFilter::Id(*name_idx.get("Thor").unwrap()));
    let out1 = q.out(EdgeFilter::None).r#in(EdgeFilter::None).run();

    assert!(out1.contains(&QueryResult::Vertex(*name_idx.get("Baldr").unwrap())));
    assert!(out1.contains(&QueryResult::Vertex(*name_idx.get("Thor").unwrap())));

    let mut q = Query::new(&graph, VertexFilter::Id(*name_idx.get("Thor").unwrap()));
    let out2 = q.out(EdgeFilter::None).r#in(EdgeFilter::None).unique().run();

    println!("{:?} {:?}", out1.len(), out2.len());
    assert!((out1.len() - out2.len()) > 0);

    // filter functions should filter
    let mut q = Query::new(&graph, VertexFilter::Id(*name_idx.get("Thor").unwrap()));
    let out = q.out(EdgeFilter::None).r#in(EdgeFilter::None).unique().filter(VertexFilter::Fn(Box::new(|v:&Vertex| v.properties[&"name".to_string()] != Value::String("Thor".to_string())))).run();

    assert!(out.contains(&QueryResult::Vertex(*name_idx.get("Baldr").unwrap())));
    assert!(!out.contains(&QueryResult::Vertex(*name_idx.get("Thor").unwrap())));
    assert_eq!(out.len(), 1);

    // property works like a map
    let mut q = Query::new(&graph, VertexFilter::Id(*name_idx.get("Thor").unwrap()));
    let out1:Vec<Value> = q.out(EdgeFilter::Label(String::from("parent"))).out(EdgeFilter::Label(String::from("parent")))
        .run().iter().map(|r| graph.get_vertex(r.as_vertex()).unwrap().properties.get(&"name".to_string()).unwrap().clone() ).collect();

    let mut q = Query::new(&graph, VertexFilter::Id(*name_idx.get("Thor").unwrap()));
    let out2:Vec<Value> = q.out(EdgeFilter::Label(String::from("parent"))).out(EdgeFilter::Label(String::from("parent"))).property("name".to_string())
        .run().iter().map(|r| r.as_value().clone()).collect();

    assert_eq!(out1, out2);

    // g.v('Thor').out().in().unique().filter({survives: true}) should be the empty array, because we don't label survivors
    let mut q = Query::new(&graph, VertexFilter::Id(*name_idx.get("Thor").unwrap()));
    let out = q.out(EdgeFilter::None).r#in(EdgeFilter::None).unique().filter(VertexFilter::Props(hashmap!{"survives".to_string() => Value::Bool(true)})).run();

    assert!(out.len() == 0);

    // g.v('Thor').out().in().unique().filter({gender: 'male'}) should contain Thor and his sibling
    let mut q = Query::new(&graph, VertexFilter::Id(*name_idx.get("Thor").unwrap()));
    let out = q.out(EdgeFilter::None).r#in(EdgeFilter::None).unique().filter(VertexFilter::Props(hashmap!{"gender".to_string() => Value::String("male".to_string())})).run();

    assert!(out.contains(&QueryResult::Vertex(*name_idx.get("Baldr").unwrap())));
    assert!(out.contains(&QueryResult::Vertex(*name_idx.get("Thor").unwrap())));

    // g.v('Thor').out().out().out().in().in().in() should contain Thor and his sibling
    let mut q = Query::new(&graph, VertexFilter::Id(*name_idx.get("Thor").unwrap()));
    let out = q.out(EdgeFilter::None).out(EdgeFilter::None).out(EdgeFilter::None).r#in(EdgeFilter::None).r#in(EdgeFilter::None).r#in(EdgeFilter::None).run();
    
    assert!(out.contains(&QueryResult::Vertex(*name_idx.get("Baldr").unwrap())));
    assert!(out.contains(&QueryResult::Vertex(*name_idx.get("Thor").unwrap())));

    // g.v('Thor').out().out().out().in().in().in().unique().take(10) should contain Thor and his sibling
    let mut q = Query::new(&graph, VertexFilter::Id(*name_idx.get("Thor").unwrap()));
    let out = q.out(EdgeFilter::None).out(EdgeFilter::None).out(EdgeFilter::None).r#in(EdgeFilter::None).r#in(EdgeFilter::None).r#in(EdgeFilter::None).unique().take(10).run();

    assert!(out.contains(&QueryResult::Vertex(*name_idx.get("Baldr").unwrap())));
    assert!(out.contains(&QueryResult::Vertex(*name_idx.get("Thor").unwrap())));

    
    // g.v('Thor').out().out().out().in().in().in().unique().take(10) should contain Thor and his sibling
    let mut q = Query::new(&graph, VertexFilter::Id(*name_idx.get("Thor").unwrap()));
    let out = q.out(EdgeFilter::None).out(EdgeFilter::None).out(EdgeFilter::None).out(EdgeFilter::None).r#in(EdgeFilter::None).r#in(EdgeFilter::None).r#in(EdgeFilter::None).r#in(EdgeFilter::None).unique().take(12).run();

    assert!(out.contains(&QueryResult::Vertex(*name_idx.get("Baldr").unwrap())));
    assert!(out.contains(&QueryResult::Vertex(*name_idx.get("Thor").unwrap())));

    // Asynchronous queries should work
    let mut q = Query::new(&graph, VertexFilter::Id(*name_idx.get("Auðumbla").unwrap()));
    q.r#in(EdgeFilter::None).r#in(EdgeFilter::None).r#in(EdgeFilter::None).property("name".to_string()).take(1);

    assert_eq!(q.run(), vec![ QueryResult::Value( Value::String( "Vé".to_string() )) ]);
    assert_eq!(q.run(), vec![ QueryResult::Value( Value::String( "Vili".to_string() )) ]);
    assert_eq!(q.run(), vec![ QueryResult::Value( Value::String( "Odin".to_string() )) ]);
    assert_eq!(q.run().len(), 0);
    assert_eq!(q.run().len(), 0);

    // Gathering ancestors up to three generations back
    let mut q = Query::new(&graph, VertexFilter::Id(*name_idx.get("Thor").unwrap()));
    let out = q.out(EdgeFilter::None).r#as("parent".to_string()).out(EdgeFilter::None).r#as("grandparent".to_string()).out(EdgeFilter::None).r#as("great-grandparent".to_string())
        .merge(vec!["parent".to_string(),"grandparent".to_string(),"great-grandparent".to_string()]).run();

    assert!(out.contains(&QueryResult::Vertex(*name_idx.get("Odin").unwrap())));
    assert!(out.contains(&QueryResult::Vertex(*name_idx.get("Borr").unwrap())));
    assert!(out.contains(&QueryResult::Vertex(*name_idx.get("Búri").unwrap())));
    assert!(out.contains(&QueryResult::Vertex(*name_idx.get("Jörð").unwrap())));
    assert!(out.contains(&QueryResult::Vertex(*name_idx.get("Nótt").unwrap())));
    assert!(out.contains(&QueryResult::Vertex(*name_idx.get("Nörfi").unwrap())));
    assert!(out.contains(&QueryResult::Vertex(*name_idx.get("Bestla").unwrap())));
    assert!(out.contains(&QueryResult::Vertex(*name_idx.get("Bölþorn").unwrap())));

    // Get Thor's sibling Baldr
    let mut q = Query::new(&graph, VertexFilter::Id(*name_idx.get("Thor").unwrap()));
    let out = q.r#as("me".to_string()).out(EdgeFilter::None).r#in(EdgeFilter::None).except("me".to_string()).unique().run();

    assert_eq!(out, vec![QueryResult::Vertex( *name_idx.get("Baldr").unwrap() )]);

    // Get Thor's uncles and aunts
    let mut q = Query::new(&graph, VertexFilter::Id(*name_idx.get("Thor").unwrap()));
    let out = q.out(EdgeFilter::None).r#as("parent".to_string()).out(EdgeFilter::None).r#in(EdgeFilter::None).except("parent".to_string()).unique().run();
 
    assert_eq!(out, vec![QueryResult::Vertex(*name_idx.get("Vé").unwrap()), QueryResult::Vertex(*name_idx.get("Vili").unwrap()), QueryResult::Vertex(*name_idx.get("Dagr").unwrap())]);
}
