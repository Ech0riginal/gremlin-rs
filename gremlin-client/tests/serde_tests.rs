//! Welcome to Macro Town

pub struct Test {
    pub serial: serde_json::Value,
    pub object: gremlin_client::prelude::GValue,
}

macro_rules! test_prelude {
    () => {
        use crate::{test, Test};
        use gremlin_client::prelude::*;
        use serde_json::json;
    };
}

macro_rules! test {
    ($fun:ident, $engine:ident, $case:ident) => {
        mod $fun {
            pub(self) use super::*;

            mod deserialize {
                pub(self) use super::*;
                use gremlin_client::prelude::GraphSONDeserializer;

                #[test]
                fn ok() {
                    let result = $engine::deserialize(&$case.serial);
                    assert!(result.is_ok(), "Deserialization failed");
                }

                #[test]
                fn accurate() {
                    let result = $engine::deserialize(&$case.serial);
                    assert!(result.is_ok(), "Deserialization failed");
                    assert_eq!(
                        $case.object,
                        result.unwrap(),
                        "Deserialization doesn't match expectation"
                    );
                }
            }

            mod serialize {
                pub(self) use super::*;
                use gremlin_client::prelude::GraphSONSerializer;

                #[test]
                fn ok() {
                    let result = $engine::serialize(&$case.object);
                    assert!(result.is_ok(), "Serialization failed");
                }

                #[test]
                fn accurate() {
                    let result = $engine::serialize(&$case.object);
                    assert!(result.is_ok(), "Serialization failed");
                    assert_eq!(
                        $case.serial,
                        result.unwrap(),
                        "Serialization doesn't match expectation"
                    );
                }
            }
        }
    };
}

pub(self) use test;

mod v2 {
    mod core {
        use ::uuid::Uuid;
        use std::str::FromStr;

        test_prelude!();

        test!(class, V2, CLASS);
        test!(date, V2, DATE);
        test!(double, V2, DOUBLE);
        test!(float, V2, FLOAT);
        test!(integer, V2, INTEGER);
        test!(long, V2, LONG);
        test!(timestamp, V2, TIMESTAMP);
        test!(uuid, V2, UUID);

        lazy_static::lazy_static! {
            pub static ref CLASS: Test = Test {
                serial: json!({ "@type" : "g:Class", "@value" : "java.io.File"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref DATE: Test = Test {
                serial: json!({ "@type" : "g:Date", "@value" : 1481750076295u64}),
                object: GValue::Date(chrono::DateTime::from_timestamp(1481750076295, 0).unwrap()),
            };
        }

        lazy_static::lazy_static! {
            pub static ref DOUBLE: Test = Test {
                serial: json!({ "@type" : "g:Double", "@value" : 100.0f64 }),
                object: GValue::Double(100.0),
            };
        }

        lazy_static::lazy_static! {
            pub static ref FLOAT: Test = Test {
                serial: json!({ "@type" : "g:Float", "@value" : 100.0f32 }),
                object: GValue::Float(100.0),
            };
        }

        lazy_static::lazy_static! {
            pub static ref INTEGER: Test = Test {
                serial: json!({ "@type" : "g:Int32", "@value" : 100u32 }),
                object: GValue::Int32(100),
            };
        }

        lazy_static::lazy_static! {
            pub static ref LONG: Test = Test {
                serial: json!({ "@type" : "g:Int64", "@value" : 100u64 }),
                object: GValue::Int64(100),
            };
        }

        lazy_static::lazy_static! {
            pub static ref TIMESTAMP: Test = Test {
                serial: json!({ "@type" : "g:Timestamp", "@value" : 1481750076295u64}),
                object: GValue::Date(chrono::DateTime::from_timestamp(1481750076295, 0).unwrap()),
            };
        }

        lazy_static::lazy_static! {
            pub static ref UUID: Test = Test {
                serial: json!({ "@type" : "g:UUID", "@value" : "41d2e28a-20a4-4ab0-b379-d810dede3786"}),
                object: GValue::Uuid(Uuid::from_str("41d2e28a-20a4-4ab0-b379-d810dede3786").unwrap()),
            };
        }
    }
    mod structure {
        test_prelude!();

        test!(edge, V2, EDGE);
        test!(path, V2, PATH);
        test!(property, V2, PROPERTY);
        test!(stargraph, V2, STARGRAPH);
        test!(tinkergraph, V2, TINKERGRAPH);
        test!(tree, V2, TREE);
        test!(vertex, V2, VERTEX);
        test!(vertexproperty, V2, VERTEXPROPERTY);

        lazy_static::lazy_static! {
            pub static ref EDGE: Test = Test {
                serial: json!({ "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 13 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 1 }, "properties" : { "since" : { "@type" : "g:Int32", "@value" : 2009 } } }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref PATH: Test = Test {
                serial: json!({ "@type" : "g:Path", "@value" : { "labels" : [ [ ], [ ], [ ] ], "objects" : [ { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person" } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 10 }, "label" : "software", "label" : "software", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 4 }, "value" : "gremlin", "vertex" : { "@type" : "g:Int32", "@value" : 10 }, "label" : "name" } } ] } } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 11 }, "label" : "software", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 5 }, "value" : "tinkergraph", "vertex" : { "@type" : "g:Int32", "@value" : 11 }, "label" : "name" } } ] } } } ] }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref PROPERTY: Test = Test {
                serial: json!({ "@type" : "g:Property", "@value" : { "key" : "since", "value" : { "@type" : "g:Int32", "@value" : 2009 }, "element" : { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 13 }, "label" : "develops", "outV" : { "@type" : "g:Int32", "@value" : 1 }, "inV" : { "@type" : "g:Int32", "@value" : 10 } } } }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref STARGRAPH: Test = Test {
                serial: json!({ "starVertex" : { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 6 }, "value" : "san diego", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1997 }, "endTime" : { "@type" : "g:Int32", "@value" : 2001 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 7 }, "value" : "santa cruz", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2001 }, "endTime" : { "@type" : "g:Int32", "@value" : 2004 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 8 }, "value" : "brussels", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 9 }, "value" : "santa fe", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 } } } } ] } } }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref TINKERGRAPH: Test = Test {
                serial: json!({ "@type" : "tinker:graph", "@value" : { "vertices" : [ { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 6 }, "value" : "san diego", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1997 }, "endTime" : { "@type" : "g:Int32", "@value" : 2001 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 7 }, "value" : "santa cruz", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2001 }, "endTime" : { "@type" : "g:Int32", "@value" : 2004 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 8 }, "value" : "brussels", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 9 }, "value" : "santa fe", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 } } } } ] } } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 7 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 1 }, "value" : "stephen", "vertex" : { "@type" : "g:Int32", "@value" : 7 }, "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 10 }, "value" : "centreville", "vertex" : { "@type" : "g:Int32", "@value" : 7 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1990 }, "endTime" : { "@type" : "g:Int32", "@value" : 2000 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 11 }, "value" : "dulles", "vertex" : { "@type" : "g:Int32", "@value" : 7 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2000 }, "endTime" : { "@type" : "g:Int32", "@value" : 2006 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 12 }, "value" : "purcellville", "vertex" : { "@type" : "g:Int32", "@value" : 7 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2006 } } } } ] } } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 8 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 2 }, "value" : "matthias", "vertex" : { "@type" : "g:Int32", "@value" : 8 }, "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 13 }, "value" : "bremen", "vertex" : { "@type" : "g:Int32", "@value" : 8 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2007 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 14 }, "value" : "baltimore", "vertex" : { "@type" : "g:Int32", "@value" : 8 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2007 }, "endTime" : { "@type" : "g:Int32", "@value" : 2011 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 15 }, "value" : "oakland", "vertex" : { "@type" : "g:Int32", "@value" : 8 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2011 }, "endTime" : { "@type" : "g:Int32", "@value" : 2014 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 16 }, "value" : "seattle", "vertex" : { "@type" : "g:Int32", "@value" : 8 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2014 } } } } ] } } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 9 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 3 }, "value" : "daniel", "vertex" : { "@type" : "g:Int32", "@value" : 9 }, "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 17 }, "value" : "spremberg", "vertex" : { "@type" : "g:Int32", "@value" : 9 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1982 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 18 }, "value" : "kaiserslautern", "vertex" : { "@type" : "g:Int32", "@value" : 9 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 }, "endTime" : { "@type" : "g:Int32", "@value" : 2009 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 19 }, "value" : "aachen", "vertex" : { "@type" : "g:Int32", "@value" : 9 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2009 } } } } ] } } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 10 }, "label" : "software", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 4 }, "value" : "gremlin", "vertex" : { "@type" : "g:Int32", "@value" : 10 }, "label" : "name" } } ] } } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 11 }, "label" : "software", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 5 }, "value" : "tinkergraph", "vertex" : { "@type" : "g:Int32", "@value" : 11 }, "label" : "name" } } ] } } } ], "edges" : [ { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 13 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 1 }, "properties" : { "since" : { "@type" : "g:Int32", "@value" : 2009 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 14 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 1 }, "properties" : { "since" : { "@type" : "g:Int32", "@value" : 2010 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 15 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 1 }, "properties" : { "skill" : { "@type" : "g:Int32", "@value" : 4 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 16 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 1 }, "properties" : { "skill" : { "@type" : "g:Int32", "@value" : 5 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 17 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 7 }, "properties" : { "since" : { "@type" : "g:Int32", "@value" : 2010 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 18 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 7 }, "properties" : { "since" : { "@type" : "g:Int32", "@value" : 2011 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 19 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 7 }, "properties" : { "skill" : { "@type" : "g:Int32", "@value" : 5 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 20 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 7 }, "properties" : { "skill" : { "@type" : "g:Int32", "@value" : 4 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 21 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 8 }, "properties" : { "since" : { "@type" : "g:Int32", "@value" : 2012 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 22 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 8 }, "properties" : { "skill" : { "@type" : "g:Int32", "@value" : 3 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 23 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 8 }, "properties" : { "skill" : { "@type" : "g:Int32", "@value" : 3 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 24 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 9 }, "properties" : { "skill" : { "@type" : "g:Int32", "@value" : 5 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 25 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 9 }, "properties" : { "skill" : { "@type" : "g:Int32", "@value" : 3 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 26 }, "label" : "traverses", "inVLabel" : "software", "outVLabel" : "software", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 10 } } } ] }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref TREE: Test = Test {
                serial: json!({ "@type" : "g:Tree", "@value" : [ { "key" : { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 6 }, "value" : "san diego", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1997 }, "endTime" : { "@type" : "g:Int32", "@value" : 2001 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 7 }, "value" : "santa cruz", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2001 }, "endTime" : { "@type" : "g:Int32", "@value" : 2004 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 8 }, "value" : "brussels", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 9 }, "value" : "santa fe", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 } } } } ] } } }, "value" : { "@type" : "g:Tree", "@value" : [ { "key" : { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 10 }, "label" : "software", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 4 }, "value" : "gremlin", "vertex" : { "@type" : "g:Int32", "@value" : 10 }, "label" : "name" } } ] } } }, "value" : { "@type" : "g:Tree", "@value" : [ { "key" : { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 11 }, "label" : "software", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 5 }, "value" : "tinkergraph", "vertex" : { "@type" : "g:Int32", "@value" : 11 }, "label" : "name" } } ] } } }, "value" : { "@type" : "g:Tree", "@value" : [ ] } } ] } } ] } } ]}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref VERTEX: Test = Test {
                serial: json!({ "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 6 }, "value" : "san diego", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1997 }, "endTime" : { "@type" : "g:Int32", "@value" : 2001 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 7 }, "value" : "santa cruz", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2001 }, "endTime" : { "@type" : "g:Int32", "@value" : 2004 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 8 }, "value" : "brussels", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 9 }, "value" : "santa fe", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 } } } } ] } }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref VERTEXPROPERTY: Test = Test {
                serial: json!({ "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "name" }}),
                object: GValue::Null,
            };
        }
    }
    mod process {
        test_prelude!();

        test!(barrier, V2, BARRIER);
        test!(binding, V2, BINDING);
        test!(bytecode, V2, BYTECODE);
        test!(cardinality, V2, CARDINALITY);
        test!(column, V2, COLUMN);
        test!(direction, V2, DIRECTION);
        test!(operator, V2, OPERATOR);
        test!(order, V2, ORDER);
        test!(pick, V2, PICK);
        test!(pop, V2, POP);
        test!(lambda, V2, LAMBDA);
        test!(metrics, V2, METRICS);
        test!(p, V2, P);
        test!(p_within, V2, P_WITHIN);
        test!(p_without, V2, P_WITHOUT);
        test!(p_and, V2, P_AND);
        test!(p_or, V2, P_OR);
        test!(scope, V2, SCOPE);
        test!(t, V2, T);
        test!(textp, V2, TEXTP);
        test!(traversalmetrics, V2, TRAVERSALMETRICS);
        test!(traverser, V2, TRAVERSER);

        lazy_static::lazy_static! {
            pub static ref BARRIER: Test = Test {
                serial: json!({ "@type" : "g:Barrier", "@value" : "normSack"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref BINDING: Test = Test {
                serial: json!({ "@type" : "g:Binding", "@value" : { "key" : "x", "value" : { "@type" : "g:Int32", "@value" : 1 } }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref BYTECODE: Test = Test {
                serial: json!({ "@type" : "g:Bytecode", "@value" : { "step" : [ [ "V" ], [ "hasLabel", "person" ], [ "out" ], [ "in" ], [ "tree" ] ] }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref CARDINALITY: Test = Test {
                serial: json!({ "@type" : "g:Cardinality", "@value" : "list"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref COLUMN: Test = Test {
                serial: json!({ "@type" : "g:Column", "@value" : "keys"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref DIRECTION: Test = Test {
                serial: json!({ "@type" : "g:Direction", "@value" : "OUT"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref OPERATOR: Test = Test {
                serial: json!({ "@type" : "g:Operator", "@value" : "sum"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref ORDER: Test = Test {
                serial: json!({ "@type" : "g:Order", "@value" : "shuffle"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref PICK: Test = Test {
                serial: json!({ "@type" : "g:Pick", "@value" : "any"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref POP: Test = Test {
                serial: json!({ "@type" : "g:Pop", "@value" : "all"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref LAMBDA: Test = Test {
                serial: json!({ "@type" : "g:Lambda", "@value" : { "script" : "{ it.get() }", "language" : "gremlin-groovy", "arguments" : 1 }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref METRICS: Test = Test {
                serial: json!({ "@type" : "g:Metrics", "@value" : { "dur" : { "@type" : "g:Double", "@value" : 100.0 }, "counts" : { "traverserCount" : { "@type" : "g:Int64", "@value" : 4 }, "elementCount" : { "@type" : "g:Int64", "@value" : 4 } }, "name" : "TinkerGraphStep(vertex,[~label.eq(person)])", "annotations" : { "percentDur" : { "@type" : "g:Double", "@value" : 25.0 } }, "id" : "7.0.0()", "metrics" : [ { "@type" : "g:Metrics", "@value" : { "dur" : { "@type" : "g:Double", "@value" : 100.0 }, "counts" : { "traverserCount" : { "@type" : "g:Int64", "@value" : 7 }, "elementCount" : { "@type" : "g:Int64", "@value" : 7 } }, "name" : "VertexStep(OUT,vertex)", "annotations" : { "percentDur" : { "@type" : "g:Double", "@value" : 25.0 } }, "id" : "3.0.0()" } } ] }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref P: Test = Test {
                serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "gt", "value" : { "@type" : "g:Int32", "@value" : 0 } }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref P_WITHIN: Test = Test {
                serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "within", "value" : [ { "@type" : "g:Int32", "@value" : 1 } ] }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref P_WITHOUT: Test = Test {
                serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "without", "value" : [ { "@type" : "g:Int32", "@value" : 1 }, { "@type" : "g:Int32", "@value" : 2 } ] }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref P_AND: Test = Test {
                serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "and", "value" : [ { "@type" : "g:P", "@value" : { "predicate" : "gt", "value" : { "@type" : "g:Int32", "@value" : 0 } } }, { "@type" : "g:P", "@value" : { "predicate" : "lt", "value" : { "@type" : "g:Int32", "@value" : 10 } } } ] }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref P_OR: Test = Test {
                serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "or", "value" : [ { "@type" : "g:P", "@value" : { "predicate" : "gt", "value" : { "@type" : "g:Int32", "@value" : 0 } } }, { "@type" : "g:P", "@value" : { "predicate" : "within", "value" : [ { "@type" : "g:Int32", "@value" : -1 }, { "@type" : "g:Int32", "@value" : -10 }, { "@type" : "g:Int32", "@value" : -100 } ] } } ] }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref SCOPE: Test = Test {
                serial: json!({ "@type" : "g:Scope", "@value" : "local"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref T: Test = Test {
                serial: json!({ "@type" : "g:T", "@value" : "label"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref TEXTP: Test = Test {
                serial: json!({ "@type" : "g:TextP", "@value" : { "predicate" : "containing", "value" : "ark" }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref TRAVERSALMETRICS: Test = Test {
                serial: json!({ "@type" : "g:TraversalMetrics", "@value" : { "dur" : { "@type" : "g:Double", "@value" : 0.004 }, "metrics" : [ { "@type" : "g:Metrics", "@value" : { "dur" : { "@type" : "g:Double", "@value" : 100.0 }, "counts" : { "traverserCount" : { "@type" : "g:Int64", "@value" : 4 }, "elementCount" : { "@type" : "g:Int64", "@value" : 4 } }, "name" : "TinkerGraphStep(vertex,[~label.eq(person)])", "annotations" : { "percentDur" : { "@type" : "g:Double", "@value" : 25.0 } }, "id" : "7.0.0()" } }, { "@type" : "g:Metrics", "@value" : { "dur" : { "@type" : "g:Double", "@value" : 100.0 }, "counts" : { "traverserCount" : { "@type" : "g:Int64", "@value" : 13 }, "elementCount" : { "@type" : "g:Int64", "@value" : 13 } }, "name" : "VertexStep(OUT,vertex)", "annotations" : { "percentDur" : { "@type" : "g:Double", "@value" : 25.0 } }, "id" : "2.0.0()" } }, { "@type" : "g:Metrics", "@value" : { "dur" : { "@type" : "g:Double", "@value" : 100.0 }, "counts" : { "traverserCount" : { "@type" : "g:Int64", "@value" : 7 }, "elementCount" : { "@type" : "g:Int64", "@value" : 7 } }, "name" : "VertexStep(OUT,vertex)", "annotations" : { "percentDur" : { "@type" : "g:Double", "@value" : 25.0 } }, "id" : "3.0.0()" } }, { "@type" : "g:Metrics", "@value" : { "dur" : { "@type" : "g:Double", "@value" : 100.0 }, "counts" : { "traverserCount" : { "@type" : "g:Int64", "@value" : 1 }, "elementCount" : { "@type" : "g:Int64", "@value" : 1 } }, "name" : "TreeStep", "annotations" : { "percentDur" : { "@type" : "g:Double", "@value" : 25.0 } }, "id" : "4.0.0()" } } ] }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref TRAVERSER: Test = Test {
                serial: json!({ "@type" : "g:Traverser", "@value" : { "bulk" : { "@type" : "g:Int64", "@value" : 1 }, "value" : { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 6 }, "value" : "san diego", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1997 }, "endTime" : { "@type" : "g:Int32", "@value" : 2001 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 7 }, "value" : "santa cruz", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2001 }, "endTime" : { "@type" : "g:Int32", "@value" : 2004 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 8 }, "value" : "brussels", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 9 }, "value" : "santa fe", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 } } } } ] } } } }}),
                object: GValue::Null,
            };
        }
    }
    mod request {
        test_prelude!();

        test!(authentication_response, V2, AUTHENTICATION_RESPONSE);
        test!(session_eval, V2, SESSION_EVAL);
        test!(session_eval_aliased, V2, SESSION_EVAL_ALIASED);
        test!(session_close, V2, SESSION_CLOSE);
        test!(sessionless_eval, V2, SESSIONLESS_EVAL);
        test!(sessionless_eval_aliased, V2, SESSIONLESS_EVAL_ALIASED);

        lazy_static::lazy_static! {
            pub static ref AUTHENTICATION_RESPONSE: Test = Test {
                serial: json!({ "requestId" : { "@type" : "g:UUID", "@value" : "cb682578-9d92-4499-9ebc-5c6aa73c5397" }, "op" : "authentication", "processor" : "", "args" : { "saslMechanism" : "PLAIN", "sasl" : "AHN0ZXBocGhlbgBwYXNzd29yZA==" }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref SESSION_EVAL: Test = Test {
                serial: json!({ "requestId" : { "@type" : "g:UUID", "@value" : "cb682578-9d92-4499-9ebc-5c6aa73c5397" }, "op" : "eval", "processor" : "session", "args" : { "gremlin" : "g.V(x)", "language" : "gremlin-groovy", "session" : { "@type" : "g:UUID", "@value" : "41d2e28a-20a4-4ab0-b379-d810dede3786" }, "bindings" : { "x" : { "@type" : "g:Int32", "@value" : 1 } } }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref SESSION_EVAL_ALIASED: Test = Test {
                serial: json!({ "requestId" : { "@type" : "g:UUID", "@value" : "cb682578-9d92-4499-9ebc-5c6aa73c5397" }, "op" : "eval", "processor" : "session", "args" : { "gremlin" : "social.V(x)", "language" : "gremlin-groovy", "aliases" : { "g" : "social" }, "session" : { "@type" : "g:UUID", "@value" : "41d2e28a-20a4-4ab0-b379-d810dede3786" }, "bindings" : { "x" : { "@type" : "g:Int32", "@value" : 1 } } }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref SESSION_CLOSE: Test = Test {
                serial: json!({ "requestId" : { "@type" : "g:UUID", "@value" : "cb682578-9d92-4499-9ebc-5c6aa73c5397" }, "op" : "close", "processor" : "session", "args" : { "session" : { "@type" : "g:UUID", "@value" : "41d2e28a-20a4-4ab0-b379-d810dede3786" } }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref SESSIONLESS_EVAL: Test = Test {
                serial: json!({ "requestId" : { "@type" : "g:UUID", "@value" : "cb682578-9d92-4499-9ebc-5c6aa73c5397" }, "op" : "eval", "processor" : "", "args" : { "gremlin" : "g.V(x)", "language" : "gremlin-groovy", "bindings" : { "x" : { "@type" : "g:Int32", "@value" : 1 } } }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref SESSIONLESS_EVAL_ALIASED: Test = Test {
                serial: json!({ "requestId" : { "@type" : "g:UUID", "@value" : "cb682578-9d92-4499-9ebc-5c6aa73c5397" }, "op" : "eval", "processor" : "", "args" : { "gremlin" : "social.V(x)", "language" : "gremlin-groovy", "aliases" : { "g" : "social" }, "bindings" : { "x" : { "@type" : "g:Int32", "@value" : 1 } } }}),
                object: GValue::Null,
            };
        }
    }
    mod response {
        test_prelude!();

        test!(authentication_challenge, V2, AUTHENTICATION_CHALLENGE);
        test!(standard_result, V2, STANDARD_RESULT);

        lazy_static::lazy_static! {
            pub static ref AUTHENTICATION_CHALLENGE: Test = Test {
                serial: json!({ "requestId" : "41d2e28a-20a4-4ab0-b379-d810dede3786", "status" : { "message" : "", "code" : 407, "attributes" : { } }, "result" : { "data" : null, "meta" : { } }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref STANDARD_RESULT: Test = Test {
                serial: json!({ "requestId" : "41d2e28a-20a4-4ab0-b379-d810dede3786", "status" : { "message" : "", "code" : 200, "attributes" : { } }, "result" : { "data" : [ { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 6 }, "value" : "san diego", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1997 }, "endTime" : { "@type" : "g:Int32", "@value" : 2001 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 7 }, "value" : "santa cruz", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2001 }, "endTime" : { "@type" : "g:Int32", "@value" : 2004 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 8 }, "value" : "brussels", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 9 }, "value" : "santa fe", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 } } } } ] } } } ], "meta" : { } }}),
                object: GValue::Null,
            };
        }
    }
    mod extended {
        test_prelude!();

        test!(bigdecimal, V2, BIGDECIMAL);
        test!(biginteger, V2, BIGINTEGER);
        test!(byte, V2, BYTE);
        test!(bytebuffer, V2, BYTEBUFFER);
        test!(char, V2, CHAR);
        test!(duration, V2, DURATION);
        test!(inetaddress, V2, INETADDRESS);
        test!(instant, V2, INSTANT);
        test!(localdate, V2, LOCALDATE);
        test!(localdatetime, V2, LOCALDATETIME);
        test!(localtime, V2, LOCALTIME);
        test!(monthday, V2, MONTHDAY);
        test!(offsetdatetime, V2, OFFSETDATETIME);
        test!(offsettime, V2, OFFSETTIME);
        test!(period, V2, PERIOD);
        test!(short, V2, SHORT);
        test!(year, V2, YEAR);
        test!(yearmonth, V2, YEARMONTH);
        test!(zoneddatetime, V2, ZONEDDATETIME);
        test!(zoneoffset, V2, ZONEOFFSET);

        lazy_static::lazy_static! {
            pub static ref BIGDECIMAL: Test = Test {
                serial: json!({ "@type" : "gx:BigDecimal", "@value" : 123456789987654321123456789987654321u128}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref BIGINTEGER: Test = Test {
                serial: json!({ "@type" : "gx:BigInteger", "@value" : 123456789987654321123456789987654321u128 }),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref BYTE: Test = Test {
                serial: json!({ "@type" : "gx:Byte", "@value" : 1}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref BYTEBUFFER: Test = Test {
                serial: json!({ "@type" : "gx:ByteBuffer", "@value" : "c29tZSBieXRlcyBmb3IgeW91"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref CHAR: Test = Test {
                serial: json!({ "@type" : "gx:Char", "@value" : "x"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref DURATION: Test = Test {
                serial: json!({ "@type" : "gx:Duration", "@value" : "PT120H"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref INETADDRESS: Test = Test {
                serial: json!({ "@type" : "gx:InetAddress", "@value" : "localhost"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref INSTANT: Test = Test {
                serial: json!({ "@type" : "gx:Instant", "@value" : "2016-12-14T16:39:19.349Z"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref LOCALDATE: Test = Test {
                serial: json!({ "@type" : "gx:LocalDate", "@value" : "2016-01-01"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref LOCALDATETIME: Test = Test {
                serial: json!({ "@type" : "gx:LocalDateTime", "@value" : "2016-01-01T12:30"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref LOCALTIME: Test = Test {
                serial: json!({ "@type" : "gx:LocalTime", "@value" : "12:30:45"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref MONTHDAY: Test = Test {
                serial: json!({ "@type" : "gx:MonthDay", "@value" : "--01-01"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref OFFSETDATETIME: Test = Test {
                serial: json!({ "@type" : "gx:OffsetDateTime", "@value" : "2007-12-03T10:15:30+01:00"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref OFFSETTIME: Test = Test {
                serial: json!({ "@type" : "gx:OffsetTime", "@value" : "10:15:30+01:00"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref PERIOD: Test = Test {
                serial: json!({ "@type" : "gx:Period", "@value" : "P1Y6M15D"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref SHORT: Test = Test {
                serial: json!({ "@type" : "gx:Int16", "@value" : 100}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref YEAR: Test = Test {
                serial: json!({ "@type" : "gx:Year", "@value" : "2016"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref YEARMONTH: Test = Test {
                serial: json!({ "@type" : "gx:YearMonth", "@value" : "2016-06"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref ZONEDDATETIME: Test = Test {
                serial: json!({ "@type" : "gx:ZonedDateTime", "@value" : "2016-12-23T12:12:24.000000036+02:00[GMT+02:00]"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref ZONEOFFSET: Test = Test {
                serial: json!({ "@type" : "gx:ZoneOffset", "@value" : "+03:06:09"}),
                object: GValue::Null,
            };
        }
    }
}

mod v3 {
    mod core {
        use ::uuid::Uuid;
        use std::str::FromStr;

        test_prelude!();

        test!(class, V3, CLASS);
        test!(date, V3, DATE);
        test!(double, V3, DOUBLE);
        test!(float, V3, FLOAT);
        test!(integer, V3, INTEGER);
        test!(list, V3, LIST);
        test!(long, V3, LONG);
        test!(map, V3, MAP);
        test!(set, V3, SET);
        test!(timestamp, V3, TIMESTAMP);
        test!(uuid, V3, UUID);

        lazy_static::lazy_static! {
            pub static ref CLASS: Test = Test {
                serial: json!({ "@type" : "g:Class", "@value" : "java.io.File"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref DATE: Test = Test {
                serial: json!({ "@type" : "g:Date", "@value" : 1481750076295u64 }),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref DOUBLE: Test = Test {
                serial: json!({ "@type" : "g:Double", "@value" : 100.0}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref FLOAT: Test = Test {
                serial: json!({ "@type" : "g:Float", "@value" : 100.0}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref INTEGER: Test = Test {
                serial: json!({ "@type" : "g:Int32", "@value" : 100}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref LIST: Test = Test {
                serial: json!({ "@type" : "g:List", "@value" : [ { "@type" : "g:Int32", "@value" : 1 }, "person", true ]}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref LONG: Test = Test {
                serial: json!({ "@type" : "g:Int64", "@value" : 100}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref MAP: Test = Test {
                serial: json!({ "@type" : "g:Map", "@value" : [ { "@type" : "g:Date", "@value" : 1481750076295u64 }, "red", { "@type" : "g:List", "@value" : [ { "@type" : "g:Int32", "@value" : 1 }, { "@type" : "g:Int32", "@value" : 2 }, { "@type" : "g:Int32", "@value" : 3 } ] }, { "@type" : "g:Date", "@value" : 1481750076295u64 }, "test", { "@type" : "g:Int32", "@value" : 123 } ]}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref SET: Test = Test {
                serial: json!({ "@type" : "g:Set", "@value" : [ { "@type" : "g:Int32", "@value" : 1 }, "person", true ]}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref TIMESTAMP: Test = Test {
                serial: json!({ "@type" : "g:Timestamp", "@value" : 1481750076295u64 }),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref UUID: Test = Test {
                serial: json!({ "@type" : "g:UUID", "@value" : "41d2e28a-20a4-4ab0-b379-d810dede3786"}),
                object: GValue::Null,
            };
        }
    }
    mod structure {
        test_prelude!();

        test!(edge, V3, EDGE);
        test!(path, V3, PATH);
        test!(property, V3, PROPERTY);
        test!(tinkergraph, V3, TINKERGRAPH);
        test!(vertex, V3, VERTEX);
        test!(vertexproperty, V3, VERTEXPROPERTY);

        lazy_static::lazy_static! {
            pub static ref EDGE: Test = Test {
                serial: json!({ "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 13 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 1 }, "properties" : { "since" : { "@type" : "g:Property", "@value" : { "key" : "since", "value" : { "@type" : "g:Int32", "@value" : 2009 } } } } }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref PATH: Test = Test {
                serial: json!({ "@type" : "g:Path", "@value" : { "labels" : { "@type" : "g:List", "@value" : [ { "@type" : "g:Set", "@value" : [ ] }, { "@type" : "g:Set", "@value" : [ ] }, { "@type" : "g:Set", "@value" : [ ] } ] }, "objects" : { "@type" : "g:List", "@value" : [ { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person" } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 10 }, "label" : "software" } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 11 }, "label" : "software" } } ] } }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref PROPERTY: Test = Test {
                serial: json!({ "@type" : "g:Property", "@value" : { "key" : "since", "value" : { "@type" : "g:Int32", "@value" : 2009 } }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref TINKERGRAPH: Test = Test {
                serial: json!({ "@type" : "tinker:graph", "@value" : { "vertices" : [ { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 6 }, "value" : "san diego", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1997 }, "endTime" : { "@type" : "g:Int32", "@value" : 2001 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 7 }, "value" : "santa cruz", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2001 }, "endTime" : { "@type" : "g:Int32", "@value" : 2004 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 8 }, "value" : "brussels", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 9 }, "value" : "santa fe", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 } } } } ] } } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 7 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 1 }, "value" : "stephen", "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 10 }, "value" : "centreville", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1990 }, "endTime" : { "@type" : "g:Int32", "@value" : 2000 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 11 }, "value" : "dulles", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2000 }, "endTime" : { "@type" : "g:Int32", "@value" : 2006 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 12 }, "value" : "purcellville", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2006 } } } } ] } } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 8 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 2 }, "value" : "matthias", "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 13 }, "value" : "bremen", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2007 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 14 }, "value" : "baltimore", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2007 }, "endTime" : { "@type" : "g:Int32", "@value" : 2011 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 15 }, "value" : "oakland", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2011 }, "endTime" : { "@type" : "g:Int32", "@value" : 2014 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 16 }, "value" : "seattle", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2014 } } } } ] } } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 9 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 3 }, "value" : "daniel", "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 17 }, "value" : "spremberg", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1982 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 18 }, "value" : "kaiserslautern", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 }, "endTime" : { "@type" : "g:Int32", "@value" : 2009 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 19 }, "value" : "aachen", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2009 } } } } ] } } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 10 }, "label" : "software", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 4 }, "value" : "gremlin", "label" : "name" } } ] } } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 11 }, "label" : "software", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 5 }, "value" : "tinkergraph", "label" : "name" } } ] } } } ], "edges" : [ { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 13 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 1 }, "properties" : { "since" : { "@type" : "g:Property", "@value" : { "key" : "since", "value" : { "@type" : "g:Int32", "@value" : 2009 } } } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 14 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 1 }, "properties" : { "since" : { "@type" : "g:Property", "@value" : { "key" : "since", "value" : { "@type" : "g:Int32", "@value" : 2010 } } } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 15 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 1 }, "properties" : { "skill" : { "@type" : "g:Property", "@value" : { "key" : "skill", "value" : { "@type" : "g:Int32", "@value" : 4 } } } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 16 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 1 }, "properties" : { "skill" : { "@type" : "g:Property", "@value" : { "key" : "skill", "value" : { "@type" : "g:Int32", "@value" : 5 } } } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 17 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 7 }, "properties" : { "since" : { "@type" : "g:Property", "@value" : { "key" : "since", "value" : { "@type" : "g:Int32", "@value" : 2010 } } } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 18 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 7 }, "properties" : { "since" : { "@type" : "g:Property", "@value" : { "key" : "since", "value" : { "@type" : "g:Int32", "@value" : 2011 } } } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 19 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 7 }, "properties" : { "skill" : { "@type" : "g:Property", "@value" : { "key" : "skill", "value" : { "@type" : "g:Int32", "@value" : 5 } } } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 20 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 7 }, "properties" : { "skill" : { "@type" : "g:Property", "@value" : { "key" : "skill", "value" : { "@type" : "g:Int32", "@value" : 4 } } } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 21 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 8 }, "properties" : { "since" : { "@type" : "g:Property", "@value" : { "key" : "since", "value" : { "@type" : "g:Int32", "@value" : 2012 } } } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 22 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 8 }, "properties" : { "skill" : { "@type" : "g:Property", "@value" : { "key" : "skill", "value" : { "@type" : "g:Int32", "@value" : 3 } } } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 23 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 8 }, "properties" : { "skill" : { "@type" : "g:Property", "@value" : { "key" : "skill", "value" : { "@type" : "g:Int32", "@value" : 3 } } } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 24 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 9 }, "properties" : { "skill" : { "@type" : "g:Property", "@value" : { "key" : "skill", "value" : { "@type" : "g:Int32", "@value" : 5 } } } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 25 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 9 }, "properties" : { "skill" : { "@type" : "g:Property", "@value" : { "key" : "skill", "value" : { "@type" : "g:Int32", "@value" : 3 } } } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 26 }, "label" : "traverses", "inVLabel" : "software", "outVLabel" : "software", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 10 } } } ] }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref VERTEX: Test = Test {
                serial: json!({ "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 6 }, "value" : "san diego", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1997 }, "endTime" : { "@type" : "g:Int32", "@value" : 2001 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 7 }, "value" : "santa cruz", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2001 }, "endTime" : { "@type" : "g:Int32", "@value" : 2004 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 8 }, "value" : "brussels", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 9 }, "value" : "santa fe", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 } } } } ] } }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref VERTEXPROPERTY: Test = Test {
                serial: json!({ "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "label" : "name" }}),
                object: GValue::Null,
            };
        }
    }
    mod process {
        test_prelude!();

        test!(barrier, V3, BARRIER);
        test!(binding, V3, BINDING);
        test!(bulkset, V3, BULKSET);
        test!(bytecode, V3, BYTECODE);
        test!(cardinality, V3, CARDINALITY);
        test!(column, V3, COLUMN);
        test!(direction, V3, DIRECTION);
        test!(operator, V3, OPERATOR);
        test!(order, V3, ORDER);
        test!(pick, V3, PICK);
        test!(pop, V3, POP);
        test!(lambda, V3, LAMBDA);
        test!(metrics, V3, METRICS);
        test!(p, V3, P);
        test!(p_within, V3, P_WITHIN);
        test!(p_without, V3, P_WITHOUT);
        test!(p_and, V3, P_AND);
        test!(p_or, V3, P_OR);
        test!(scope, V3, SCOPE);
        test!(t, V3, T);
        test!(textp, V3, TEXTP);
        test!(traversalmetrics, V3, TRAVERSALMETRICS);
        test!(traverser, V3, TRAVERSER);

        lazy_static::lazy_static! {
            pub static ref BARRIER: Test = Test {
                serial: json!({ "@type" : "g:Barrier", "@value" : "normSack"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref BINDING: Test = Test {
                serial: json!({ "@type" : "g:Binding", "@value" : { "key" : "x", "value" : { "@type" : "g:Int32", "@value" : 1 } }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref BULKSET: Test = Test {
                serial: json!({ "@type" : "g:BulkSet", "@value" : [ "marko", { "@type" : "g:Int64", "@value" : 1 }, "josh", { "@type" : "g:Int64", "@value" : 2 } ]}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref BYTECODE: Test = Test {
                serial: json!({ "@type" : "g:Bytecode", "@value" : { "step" : [ [ "V" ], [ "hasLabel", "person" ], [ "out" ], [ "in" ], [ "tree" ] ] }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref CARDINALITY: Test = Test {
                serial: json!({ "@type" : "g:Cardinality", "@value" : "list"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref COLUMN: Test = Test {
                serial: json!({ "@type" : "g:Column", "@value" : "keys"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref DIRECTION: Test = Test {
                serial: json!({ "@type" : "g:Direction", "@value" : "OUT"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref OPERATOR: Test = Test {
                serial: json!({ "@type" : "g:Operator", "@value" : "sum"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref ORDER: Test = Test {
                serial: json!({ "@type" : "g:Order", "@value" : "shuffle"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref PICK: Test = Test {
                serial: json!({ "@type" : "g:Pick", "@value" : "any"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref POP: Test = Test {
                serial: json!({ "@type" : "g:Pop", "@value" : "all"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref LAMBDA: Test = Test {
                serial: json!({ "@type" : "g:Lambda", "@value" : { "script" : "{ it.get() }", "language" : "gremlin-groovy", "arguments" : 1 }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref METRICS: Test = Test {
                serial: json!({ "@type" : "g:Metrics", "@value" : { "@type" : "g:Map", "@value" : [ "dur", { "@type" : "g:Double", "@value" : 100.0 }, "counts", { "@type" : "g:Map", "@value" : [ "traverserCount", { "@type" : "g:Int64", "@value" : 4 }, "elementCount", { "@type" : "g:Int64", "@value" : 4 } ] }, "name", "TinkerGraphStep(vertex,[~label.eq(person)])", "annotations", { "@type" : "g:Map", "@value" : [ "percentDur", { "@type" : "g:Double", "@value" : 25.0 } ] }, "id", "7.0.0()", "metrics", { "@type" : "g:List", "@value" : [ { "@type" : "g:Metrics", "@value" : { "@type" : "g:Map", "@value" : [ "dur", { "@type" : "g:Double", "@value" : 100.0 }, "counts", { "@type" : "g:Map", "@value" : [ "traverserCount", { "@type" : "g:Int64", "@value" : 7 }, "elementCount", { "@type" : "g:Int64", "@value" : 7 } ] }, "name", "VertexStep(OUT,vertex)", "annotations", { "@type" : "g:Map", "@value" : [ "percentDur", { "@type" : "g:Double", "@value" : 25.0 } ] }, "id", "3.0.0()" ] } } ] } ] }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref P: Test = Test {
                serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "gt", "value" : { "@type" : "g:Int32", "@value" : 0 } }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref P_WITHIN: Test = Test {
                serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "within", "value" : [ { "@type" : "g:Int32", "@value" : 1 } ] }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref P_WITHOUT: Test = Test {
                serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "without", "value" : [ { "@type" : "g:Int32", "@value" : 1 }, { "@type" : "g:Int32", "@value" : 2 } ] }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref P_AND: Test = Test {
                serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "and", "value" : [ { "@type" : "g:P", "@value" : { "predicate" : "gt", "value" : { "@type" : "g:Int32", "@value" : 0 } } }, { "@type" : "g:P", "@value" : { "predicate" : "lt", "value" : { "@type" : "g:Int32", "@value" : 10 } } } ] }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref P_OR: Test = Test {
                serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "or", "value" : [ { "@type" : "g:P", "@value" : { "predicate" : "gt", "value" : { "@type" : "g:Int32", "@value" : 0 } } }, { "@type" : "g:P", "@value" : { "predicate" : "within", "value" : { "@type" : "g:List", "@value" : [ { "@type" : "g:Int32", "@value" : -1 }, { "@type" : "g:Int32", "@value" : -10 }, { "@type" : "g:Int32", "@value" : -100 } ] } } } ] }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref SCOPE: Test = Test {
                serial: json!({ "@type" : "g:Scope", "@value" : "local"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref T: Test = Test {
                serial: json!({ "@type" : "g:T", "@value" : "label"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref TEXTP: Test = Test {
                serial: json!({ "@type" : "g:TextP", "@value" : { "predicate" : "containing", "value" : "ark" }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref TRAVERSALMETRICS: Test = Test {
                serial: json!({ "@type" : "g:TraversalMetrics", "@value" : { "@type" : "g:Map", "@value" : [ "dur", { "@type" : "g:Double", "@value" : 0.004 }, "metrics", { "@type" : "g:List", "@value" : [ { "@type" : "g:Metrics", "@value" : { "@type" : "g:Map", "@value" : [ "dur", { "@type" : "g:Double", "@value" : 100.0 }, "counts", { "@type" : "g:Map", "@value" : [ "traverserCount", { "@type" : "g:Int64", "@value" : 4 }, "elementCount", { "@type" : "g:Int64", "@value" : 4 } ] }, "name", "TinkerGraphStep(vertex,[~label.eq(person)])", "annotations", { "@type" : "g:Map", "@value" : [ "percentDur", { "@type" : "g:Double", "@value" : 25.0 } ] }, "id", "7.0.0()" ] } }, { "@type" : "g:Metrics", "@value" : { "@type" : "g:Map", "@value" : [ "dur", { "@type" : "g:Double", "@value" : 100.0 }, "counts", { "@type" : "g:Map", "@value" : [ "traverserCount", { "@type" : "g:Int64", "@value" : 13 }, "elementCount", { "@type" : "g:Int64", "@value" : 13 } ] }, "name", "VertexStep(OUT,vertex)", "annotations", { "@type" : "g:Map", "@value" : [ "percentDur", { "@type" : "g:Double", "@value" : 25.0 } ] }, "id", "2.0.0()" ] } }, { "@type" : "g:Metrics", "@value" : { "@type" : "g:Map", "@value" : [ "dur", { "@type" : "g:Double", "@value" : 100.0 }, "counts", { "@type" : "g:Map", "@value" : [ "traverserCount", { "@type" : "g:Int64", "@value" : 7 }, "elementCount", { "@type" : "g:Int64", "@value" : 7 } ] }, "name", "VertexStep(OUT,vertex)", "annotations", { "@type" : "g:Map", "@value" : [ "percentDur", { "@type" : "g:Double", "@value" : 25.0 } ] }, "id", "3.0.0()" ] } }, { "@type" : "g:Metrics", "@value" : { "@type" : "g:Map", "@value" : [ "dur", { "@type" : "g:Double", "@value" : 100.0 }, "counts", { "@type" : "g:Map", "@value" : [ "traverserCount", { "@type" : "g:Int64", "@value" : 1 }, "elementCount", { "@type" : "g:Int64", "@value" : 1 } ] }, "name", "TreeStep", "annotations", { "@type" : "g:Map", "@value" : [ "percentDur", { "@type" : "g:Double", "@value" : 25.0 } ] }, "id", "4.0.0()" ] } } ] } ] }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref TRAVERSER: Test = Test {
                serial: json!({ "@type" : "g:Traverser", "@value" : { "bulk" : { "@type" : "g:Int64", "@value" : 1 }, "value" : { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 6 }, "value" : "san diego", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1997 }, "endTime" : { "@type" : "g:Int32", "@value" : 2001 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 7 }, "value" : "santa cruz", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2001 }, "endTime" : { "@type" : "g:Int32", "@value" : 2004 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 8 }, "value" : "brussels", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 9 }, "value" : "santa fe", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 } } } } ] } } } }}),
                object: GValue::Null,
            };
        }
    }
    mod request {
        test_prelude!();

        test!(authentication_response, V3, AUTHENTICATION_RESPONSE);
        test!(session_eval, V3, SESSION_EVAL);
        test!(session_eval_aliased, V3, SESSION_EVAL_ALIASED);
        test!(session_close, V3, SESSION_CLOSE);
        test!(sessionless_eval, V3, SESSIONLESS_EVAL);
        test!(sessionless_eval_aliased, V3, SESSIONLESS_EVAL_ALIASED);

        lazy_static::lazy_static! {
            pub static ref AUTHENTICATION_RESPONSE: Test = Test {
                serial: json!({ "requestId" : "cb682578-9d92-4499-9ebc-5c6aa73c5397", "op" : "authentication", "processor" : "", "args" : { "@type" : "g:Map", "@value" : [ "saslMechanism", "PLAIN", "sasl", "AHN0ZXBocGhlbgBwYXNzd29yZA==" ] }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref SESSION_EVAL: Test = Test {
                serial: json!({ "requestId" : "cb682578-9d92-4499-9ebc-5c6aa73c5397", "op" : "eval", "processor" : "session", "args" : { "@type" : "g:Map", "@value" : [ "gremlin", "g.V(x)", "language", "gremlin-groovy", "session", { "@type" : "g:UUID", "@value" : "41d2e28a-20a4-4ab0-b379-d810dede3786" }, "bindings", { "@type" : "g:Map", "@value" : [ "x", { "@type" : "g:Int32", "@value" : 1 } ] } ] }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref SESSION_EVAL_ALIASED: Test = Test {
                serial: json!({ "requestId" : "cb682578-9d92-4499-9ebc-5c6aa73c5397", "op" : "eval", "processor" : "session", "args" : { "@type" : "g:Map", "@value" : [ "gremlin", "social.V(x)", "language", "gremlin-groovy", "aliases", { "@type" : "g:Map", "@value" : [ "g", "social" ] }, "session", { "@type" : "g:UUID", "@value" : "41d2e28a-20a4-4ab0-b379-d810dede3786" }, "bindings", { "@type" : "g:Map", "@value" : [ "x", { "@type" : "g:Int32", "@value" : 1 } ] } ] }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref SESSION_CLOSE: Test = Test {
                serial: json!({ "requestId" : "cb682578-9d92-4499-9ebc-5c6aa73c5397", "op" : "close", "processor" : "session", "args" : { "@type" : "g:Map", "@value" : [ "session", { "@type" : "g:UUID", "@value" : "41d2e28a-20a4-4ab0-b379-d810dede3786" } ] }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref SESSIONLESS_EVAL: Test = Test {
                serial: json!({ "requestId" : "cb682578-9d92-4499-9ebc-5c6aa73c5397", "op" : "eval", "processor" : "", "args" : { "@type" : "g:Map", "@value" : [ "gremlin", "g.V(x)", "language", "gremlin-groovy", "bindings", { "@type" : "g:Map", "@value" : [ "x", { "@type" : "g:Int32", "@value" : 1 } ] } ] }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref SESSIONLESS_EVAL_ALIASED: Test = Test {
                serial: json!({ "requestId" : "cb682578-9d92-4499-9ebc-5c6aa73c5397", "op" : "eval", "processor" : "", "args" : { "@type" : "g:Map", "@value" : [ "gremlin", "social.V(x)", "language", "gremlin-groovy", "aliases", { "@type" : "g:Map", "@value" : [ "g", "social" ] }, "bindings", { "@type" : "g:Map", "@value" : [ "x", { "@type" : "g:Int32", "@value" : 1 } ] } ] }}),
                object: GValue::Null,
            };
        }
    }
    mod response {
        test_prelude!();

        test!(authentication_challenge, V3, AUTHENTICATION_CHALLENGE);
        test!(standard_result, V3, STANDARD_RESULT);

        lazy_static::lazy_static! {
            pub static ref AUTHENTICATION_CHALLENGE: Test = Test {
                serial: json!({ "requestId" : "41d2e28a-20a4-4ab0-b379-d810dede3786", "status" : { "message" : "", "code" : 407, "attributes" : { "@type" : "g:Map", "@value" : [ ] } }, "result" : { "data" : null, "meta" : { "@type" : "g:Map", "@value" : [ ] } }}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref STANDARD_RESULT: Test = Test {
                serial: json!({ "requestId" : "41d2e28a-20a4-4ab0-b379-d810dede3786", "status" : { "message" : "", "code" : 200, "attributes" : { "@type" : "g:Map", "@value" : [ ] } }, "result" : { "data" : { "@type" : "g:List", "@value" : [ { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 6 }, "value" : "san diego", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1997 }, "endTime" : { "@type" : "g:Int32", "@value" : 2001 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 7 }, "value" : "santa cruz", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2001 }, "endTime" : { "@type" : "g:Int32", "@value" : 2004 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 8 }, "value" : "brussels", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 9 }, "value" : "santa fe", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 } } } } ] } } } ] }, "meta" : { "@type" : "g:Map", "@value" : [ ] } }}),
                object: GValue::Null,
            };
        }
    }
    mod extended {
        test_prelude!();

        test!(bigdecimal, V3, BIGDECIMAL);
        test!(biginteger, V3, BIGINTEGER);
        test!(byte, V3, BYTE);
        test!(bytebuffer, V3, BYTEBUFFER);
        test!(char, V3, CHAR);
        test!(duration, V3, DURATION);
        test!(inetaddress, V3, INETADDRESS);
        test!(instant, V3, INSTANT);
        test!(localdate, V3, LOCALDATE);
        test!(localdatetime, V3, LOCALDATETIME);
        test!(localtime, V3, LOCALTIME);
        test!(monthday, V3, MONTHDAY);
        test!(offsetdatetime, V3, OFFSETDATETIME);
        test!(offsettime, V3, OFFSETTIME);
        test!(period, V3, PERIOD);
        test!(short, V3, SHORT);
        test!(year, V3, YEAR);
        test!(yearmonth, V3, YEARMONTH);
        test!(zoneddatetime, V3, ZONEDDATETIME);
        test!(zoneoffset, V3, ZONEOFFSET);

        lazy_static::lazy_static! {
            pub static ref BIGDECIMAL: Test = Test {
                serial: json!({ "@type" : "gx:BigDecimal", "@value" : 123456789987654321123456789987654321u128 }),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref BIGINTEGER: Test = Test {
                serial: json!({ "@type" : "gx:BigInteger", "@value" : 123456789987654321123456789987654321u128 }),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref BYTE: Test = Test {
                serial: json!({ "@type" : "gx:Byte", "@value" : 1}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref BYTEBUFFER: Test = Test {
                serial: json!({ "@type" : "gx:ByteBuffer", "@value" : "c29tZSBieXRlcyBmb3IgeW91"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref CHAR: Test = Test {
                serial: json!({ "@type" : "gx:Char", "@value" : "x"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref DURATION: Test = Test {
                serial: json!({ "@type" : "gx:Duration", "@value" : "PT120H"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref INETADDRESS: Test = Test {
                serial: json!({ "@type" : "gx:InetAddress", "@value" : "localhost"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref INSTANT: Test = Test {
                serial: json!({ "@type" : "gx:Instant", "@value" : "2016-12-14T16:39:19.349Z"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref LOCALDATE: Test = Test {
                serial: json!({ "@type" : "gx:LocalDate", "@value" : "2016-01-01"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref LOCALDATETIME: Test = Test {
                serial: json!({ "@type" : "gx:LocalDateTime", "@value" : "2016-01-01T12:30"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref LOCALTIME: Test = Test {
                serial: json!({ "@type" : "gx:LocalTime", "@value" : "12:30:45"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref MONTHDAY: Test = Test {
                serial: json!({ "@type" : "gx:MonthDay", "@value" : "--01-01"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref OFFSETDATETIME: Test = Test {
                serial: json!({ "@type" : "gx:OffsetDateTime", "@value" : "2007-12-03T10:15:30+01:00"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref OFFSETTIME: Test = Test {
                serial: json!({ "@type" : "gx:OffsetTime", "@value" : "10:15:30+01:00"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref PERIOD: Test = Test {
                serial: json!({ "@type" : "gx:Period", "@value" : "P1Y6M15D"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref SHORT: Test = Test {
                serial: json!({ "@type" : "gx:Int16", "@value" : 100}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref YEAR: Test = Test {
                serial: json!({ "@type" : "gx:Year", "@value" : "2016"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref YEARMONTH: Test = Test {
                serial: json!({ "@type" : "gx:YearMonth", "@value" : "2016-06"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref ZONEDDATETIME: Test = Test {
                serial: json!({ "@type" : "gx:ZonedDateTime", "@value" : "2016-12-23T12:12:24.000000036+02:00[GMT+02:00]"}),
                object: GValue::Null,
            };
        }

        lazy_static::lazy_static! {
            pub static ref ZONEOFFSET: Test = Test {
                serial: json!({ "@type" : "gx:ZoneOffset", "@value" : "+03:06:09"}),
                object: GValue::Null,
            };
        }
    }
}

mod v4 {}
