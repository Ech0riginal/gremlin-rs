use super::*;
use crate::prelude::{edge, vertex};
use serde_json::json;

use crate::io::GraphSONDeserializer;
use crate::structure::{
    GValue, Map, Metric, Path, Property, Token, TraversalMetrics, Vertex, VertexProperty, GID,
};
use std::collections::HashMap;

#[test]
fn test_collections() {
    // List
    let value = json!({"@type": "g:List", "@value": [{"@type": "g:Int32", "@value": 1},
                                                      {"@type": "g:Int32", "@value": 2},
                                                      "3"]});

    let result = V3::deserialize(&value).expect("Failed to deserialize a List");

    assert_eq!(
        result,
        GValue::List(
            vec![
                GValue::Int32(1),
                GValue::Int32(2),
                GValue::String(String::from("3")),
            ]
            .into()
        )
    );

    // Set
    let value = json!({"@type": "g:Set", "@value": [{"@type": "g:Int32", "@value": 1},
                                                     {"@type": "g:Int32", "@value": 2},
                                                     {"@type": "g:Float", "@value": 2.0},
                                                     "3"]});

    let result = V3::deserialize(&value).expect("Failed to deserialize a Set");

    assert_eq!(
        result,
        GValue::List(
            vec![
                GValue::Int32(1),
                GValue::Int32(2),
                GValue::Float(2.0),
                GValue::String(String::from("3")),
            ]
            .into()
        )
    );

    // Map

    let value = json!({"@type": "g:Map",
                        "@value": ['a', {"@type": "g:Int32", "@value": 1}, 'b', "marko"]});

    let result = V3::deserialize(&value).expect("Failed to deserialize a Map");

    let mut map = HashMap::new();
    map.insert(String::from("a"), GValue::Int32(1));
    map.insert(String::from("b"), GValue::String(String::from("marko")));
    assert_eq!(result, GValue::from(map));
}

#[test]
fn test_number_input() {
    // I32
    let value = json!({
        "@type": "g:Int32",
        "@value": 31
    });

    let result = V3::deserialize(&value).expect("Failed to deserialize an Int32");
    assert_eq!(result, GValue::Int32(31));

    // I64
    let value = json!({
        "@type": "g:Int64",
        "@value": 31
    });

    let result = V3::deserialize(&value).expect("Failed to deserialize an Int64");
    assert_eq!(result, GValue::Int64(31));

    // F32
    let value = json!({
        "@type": "g:Float",
        "@value": 31.3
    });

    let result = V3::deserialize(&value).expect("Failed to deserialize Float");

    assert_eq!(result, GValue::Float(31.3));

    // F64
    let value = json!({
        "@type": "g:Double",
        "@value": 31.3
    });

    let result = V3::deserialize(&value).expect("Failed to deserialize Double");
    assert_eq!(result, GValue::Double(31.3));

    // Date
    let _ = json!({
        "@type": "g:Date",
        "@value": 1551825863
    });

    // UUID
    let value = json!({
        "@type" : "g:UUID",
        "@value" : "41d2e28a-20a4-4ab0-b379-d810dede3786"
    });

    let result = V3::deserialize(&value).expect("Failed to deserialize Double");
    assert_eq!(
        result,
        GValue::Uuid(uuid::Uuid::parse_str("41d2e28a-20a4-4ab0-b379-d810dede3786").unwrap())
    );
}

#[test]
fn test_properties() {
    let value = json!({"@type":"g:VertexProperty", "@value":{"id":{"@type":"g:Int32","@value":1},"label":"name","value":"marko"}});

    let result = V3::deserialize(&value).expect("Failed to deserialize a VertexProperty");

    assert_eq!(
        result,
        VertexProperty::new(
            GID::Int32(1),
            String::from("name"),
            GValue::String(String::from("marko"))
        )
        .into()
    );

    let value = json!({"@type":"g:Property","@value":{"key":"since","value":{"@type":"g:Int32","@value":2009}}});

    let result = V3::deserialize(&value).expect("Failed to deserialize a VertexProperty");

    assert_eq!(
        result,
        Property::new(String::from("since"), GValue::Int32(2009)).into()
    );
}
#[test]
fn test_vertex() {
    let value = json!({"@type":"g:Vertex", "@value":{"id":{"@type":"g:Int32","@value":45}}});

    let result = V3::deserialize(&value).expect("Failed to deserialize a Vertex");

    assert_eq!(
        result,
        Vertex::new(GID::Int32(45), String::from("vertex"), HashMap::new()).into()
    );

    let value = r#"{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":1},"label":"person","properties":{"name":[{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":0},"value":"marko","label":"name"}}],"location":[{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":6},"value":"san diego","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":1997},"endTime":{"@type":"g:Int32","@value":2001}}}},{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":7},"value":"santa cruz","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":2001},"endTime":{"@type":"g:Int32","@value":2004}}}},{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":8},"value":"brussels","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":2004},"endTime":{"@type":"g:Int32","@value":2005}}}},{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":9},"value":"santa fe","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":2005}}}}]}}}"#;

    let val = serde_json::from_str(&value).expect("Failed to serialize");

    let result = V3::deserialize(&val).expect("Failed to deserialize a vertex");

    assert_eq!(
        result,
        vertex!({
                id => 1,
                label => "person",
                properties => {
                    "name" => [ { id => 0 as i64 , value => "marko"}],
                    "location" => [{ id => 6 as i64, value => "san diego"},{ id => 7  as i64 , value => "santa cruz"},{ id => 8  as i64, value => "brussels"},{ id => 9  as i64, value => "santa fe"}]
                }
            }).into()
    );
}

#[test]
fn test_edge() {
    let value = json!({"@type":"g:Edge","@value":{"id":{"@type":"g:Int32","@value":13},"label":"develops","inVLabel":"software","outVLabel":"person","inV":{"@type":"g:Int32","@value":10},"outV":{"@type":"g:Int32","@value":1},"properties":{"since":{"@type":"g:Property","@value":{"key":"since","value":{"@type":"g:Int32","@value":2009}}}}}});

    let result = V3::deserialize(&value).expect("Failed to deserialize an Edge");

    assert_eq!(
        result,
        edge!({
            id => 13,
            label=> "develops",
            inV => {
                id => 10,
                label => "software"
            },
            outV => {
                id => 1,
                label => "person"
            },
            properties => {

            }
        })
        .into()
    );
}

#[test]
fn test_path() {
    let value = json!({
        "@type":"g:Path",
        "@value":{
            "labels":{
                "@type":"g:List",
                "@value":[
                    {"@type":"g:Set","@value":[]},
                    {"@type":"g:Set","@value":[]},
                    {"@type":"g:Set","@value":[]}
                ]
            },
            "objects":{
                "@type":"g:List",
                "@value":[
                    {
                        "@type":"g:Vertex",
                        "@value":{
                            "id":{
                                "@type":"g:Int32",
                                "@value":1
                            },
                            "label":"person"
                        }
                    },
                    {
                        "@type":"g:Vertex",
                        "@value":{
                            "id":{
                                "@type":"g:Int32","@value":10
                            },
                           "label":"software"
                        }
                    },{
                        "@type":"g:Vertex",
                        "@value":{
                            "id":{
                                "@type":"g:Int32",
                                "@value":11
                            },
                            "label":"software"
                        }
                    }
                ]
            }
        }
    });

    let result = V3::deserialize(&value).expect("Failed to deserialize a Path");

    let empty: GValue = vec![].into();

    let path = Path::new(
        vec![empty.clone(), empty.clone(), empty.clone()].into(),
        vec![
            vertex!({ id => 1, label => "person", properties => {}}).into(),
            vertex!({ id => 10, label => "software", properties => {}}).into(),
            vertex!({ id => 11, label => "software", properties => {}}).into(),
        ]
        .into(),
    );
    assert_eq!(result, path.into());
}

#[test]
fn test_traversal_metrics() {
    let value = serde_json::from_str(r#"{"@type":"g:TraversalMetrics","@value":{"@type":"g:Map","@value":["dur",{"@type":"g:Double","@value":0.004},"metrics",{"@type":"g:List","@value":[{"@type":"g:Metrics","@value":{"@type":"g:Map","@value":["dur",{"@type":"g:Double","@value":100.0},"counts",{"@type":"g:Map","@value":["traverserCount",{"@type":"g:Int64","@value":4},"elementCount",{"@type":"g:Int64","@value":4}]},"name","TinkerGraphStep(vertex,[~label.eq(person)])","annotations",{"@type":"g:Map","@value":["percentDur",{"@type":"g:Double","@value":25.0}]},"id","7.0.0()"]}},{"@type":"g:Metrics","@value":{"@type":"g:Map","@value":["dur",{"@type":"g:Double","@value":100.0},"counts",{"@type":"g:Map","@value":["traverserCount",{"@type":"g:Int64","@value":13},"elementCount",{"@type":"g:Int64","@value":13}]},"name","VertexStep(OUT,vertex)","annotations",{"@type":"g:Map","@value":["percentDur",{"@type":"g:Double","@value":25.0}]},"id","2.0.0()"]}},{"@type":"g:Metrics","@value":{"@type":"g:Map","@value":["dur",{"@type":"g:Double","@value":100.0},"counts",{"@type":"g:Map","@value":["traverserCount",{"@type":"g:Int64","@value":7},"elementCount",{"@type":"g:Int64","@value":7}]},"name","VertexStep(OUT,vertex)","annotations",{"@type":"g:Map","@value":["percentDur",{"@type":"g:Double","@value":25.0}]},"id","3.0.0()"]}},{"@type":"g:Metrics","@value":{"@type":"g:Map","@value":["dur",{"@type":"g:Double","@value":100.0},"counts",{"@type":"g:Map","@value":["traverserCount",{"@type":"g:Int64","@value":1},"elementCount",{"@type":"g:Int64","@value":1}]},"name","TreeStep","annotations",{"@type":"g:Map","@value":["percentDur",{"@type":"g:Double","@value":25.0}]},"id","4.0.0()"]}}]}]}}"#).expect("Error parsing json");

    let result = V3::deserialize(&value).expect("Failed to deserialize a TraversalMetrics");

    let traversal_metrics = TraversalMetrics::new(
        0.004,
        vec![
            Metric::new(
                "7.0.0()",
                "TinkerGraphStep(vertex,[~label.eq(person)])",
                100.0,
                4,
                4,
                25.0,
                vec![],
            ),
            Metric::new(
                "2.0.0()",
                "VertexStep(OUT,vertex)",
                100.0,
                13,
                13,
                25.0,
                vec![],
            ),
            Metric::new(
                "3.0.0()",
                "VertexStep(OUT,vertex)",
                100.0,
                7,
                7,
                25.0,
                vec![],
            ),
            Metric::new("4.0.0()", "TreeStep", 100.0, 1, 1, 25.0, vec![]),
        ],
    );

    assert_eq!(result, traversal_metrics.into());
}

#[test]
fn test_token() {
    let value = json!({
        "@type": "g:T",
        "@value": "id"
    });
    let result = V3::deserialize(&value).expect("Failed to deserialize a Token");

    assert_eq!(result, GValue::Token(Token::new("id")));
}

#[test]
fn test_map_with_token() {
    let value = json!({
        "@type": "g:Map",
         "@value": [
            {"@type": "g:T","@value": "label"},
            "person",
            "name",
            {"@type": "g:List","@value": ["marko"]}
         ]
    });

    let result = V3::deserialize(&value).expect("Failed to deserialize a Token");

    let value_map: Map = [
        ("label".into(), GValue::String(String::from("person"))),
        (
            "name".into(),
            GValue::List(vec![String::from("marko").into()].into()),
        ),
    ]
    .iter()
    .cloned()
    .collect();

    assert_eq!(result, GValue::Map(value_map));
}
