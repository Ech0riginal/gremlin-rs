use gremlin_client::prelude::*;
use serde_json::Value;

pub struct TestCase {
    pub serial: Value,
    pub object: GValue,
}

impl TestCase {
    pub fn test<DS: GraphSON>(&self) {
        self.deserialize::<DS>();
        self.serialialize::<DS>();
    }

    pub fn deserialize<DS: GraphSON>(&self) {
        let result = DS::deserialize(&self.serial);
        assert!(result.is_ok(), "Deserialization failed");
        assert_eq!(
            self.object,
            result.unwrap(),
            "Deserialization doesn't match expectation"
        );
    }

    /// I had a stroke typing this but its great so it stays
    pub fn serialialize<DS: GraphSON>(&self) {
        let result = DS::serialize(&self.object);
        assert!(result.is_ok(), "Serialization failed");
        assert_eq!(
            self.serial,
            result.unwrap(),
            "Serialization doesn't match expectation"
        );
    }
}

macro_rules! test {
    ($fun:ident, $engine:ident, $case:ident) => {
        #[test]
        fn $fun() {
            $case.test::<gremlin_client::prelude::$engine>();
        }
    };
}

pub(self) use test;

mod v_1d0 {
    use super::TestCase;
    use gremlin_client::prelude::*;
    use serde_json::json;

    super::test!(v1d0_edge, V2, EDGE);
    super::test!(v1d0_path, V2, PATH);
    super::test!(v1d0_property, V2, PROPERTY);
    super::test!(v1d0_tinkergraph, V2, TINKERGRAPH);
    super::test!(v1d0_vertex, V2, VERTEX);
    super::test!(v1d0_vertexproperty, V2, VERTEXPROPERTY);
    super::test!(v1d0_authentication_response, V2, AUTHENTICATION_RESPONSE);
    super::test!(v1d0_session_eval, V2, SESSION_EVAL);
    super::test!(v1d0_session_eval_aliased, V2, SESSION_EVAL_ALIASED);
    super::test!(v1d0_session_close, V2, SESSION_CLOSE);
    super::test!(v1d0_sessionless_eval, V2, SESSIONLESS_EVAL);
    super::test!(v1d0_sessionless_eval_aliased, V2, SESSIONLESS_EVAL_ALIASED);
    super::test!(v1d0_authentication_challenge, V2, AUTHENTICATION_CHALLENGE);
    super::test!(v1d0_standard_result, V2, STANDARD_RESULT);

    lazy_static::lazy_static! {
        pub static ref EDGE: TestCase = TestCase {
            serial: json!({ "id" : 13, "label" : "develops", "type" : "edge", "inVLabel" : "software", "outVLabel" : "person", "inV" : 10, "outV" : 1, "properties" : { "since" : 2009 }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref PATH: TestCase = TestCase {
            serial: json!({ "labels" : [ [ ], [ ], [ ] ], "objects" : [ { "id" : 1, "label" : "person", "type" : "vertex", "properties" : { "name" : [ { "id" : 0, "value" : "marko" } ], "location" : [ { "id" : 6, "value" : "san diego", "properties" : { "startTime" : 1997, "endTime" : 2001 } }, { "id" : 7, "value" : "santa cruz", "properties" : { "startTime" : 2001, "endTime" : 2004 } }, { "id" : 8, "value" : "brussels", "properties" : { "startTime" : 2004, "endTime" : 2005 } }, { "id" : 9, "value" : "santa fe", "properties" : { "startTime" : 2005 } } ] } }, { "id" : 10, "label" : "software", "type" : "vertex", "properties" : { "name" : [ { "id" : 4, "value" : "gremlin" } ] } }, { "id" : 11, "label" : "software", "type" : "vertex", "properties" : { "name" : [ { "id" : 5, "value" : "tinkergraph" } ] } } ]}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref PROPERTY: TestCase = TestCase {
            serial: json!({ "key" : "since", "value" : 2009}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref TINKERGRAPH: TestCase = TestCase {
            serial: json!({ "vertices" : [ { "id" : 1, "label" : "person", "type" : "vertex", "properties" : { "name" : [ { "id" : 0, "value" : "marko" } ], "location" : [ { "id" : 6, "value" : "san diego", "properties" : { "startTime" : 1997, "endTime" : 2001 } }, { "id" : 7, "value" : "santa cruz", "properties" : { "startTime" : 2001, "endTime" : 2004 } }, { "id" : 8, "value" : "brussels", "properties" : { "startTime" : 2004, "endTime" : 2005 } }, { "id" : 9, "value" : "santa fe", "properties" : { "startTime" : 2005 } } ] } }, { "id" : 7, "label" : "person", "type" : "vertex", "properties" : { "name" : [ { "id" : 1, "value" : "stephen" } ], "location" : [ { "id" : 10, "value" : "centreville", "properties" : { "startTime" : 1990, "endTime" : 2000 } }, { "id" : 11, "value" : "dulles", "properties" : { "startTime" : 2000, "endTime" : 2006 } }, { "id" : 12, "value" : "purcellville", "properties" : { "startTime" : 2006 } } ] } }, { "id" : 8, "label" : "person", "type" : "vertex", "properties" : { "name" : [ { "id" : 2, "value" : "matthias" } ], "location" : [ { "id" : 13, "value" : "bremen", "properties" : { "startTime" : 2004, "endTime" : 2007 } }, { "id" : 14, "value" : "baltimore", "properties" : { "startTime" : 2007, "endTime" : 2011 } }, { "id" : 15, "value" : "oakland", "properties" : { "startTime" : 2011, "endTime" : 2014 } }, { "id" : 16, "value" : "seattle", "properties" : { "startTime" : 2014 } } ] } }, { "id" : 9, "label" : "person", "type" : "vertex", "properties" : { "name" : [ { "id" : 3, "value" : "daniel" } ], "location" : [ { "id" : 17, "value" : "spremberg", "properties" : { "startTime" : 1982, "endTime" : 2005 } }, { "id" : 18, "value" : "kaiserslautern", "properties" : { "startTime" : 2005, "endTime" : 2009 } }, { "id" : 19, "value" : "aachen", "properties" : { "startTime" : 2009 } } ] } }, { "id" : 10, "label" : "software", "type" : "vertex", "properties" : { "name" : [ { "id" : 4, "value" : "gremlin" } ] } }, { "id" : 11, "label" : "software", "type" : "vertex", "properties" : { "name" : [ { "id" : 5, "value" : "tinkergraph" } ] } } ], "edges" : [ { "id" : 13, "label" : "develops", "type" : "edge", "inVLabel" : "software", "outVLabel" : "person", "inV" : 10, "outV" : 1, "properties" : { "since" : 2009 } }, { "id" : 14, "label" : "develops", "type" : "edge", "inVLabel" : "software", "outVLabel" : "person", "inV" : 11, "outV" : 1, "properties" : { "since" : 2010 } }, { "id" : 15, "label" : "uses", "type" : "edge", "inVLabel" : "software", "outVLabel" : "person", "inV" : 10, "outV" : 1, "properties" : { "skill" : 4 } }, { "id" : 16, "label" : "uses", "type" : "edge", "inVLabel" : "software", "outVLabel" : "person", "inV" : 11, "outV" : 1, "properties" : { "skill" : 5 } }, { "id" : 17, "label" : "develops", "type" : "edge", "inVLabel" : "software", "outVLabel" : "person", "inV" : 10, "outV" : 7, "properties" : { "since" : 2010 } }, { "id" : 18, "label" : "develops", "type" : "edge", "inVLabel" : "software", "outVLabel" : "person", "inV" : 11, "outV" : 7, "properties" : { "since" : 2011 } }, { "id" : 19, "label" : "uses", "type" : "edge", "inVLabel" : "software", "outVLabel" : "person", "inV" : 10, "outV" : 7, "properties" : { "skill" : 5 } }, { "id" : 20, "label" : "uses", "type" : "edge", "inVLabel" : "software", "outVLabel" : "person", "inV" : 11, "outV" : 7, "properties" : { "skill" : 4 } }, { "id" : 21, "label" : "develops", "type" : "edge", "inVLabel" : "software", "outVLabel" : "person", "inV" : 10, "outV" : 8, "properties" : { "since" : 2012 } }, { "id" : 22, "label" : "uses", "type" : "edge", "inVLabel" : "software", "outVLabel" : "person", "inV" : 10, "outV" : 8, "properties" : { "skill" : 3 } }, { "id" : 23, "label" : "uses", "type" : "edge", "inVLabel" : "software", "outVLabel" : "person", "inV" : 11, "outV" : 8, "properties" : { "skill" : 3 } }, { "id" : 24, "label" : "uses", "type" : "edge", "inVLabel" : "software", "outVLabel" : "person", "inV" : 10, "outV" : 9, "properties" : { "skill" : 5 } }, { "id" : 25, "label" : "uses", "type" : "edge", "inVLabel" : "software", "outVLabel" : "person", "inV" : 11, "outV" : 9, "properties" : { "skill" : 3 } }, { "id" : 26, "label" : "traverses", "type" : "edge", "inVLabel" : "software", "outVLabel" : "software", "inV" : 11, "outV" : 10 } ]}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref VERTEX: TestCase = TestCase {
            serial: json!({ "id" : 1, "label" : "person", "type" : "vertex", "properties" : { "name" : [ { "id" : 0, "value" : "marko" } ], "location" : [ { "id" : 6, "value" : "san diego", "properties" : { "startTime" : 1997, "endTime" : 2001 } }, { "id" : 7, "value" : "santa cruz", "properties" : { "startTime" : 2001, "endTime" : 2004 } }, { "id" : 8, "value" : "brussels", "properties" : { "startTime" : 2004, "endTime" : 2005 } }, { "id" : 9, "value" : "santa fe", "properties" : { "startTime" : 2005 } } ] }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref VERTEXPROPERTY: TestCase = TestCase {
            serial: json!({ "id" : 0, "value" : "marko", "label" : "name"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref AUTHENTICATION_RESPONSE: TestCase = TestCase {
            serial: json!({ "requestId" : "cb682578-9d92-4499-9ebc-5c6aa73c5397", "op" : "authentication", "processor" : "", "args" : { "saslMechanism" : "PLAIN", "sasl" : "AHN0ZXBocGhlbgBwYXNzd29yZA==" }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref SESSION_EVAL: TestCase = TestCase {
            serial: json!({ "requestId" : "cb682578-9d92-4499-9ebc-5c6aa73c5397", "op" : "eval", "processor" : "session", "args" : { "gremlin" : "g.V(x)", "language" : "gremlin-groovy", "session" : "41d2e28a-20a4-4ab0-b379-d810dede3786", "bindings" : { "x" : 1 } }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref SESSION_EVAL_ALIASED: TestCase = TestCase {
            serial: json!({ "requestId" : "cb682578-9d92-4499-9ebc-5c6aa73c5397", "op" : "eval", "processor" : "session", "args" : { "gremlin" : "social.V(x)", "language" : "gremlin-groovy", "aliases" : { "g" : "social" }, "session" : "41d2e28a-20a4-4ab0-b379-d810dede3786", "bindings" : { "x" : 1 } }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref SESSION_CLOSE: TestCase = TestCase {
            serial: json!({ "requestId" : "cb682578-9d92-4499-9ebc-5c6aa73c5397", "op" : "close", "processor" : "session", "args" : { "session" : "41d2e28a-20a4-4ab0-b379-d810dede3786" }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref SESSIONLESS_EVAL: TestCase = TestCase {
            serial: json!({ "requestId" : "cb682578-9d92-4499-9ebc-5c6aa73c5397", "op" : "eval", "processor" : "", "args" : { "gremlin" : "g.V(x)", "language" : "gremlin-groovy", "bindings" : { "x" : 1 } }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref SESSIONLESS_EVAL_ALIASED: TestCase = TestCase {
            serial: json!({ "requestId" : "cb682578-9d92-4499-9ebc-5c6aa73c5397", "op" : "eval", "processor" : "", "args" : { "gremlin" : "social.V(x)", "language" : "gremlin-groovy", "aliases" : { "g" : "social" }, "bindings" : { "x" : 1 } }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref AUTHENTICATION_CHALLENGE: TestCase = TestCase {
            serial: json!({ "requestId" : "41d2e28a-20a4-4ab0-b379-d810dede3786", "status" : { "message" : "", "code" : 407, "attributes" : { } }, "result" : { "data" : null, "meta" : { } }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref STANDARD_RESULT: TestCase = TestCase {
            serial: json!({ "requestId" : "41d2e28a-20a4-4ab0-b379-d810dede3786", "status" : { "message" : "", "code" : 200, "attributes" : { } }, "result" : { "data" : [ { "id" : 1, "label" : "person", "type" : "vertex", "properties" : { "name" : [ { "id" : 0, "value" : "marko" } ], "location" : [ { "id" : 6, "value" : "san diego", "properties" : { "startTime" : 1997, "endTime" : 2001 } }, { "id" : 7, "value" : "santa cruz", "properties" : { "startTime" : 2001, "endTime" : 2004 } }, { "id" : 8, "value" : "brussels", "properties" : { "startTime" : 2004, "endTime" : 2005 } }, { "id" : 9, "value" : "santa fe", "properties" : { "startTime" : 2005 } } ] } } ], "meta" : { } }}),
            object: GValue::Null,
        };
    }
}

mod v_2d0 {
    use super::TestCase;
    use gremlin_client::prelude::*;
    use serde_json::json;

    super::test!(v2d0_class, V2, CLASS);
    super::test!(v2d0_date, V2, DATE);
    super::test!(v2d0_double, V2, DOUBLE);
    super::test!(v2d0_float, V2, FLOAT);
    super::test!(v2d0_integer, V2, INTEGER);
    super::test!(v2d0_long, V2, LONG);
    super::test!(v2d0_timestamp, V2, TIMESTAMP);
    super::test!(v2d0_uuid, V2, UUID);
    super::test!(v2d0_edge, V2, EDGE);
    super::test!(v2d0_path, V2, PATH);
    super::test!(v2d0_property, V2, PROPERTY);
    super::test!(v2d0_stargraph, V2, STARGRAPH);
    super::test!(v2d0_tinkergraph, V2, TINKERGRAPH);
    super::test!(v2d0_tree, V2, TREE);
    super::test!(v2d0_vertex, V2, VERTEX);
    super::test!(v2d0_vertexproperty, V2, VERTEXPROPERTY);
    super::test!(v2d0_barrier, V2, BARRIER);
    super::test!(v2d0_binding, V2, BINDING);
    super::test!(v2d0_bytecode, V2, BYTECODE);
    super::test!(v2d0_cardinality, V2, CARDINALITY);
    super::test!(v2d0_column, V2, COLUMN);
    super::test!(v2d0_direction, V2, DIRECTION);
    super::test!(v2d0_operator, V2, OPERATOR);
    super::test!(v2d0_order, V2, ORDER);
    super::test!(v2d0_pick, V2, PICK);
    super::test!(v2d0_pop, V2, POP);
    super::test!(v2d0_lambda, V2, LAMBDA);
    super::test!(v2d0_metrics, V2, METRICS);
    super::test!(v2d0_p, V2, P);
    super::test!(v2d0_p_within, V2, P_WITHIN);
    super::test!(v2d0_p_without, V2, P_WITHOUT);
    super::test!(v2d0_p_and, V2, P_AND);
    super::test!(v2d0_p_or, V2, P_OR);
    super::test!(v2d0_scope, V2, SCOPE);
    super::test!(v2d0_t, V2, T);
    super::test!(v2d0_textp, V2, TEXTP);
    super::test!(v2d0_traversalmetrics, V2, TRAVERSALMETRICS);
    super::test!(v2d0_traverser, V2, TRAVERSER);
    super::test!(v2d0_authentication_response, V2, AUTHENTICATION_RESPONSE);
    super::test!(v2d0_session_eval, V2, SESSION_EVAL);
    super::test!(v2d0_session_eval_aliased, V2, SESSION_EVAL_ALIASED);
    super::test!(v2d0_session_close, V2, SESSION_CLOSE);
    super::test!(v2d0_sessionless_eval, V2, SESSIONLESS_EVAL);
    super::test!(v2d0_sessionless_eval_aliased, V2, SESSIONLESS_EVAL_ALIASED);
    super::test!(v2d0_authentication_challenge, V2, AUTHENTICATION_CHALLENGE);
    super::test!(v2d0_standard_result, V2, STANDARD_RESULT);
    super::test!(v2d0_bigdecimal, V2, BIGDECIMAL);
    super::test!(v2d0_biginteger, V2, BIGINTEGER);
    super::test!(v2d0_byte, V2, BYTE);
    super::test!(v2d0_bytebuffer, V2, BYTEBUFFER);
    super::test!(v2d0_char, V2, CHAR);
    super::test!(v2d0_duration, V2, DURATION);
    super::test!(v2d0_inetaddress, V2, INETADDRESS);
    super::test!(v2d0_instant, V2, INSTANT);
    super::test!(v2d0_localdate, V2, LOCALDATE);
    super::test!(v2d0_localdatetime, V2, LOCALDATETIME);
    super::test!(v2d0_localtime, V2, LOCALTIME);
    super::test!(v2d0_monthday, V2, MONTHDAY);
    super::test!(v2d0_offsetdatetime, V2, OFFSETDATETIME);
    super::test!(v2d0_offsettime, V2, OFFSETTIME);
    super::test!(v2d0_period, V2, PERIOD);
    super::test!(v2d0_short, V2, SHORT);
    super::test!(v2d0_year, V2, YEAR);
    super::test!(v2d0_yearmonth, V2, YEARMONTH);
    super::test!(v2d0_zoneddatetime, V2, ZONEDDATETIME);
    super::test!(v2d0_zoneoffset, V2, ZONEOFFSET);

    lazy_static::lazy_static! {
        pub static ref CLASS: TestCase = TestCase {
            serial: json!({ "@type" : "g:Class", "@value" : "java.io.File"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref DATE: TestCase = TestCase {
            serial: json!({ "@type" : "g:Date", "@value" : 1481750076295u64}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref DOUBLE: TestCase = TestCase {
            serial: json!({ "@type" : "g:Double", "@value" : 100.0f32}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref FLOAT: TestCase = TestCase {
            serial: json!({ "@type" : "g:Float", "@value" : 100.0f64}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref INTEGER: TestCase = TestCase {
            serial: json!({ "@type" : "g:Int32", "@value" : 100u32}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref LONG: TestCase = TestCase {
            serial: json!({ "@type" : "g:Int64", "@value" : 100u64 }),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref TIMESTAMP: TestCase = TestCase {
            serial: json!({ "@type" : "g:Timestamp", "@value" : 1481750076295u64}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref UUID: TestCase = TestCase {
            serial: json!({ "@type" : "g:UUID", "@value" : "41d2e28a-20a4-4ab0-b379-d810dede3786"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref EDGE: TestCase = TestCase {
            serial: json!({ "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 13 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 1 }, "properties" : { "since" : { "@type" : "g:Int32", "@value" : 2009 } } }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref PATH: TestCase = TestCase {
            serial: json!({ "@type" : "g:Path", "@value" : { "labels" : [ [ ], [ ], [ ] ], "objects" : [ { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person" } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 10 }, "label" : "software", "label" : "software", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 4 }, "value" : "gremlin", "vertex" : { "@type" : "g:Int32", "@value" : 10 }, "label" : "name" } } ] } } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 11 }, "label" : "software", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 5 }, "value" : "tinkergraph", "vertex" : { "@type" : "g:Int32", "@value" : 11 }, "label" : "name" } } ] } } } ] }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref PROPERTY: TestCase = TestCase {
            serial: json!({ "@type" : "g:Property", "@value" : { "key" : "since", "value" : { "@type" : "g:Int32", "@value" : 2009 }, "element" : { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 13 }, "label" : "develops", "outV" : { "@type" : "g:Int32", "@value" : 1 }, "inV" : { "@type" : "g:Int32", "@value" : 10 } } } }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref STARGRAPH: TestCase = TestCase {
            serial: json!({ "starVertex" : { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 6 }, "value" : "san diego", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1997 }, "endTime" : { "@type" : "g:Int32", "@value" : 2001 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 7 }, "value" : "santa cruz", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2001 }, "endTime" : { "@type" : "g:Int32", "@value" : 2004 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 8 }, "value" : "brussels", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 9 }, "value" : "santa fe", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 } } } } ] } } }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref TINKERGRAPH: TestCase = TestCase {
            serial: json!({ "@type" : "tinker:graph", "@value" : { "vertices" : [ { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 6 }, "value" : "san diego", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1997 }, "endTime" : { "@type" : "g:Int32", "@value" : 2001 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 7 }, "value" : "santa cruz", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2001 }, "endTime" : { "@type" : "g:Int32", "@value" : 2004 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 8 }, "value" : "brussels", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 9 }, "value" : "santa fe", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 } } } } ] } } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 7 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 1 }, "value" : "stephen", "vertex" : { "@type" : "g:Int32", "@value" : 7 }, "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 10 }, "value" : "centreville", "vertex" : { "@type" : "g:Int32", "@value" : 7 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1990 }, "endTime" : { "@type" : "g:Int32", "@value" : 2000 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 11 }, "value" : "dulles", "vertex" : { "@type" : "g:Int32", "@value" : 7 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2000 }, "endTime" : { "@type" : "g:Int32", "@value" : 2006 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 12 }, "value" : "purcellville", "vertex" : { "@type" : "g:Int32", "@value" : 7 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2006 } } } } ] } } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 8 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 2 }, "value" : "matthias", "vertex" : { "@type" : "g:Int32", "@value" : 8 }, "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 13 }, "value" : "bremen", "vertex" : { "@type" : "g:Int32", "@value" : 8 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2007 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 14 }, "value" : "baltimore", "vertex" : { "@type" : "g:Int32", "@value" : 8 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2007 }, "endTime" : { "@type" : "g:Int32", "@value" : 2011 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 15 }, "value" : "oakland", "vertex" : { "@type" : "g:Int32", "@value" : 8 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2011 }, "endTime" : { "@type" : "g:Int32", "@value" : 2014 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 16 }, "value" : "seattle", "vertex" : { "@type" : "g:Int32", "@value" : 8 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2014 } } } } ] } } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 9 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 3 }, "value" : "daniel", "vertex" : { "@type" : "g:Int32", "@value" : 9 }, "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 17 }, "value" : "spremberg", "vertex" : { "@type" : "g:Int32", "@value" : 9 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1982 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 18 }, "value" : "kaiserslautern", "vertex" : { "@type" : "g:Int32", "@value" : 9 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 }, "endTime" : { "@type" : "g:Int32", "@value" : 2009 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 19 }, "value" : "aachen", "vertex" : { "@type" : "g:Int32", "@value" : 9 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2009 } } } } ] } } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 10 }, "label" : "software", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 4 }, "value" : "gremlin", "vertex" : { "@type" : "g:Int32", "@value" : 10 }, "label" : "name" } } ] } } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 11 }, "label" : "software", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 5 }, "value" : "tinkergraph", "vertex" : { "@type" : "g:Int32", "@value" : 11 }, "label" : "name" } } ] } } } ], "edges" : [ { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 13 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 1 }, "properties" : { "since" : { "@type" : "g:Int32", "@value" : 2009 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 14 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 1 }, "properties" : { "since" : { "@type" : "g:Int32", "@value" : 2010 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 15 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 1 }, "properties" : { "skill" : { "@type" : "g:Int32", "@value" : 4 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 16 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 1 }, "properties" : { "skill" : { "@type" : "g:Int32", "@value" : 5 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 17 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 7 }, "properties" : { "since" : { "@type" : "g:Int32", "@value" : 2010 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 18 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 7 }, "properties" : { "since" : { "@type" : "g:Int32", "@value" : 2011 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 19 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 7 }, "properties" : { "skill" : { "@type" : "g:Int32", "@value" : 5 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 20 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 7 }, "properties" : { "skill" : { "@type" : "g:Int32", "@value" : 4 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 21 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 8 }, "properties" : { "since" : { "@type" : "g:Int32", "@value" : 2012 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 22 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 8 }, "properties" : { "skill" : { "@type" : "g:Int32", "@value" : 3 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 23 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 8 }, "properties" : { "skill" : { "@type" : "g:Int32", "@value" : 3 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 24 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 9 }, "properties" : { "skill" : { "@type" : "g:Int32", "@value" : 5 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 25 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 9 }, "properties" : { "skill" : { "@type" : "g:Int32", "@value" : 3 } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 26 }, "label" : "traverses", "inVLabel" : "software", "outVLabel" : "software", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 10 } } } ] }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref TREE: TestCase = TestCase {
            serial: json!({ "@type" : "g:Tree", "@value" : [ { "key" : { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 6 }, "value" : "san diego", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1997 }, "endTime" : { "@type" : "g:Int32", "@value" : 2001 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 7 }, "value" : "santa cruz", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2001 }, "endTime" : { "@type" : "g:Int32", "@value" : 2004 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 8 }, "value" : "brussels", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 9 }, "value" : "santa fe", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 } } } } ] } } }, "value" : { "@type" : "g:Tree", "@value" : [ { "key" : { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 10 }, "label" : "software", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 4 }, "value" : "gremlin", "vertex" : { "@type" : "g:Int32", "@value" : 10 }, "label" : "name" } } ] } } }, "value" : { "@type" : "g:Tree", "@value" : [ { "key" : { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 11 }, "label" : "software", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 5 }, "value" : "tinkergraph", "vertex" : { "@type" : "g:Int32", "@value" : 11 }, "label" : "name" } } ] } } }, "value" : { "@type" : "g:Tree", "@value" : [ ] } } ] } } ] } } ]}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref VERTEX: TestCase = TestCase {
            serial: json!({ "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 6 }, "value" : "san diego", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1997 }, "endTime" : { "@type" : "g:Int32", "@value" : 2001 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 7 }, "value" : "santa cruz", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2001 }, "endTime" : { "@type" : "g:Int32", "@value" : 2004 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 8 }, "value" : "brussels", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 9 }, "value" : "santa fe", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 } } } } ] } }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref VERTEXPROPERTY: TestCase = TestCase {
            serial: json!({ "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "name" }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref BARRIER: TestCase = TestCase {
            serial: json!({ "@type" : "g:Barrier", "@value" : "normSack"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref BINDING: TestCase = TestCase {
            serial: json!({ "@type" : "g:Binding", "@value" : { "key" : "x", "value" : { "@type" : "g:Int32", "@value" : 1 } }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref BYTECODE: TestCase = TestCase {
            serial: json!({ "@type" : "g:Bytecode", "@value" : { "step" : [ [ "V" ], [ "hasLabel", "person" ], [ "out" ], [ "in" ], [ "tree" ] ] }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref CARDINALITY: TestCase = TestCase {
            serial: json!({ "@type" : "g:Cardinality", "@value" : "list"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref COLUMN: TestCase = TestCase {
            serial: json!({ "@type" : "g:Column", "@value" : "keys"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref DIRECTION: TestCase = TestCase {
            serial: json!({ "@type" : "g:Direction", "@value" : "OUT"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref OPERATOR: TestCase = TestCase {
            serial: json!({ "@type" : "g:Operator", "@value" : "sum"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref ORDER: TestCase = TestCase {
            serial: json!({ "@type" : "g:Order", "@value" : "shuffle"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref PICK: TestCase = TestCase {
            serial: json!({ "@type" : "g:Pick", "@value" : "any"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref POP: TestCase = TestCase {
            serial: json!({ "@type" : "g:Pop", "@value" : "all"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref LAMBDA: TestCase = TestCase {
            serial: json!({ "@type" : "g:Lambda", "@value" : { "script" : "{ it.get() }", "language" : "gremlin-groovy", "arguments" : 1 }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref METRICS: TestCase = TestCase {
            serial: json!({ "@type" : "g:Metrics", "@value" : { "dur" : { "@type" : "g:Double", "@value" : 100.0 }, "counts" : { "traverserCount" : { "@type" : "g:Int64", "@value" : 4 }, "elementCount" : { "@type" : "g:Int64", "@value" : 4 } }, "name" : "TinkerGraphStep(vertex,[~label.eq(person)])", "annotations" : { "percentDur" : { "@type" : "g:Double", "@value" : 25.0 } }, "id" : "7.0.0()", "metrics" : [ { "@type" : "g:Metrics", "@value" : { "dur" : { "@type" : "g:Double", "@value" : 100.0 }, "counts" : { "traverserCount" : { "@type" : "g:Int64", "@value" : 7 }, "elementCount" : { "@type" : "g:Int64", "@value" : 7 } }, "name" : "VertexStep(OUT,vertex)", "annotations" : { "percentDur" : { "@type" : "g:Double", "@value" : 25.0 } }, "id" : "3.0.0()" } } ] }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref P: TestCase = TestCase {
            serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "gt", "value" : { "@type" : "g:Int32", "@value" : 0 } }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref P_WITHIN: TestCase = TestCase {
            serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "within", "value" : [ { "@type" : "g:Int32", "@value" : 1 } ] }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref P_WITHOUT: TestCase = TestCase {
            serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "without", "value" : [ { "@type" : "g:Int32", "@value" : 1 }, { "@type" : "g:Int32", "@value" : 2 } ] }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref P_AND: TestCase = TestCase {
            serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "and", "value" : [ { "@type" : "g:P", "@value" : { "predicate" : "gt", "value" : { "@type" : "g:Int32", "@value" : 0 } } }, { "@type" : "g:P", "@value" : { "predicate" : "lt", "value" : { "@type" : "g:Int32", "@value" : 10 } } } ] }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref P_OR: TestCase = TestCase {
            serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "or", "value" : [ { "@type" : "g:P", "@value" : { "predicate" : "gt", "value" : { "@type" : "g:Int32", "@value" : 0 } } }, { "@type" : "g:P", "@value" : { "predicate" : "within", "value" : [ { "@type" : "g:Int32", "@value" : -1 }, { "@type" : "g:Int32", "@value" : -10 }, { "@type" : "g:Int32", "@value" : -100 } ] } } ] }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref SCOPE: TestCase = TestCase {
            serial: json!({ "@type" : "g:Scope", "@value" : "local"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref T: TestCase = TestCase {
            serial: json!({ "@type" : "g:T", "@value" : "label"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref TEXTP: TestCase = TestCase {
            serial: json!({ "@type" : "g:TextP", "@value" : { "predicate" : "containing", "value" : "ark" }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref TRAVERSALMETRICS: TestCase = TestCase {
            serial: json!({ "@type" : "g:TraversalMetrics", "@value" : { "dur" : { "@type" : "g:Double", "@value" : 0.004 }, "metrics" : [ { "@type" : "g:Metrics", "@value" : { "dur" : { "@type" : "g:Double", "@value" : 100.0 }, "counts" : { "traverserCount" : { "@type" : "g:Int64", "@value" : 4 }, "elementCount" : { "@type" : "g:Int64", "@value" : 4 } }, "name" : "TinkerGraphStep(vertex,[~label.eq(person)])", "annotations" : { "percentDur" : { "@type" : "g:Double", "@value" : 25.0 } }, "id" : "7.0.0()" } }, { "@type" : "g:Metrics", "@value" : { "dur" : { "@type" : "g:Double", "@value" : 100.0 }, "counts" : { "traverserCount" : { "@type" : "g:Int64", "@value" : 13 }, "elementCount" : { "@type" : "g:Int64", "@value" : 13 } }, "name" : "VertexStep(OUT,vertex)", "annotations" : { "percentDur" : { "@type" : "g:Double", "@value" : 25.0 } }, "id" : "2.0.0()" } }, { "@type" : "g:Metrics", "@value" : { "dur" : { "@type" : "g:Double", "@value" : 100.0 }, "counts" : { "traverserCount" : { "@type" : "g:Int64", "@value" : 7 }, "elementCount" : { "@type" : "g:Int64", "@value" : 7 } }, "name" : "VertexStep(OUT,vertex)", "annotations" : { "percentDur" : { "@type" : "g:Double", "@value" : 25.0 } }, "id" : "3.0.0()" } }, { "@type" : "g:Metrics", "@value" : { "dur" : { "@type" : "g:Double", "@value" : 100.0 }, "counts" : { "traverserCount" : { "@type" : "g:Int64", "@value" : 1 }, "elementCount" : { "@type" : "g:Int64", "@value" : 1 } }, "name" : "TreeStep", "annotations" : { "percentDur" : { "@type" : "g:Double", "@value" : 25.0 } }, "id" : "4.0.0()" } } ] }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref TRAVERSER: TestCase = TestCase {
            serial: json!({ "@type" : "g:Traverser", "@value" : { "bulk" : { "@type" : "g:Int64", "@value" : 1 }, "value" : { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 6 }, "value" : "san diego", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1997 }, "endTime" : { "@type" : "g:Int32", "@value" : 2001 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 7 }, "value" : "santa cruz", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2001 }, "endTime" : { "@type" : "g:Int32", "@value" : 2004 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 8 }, "value" : "brussels", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 9 }, "value" : "santa fe", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 } } } } ] } } } }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref AUTHENTICATION_RESPONSE: TestCase = TestCase {
            serial: json!({ "requestId" : { "@type" : "g:UUID", "@value" : "cb682578-9d92-4499-9ebc-5c6aa73c5397" }, "op" : "authentication", "processor" : "", "args" : { "saslMechanism" : "PLAIN", "sasl" : "AHN0ZXBocGhlbgBwYXNzd29yZA==" }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref SESSION_EVAL: TestCase = TestCase {
            serial: json!({ "requestId" : { "@type" : "g:UUID", "@value" : "cb682578-9d92-4499-9ebc-5c6aa73c5397" }, "op" : "eval", "processor" : "session", "args" : { "gremlin" : "g.V(x)", "language" : "gremlin-groovy", "session" : { "@type" : "g:UUID", "@value" : "41d2e28a-20a4-4ab0-b379-d810dede3786" }, "bindings" : { "x" : { "@type" : "g:Int32", "@value" : 1 } } }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref SESSION_EVAL_ALIASED: TestCase = TestCase {
            serial: json!({ "requestId" : { "@type" : "g:UUID", "@value" : "cb682578-9d92-4499-9ebc-5c6aa73c5397" }, "op" : "eval", "processor" : "session", "args" : { "gremlin" : "social.V(x)", "language" : "gremlin-groovy", "aliases" : { "g" : "social" }, "session" : { "@type" : "g:UUID", "@value" : "41d2e28a-20a4-4ab0-b379-d810dede3786" }, "bindings" : { "x" : { "@type" : "g:Int32", "@value" : 1 } } }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref SESSION_CLOSE: TestCase = TestCase {
            serial: json!({ "requestId" : { "@type" : "g:UUID", "@value" : "cb682578-9d92-4499-9ebc-5c6aa73c5397" }, "op" : "close", "processor" : "session", "args" : { "session" : { "@type" : "g:UUID", "@value" : "41d2e28a-20a4-4ab0-b379-d810dede3786" } }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref SESSIONLESS_EVAL: TestCase = TestCase {
            serial: json!({ "requestId" : { "@type" : "g:UUID", "@value" : "cb682578-9d92-4499-9ebc-5c6aa73c5397" }, "op" : "eval", "processor" : "", "args" : { "gremlin" : "g.V(x)", "language" : "gremlin-groovy", "bindings" : { "x" : { "@type" : "g:Int32", "@value" : 1 } } }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref SESSIONLESS_EVAL_ALIASED: TestCase = TestCase {
            serial: json!({ "requestId" : { "@type" : "g:UUID", "@value" : "cb682578-9d92-4499-9ebc-5c6aa73c5397" }, "op" : "eval", "processor" : "", "args" : { "gremlin" : "social.V(x)", "language" : "gremlin-groovy", "aliases" : { "g" : "social" }, "bindings" : { "x" : { "@type" : "g:Int32", "@value" : 1 } } }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref AUTHENTICATION_CHALLENGE: TestCase = TestCase {
            serial: json!({ "requestId" : "41d2e28a-20a4-4ab0-b379-d810dede3786", "status" : { "message" : "", "code" : 407, "attributes" : { } }, "result" : { "data" : null, "meta" : { } }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref STANDARD_RESULT: TestCase = TestCase {
            serial: json!({ "requestId" : "41d2e28a-20a4-4ab0-b379-d810dede3786", "status" : { "message" : "", "code" : 200, "attributes" : { } }, "result" : { "data" : [ { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 6 }, "value" : "san diego", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1997 }, "endTime" : { "@type" : "g:Int32", "@value" : 2001 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 7 }, "value" : "santa cruz", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2001 }, "endTime" : { "@type" : "g:Int32", "@value" : 2004 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 8 }, "value" : "brussels", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 9 }, "value" : "santa fe", "vertex" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 } } } } ] } } } ], "meta" : { } }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref BIGDECIMAL: TestCase = TestCase {
            serial: json!({ "@type" : "gx:BigDecimal", "@value" : 123456789987654321123456789987654321u128}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref BIGINTEGER: TestCase = TestCase {
            serial: json!({ "@type" : "gx:BigInteger", "@value" : 123456789987654321123456789987654321u128}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref BYTE: TestCase = TestCase {
            serial: json!({ "@type" : "gx:Byte", "@value" : 1}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref BYTEBUFFER: TestCase = TestCase {
            serial: json!({ "@type" : "gx:ByteBuffer", "@value" : "c29tZSBieXRlcyBmb3IgeW91"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref CHAR: TestCase = TestCase {
            serial: json!({ "@type" : "gx:Char", "@value" : "x"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref DURATION: TestCase = TestCase {
            serial: json!({ "@type" : "gx:Duration", "@value" : "PT120H"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref INETADDRESS: TestCase = TestCase {
            serial: json!({ "@type" : "gx:InetAddress", "@value" : "localhost"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref INSTANT: TestCase = TestCase {
            serial: json!({ "@type" : "gx:Instant", "@value" : "2016-12-14T16:39:19.349Z"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref LOCALDATE: TestCase = TestCase {
            serial: json!({ "@type" : "gx:LocalDate", "@value" : "2016-01-01"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref LOCALDATETIME: TestCase = TestCase {
            serial: json!({ "@type" : "gx:LocalDateTime", "@value" : "2016-01-01T12:30"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref LOCALTIME: TestCase = TestCase {
            serial: json!({ "@type" : "gx:LocalTime", "@value" : "12:30:45"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref MONTHDAY: TestCase = TestCase {
            serial: json!({ "@type" : "gx:MonthDay", "@value" : "--01-01"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref OFFSETDATETIME: TestCase = TestCase {
            serial: json!({ "@type" : "gx:OffsetDateTime", "@value" : "2007-12-03T10:15:30+01:00"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref OFFSETTIME: TestCase = TestCase {
            serial: json!({ "@type" : "gx:OffsetTime", "@value" : "10:15:30+01:00"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref PERIOD: TestCase = TestCase {
            serial: json!({ "@type" : "gx:Period", "@value" : "P1Y6M15D"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref SHORT: TestCase = TestCase {
            serial: json!({ "@type" : "gx:Int16", "@value" : 100}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref YEAR: TestCase = TestCase {
            serial: json!({ "@type" : "gx:Year", "@value" : "2016"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref YEARMONTH: TestCase = TestCase {
            serial: json!({ "@type" : "gx:YearMonth", "@value" : "2016-06"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref ZONEDDATETIME: TestCase = TestCase {
            serial: json!({ "@type" : "gx:ZonedDateTime", "@value" : "2016-12-23T12:12:24.000000036+02:00[GMT+02:00]"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref ZONEOFFSET: TestCase = TestCase {
            serial: json!({ "@type" : "gx:ZoneOffset", "@value" : "+03:06:09"}),
            object: GValue::Null,
        };
    }
}

mod v_3d0 {
    use super::TestCase;
    use gremlin_client::prelude::*;
    use serde_json::json;

    super::test!(v3d0_class, V3, CLASS);
    super::test!(v3d0_date, V3, DATE);
    super::test!(v3d0_double, V3, DOUBLE);
    super::test!(v3d0_float, V3, FLOAT);
    super::test!(v3d0_integer, V3, INTEGER);
    super::test!(v3d0_list, V3, LIST);
    super::test!(v3d0_long, V3, LONG);
    super::test!(v3d0_map, V3, MAP);
    super::test!(v3d0_set, V3, SET);
    super::test!(v3d0_timestamp, V3, TIMESTAMP);
    super::test!(v3d0_uuid, V3, UUID);
    super::test!(v3d0_edge, V3, EDGE);
    super::test!(v3d0_path, V3, PATH);
    super::test!(v3d0_property, V3, PROPERTY);
    super::test!(v3d0_tinkergraph, V3, TINKERGRAPH);
    super::test!(v3d0_vertex, V3, VERTEX);
    super::test!(v3d0_vertexproperty, V3, VERTEXPROPERTY);
    super::test!(v3d0_barrier, V3, BARRIER);
    super::test!(v3d0_binding, V3, BINDING);
    super::test!(v3d0_bulkset, V3, BULKSET);
    super::test!(v3d0_bytecode, V3, BYTECODE);
    super::test!(v3d0_cardinality, V3, CARDINALITY);
    super::test!(v3d0_column, V3, COLUMN);
    super::test!(v3d0_direction, V3, DIRECTION);
    super::test!(v3d0_operator, V3, OPERATOR);
    super::test!(v3d0_order, V3, ORDER);
    super::test!(v3d0_pick, V3, PICK);
    super::test!(v3d0_pop, V3, POP);
    super::test!(v3d0_lambda, V3, LAMBDA);
    super::test!(v3d0_metrics, V3, METRICS);
    super::test!(v3d0_p, V3, P);
    super::test!(v3d0_p_within, V3, P_WITHIN);
    super::test!(v3d0_p_without, V3, P_WITHOUT);
    super::test!(v3d0_p_and, V3, P_AND);
    super::test!(v3d0_p_or, V3, P_OR);
    super::test!(v3d0_scope, V3, SCOPE);
    super::test!(v3d0_t, V3, T);
    super::test!(v3d0_textp, V3, TEXTP);
    super::test!(v3d0_traversalmetrics, V3, TRAVERSALMETRICS);
    super::test!(v3d0_traverser, V3, TRAVERSER);
    super::test!(v3d0_authentication_response, V3, AUTHENTICATION_RESPONSE);
    super::test!(v3d0_session_eval, V3, SESSION_EVAL);
    super::test!(v3d0_session_eval_aliased, V3, SESSION_EVAL_ALIASED);
    super::test!(v3d0_session_close, V3, SESSION_CLOSE);
    super::test!(v3d0_sessionless_eval, V3, SESSIONLESS_EVAL);
    super::test!(v3d0_sessionless_eval_aliased, V3, SESSIONLESS_EVAL_ALIASED);
    super::test!(v3d0_authentication_challenge, V3, AUTHENTICATION_CHALLENGE);
    super::test!(v3d0_standard_result, V3, STANDARD_RESULT);
    super::test!(v3d0_bigdecimal, V3, BIGDECIMAL);
    super::test!(v3d0_biginteger, V3, BIGINTEGER);
    super::test!(v3d0_byte, V3, BYTE);
    super::test!(v3d0_bytebuffer, V3, BYTEBUFFER);
    super::test!(v3d0_char, V3, CHAR);
    super::test!(v3d0_duration, V3, DURATION);
    super::test!(v3d0_inetaddress, V3, INETADDRESS);
    super::test!(v3d0_instant, V3, INSTANT);
    super::test!(v3d0_localdate, V3, LOCALDATE);
    super::test!(v3d0_localdatetime, V3, LOCALDATETIME);
    super::test!(v3d0_localtime, V3, LOCALTIME);
    super::test!(v3d0_monthday, V3, MONTHDAY);
    super::test!(v3d0_offsetdatetime, V3, OFFSETDATETIME);
    super::test!(v3d0_offsettime, V3, OFFSETTIME);
    super::test!(v3d0_period, V3, PERIOD);
    super::test!(v3d0_short, V3, SHORT);
    super::test!(v3d0_year, V3, YEAR);
    super::test!(v3d0_yearmonth, V3, YEARMONTH);
    super::test!(v3d0_zoneddatetime, V3, ZONEDDATETIME);
    super::test!(v3d0_zoneoffset, V3, ZONEOFFSET);

    lazy_static::lazy_static! {
        pub static ref CLASS: TestCase = TestCase {
            serial: json!({ "@type" : "g:Class", "@value" : "java.io.File"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref DATE: TestCase = TestCase {
            serial: json!({ "@type" : "g:Date", "@value" : 1481750076295u64 }),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref DOUBLE: TestCase = TestCase {
            serial: json!({ "@type" : "g:Double", "@value" : 100.0f32 }),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref FLOAT: TestCase = TestCase {
            serial: json!({ "@type" : "g:Float", "@value" : 100.0f64 }),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref INTEGER: TestCase = TestCase {
            serial: json!({ "@type" : "g:Int32", "@value" : 100u32 }),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref LIST: TestCase = TestCase {
            serial: json!({ "@type" : "g:List", "@value" : [ { "@type" : "g:Int32", "@value" : 1 }, "person", true ]}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref LONG: TestCase = TestCase {
            serial: json!({ "@type" : "g:Int64", "@value" : 100}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref MAP: TestCase = TestCase {
            serial: json!({ "@type" : "g:Map", "@value" : [ { "@type" : "g:Date", "@value" : 1481750076295u64 }, "red", { "@type" : "g:List", "@value" : [ { "@type" : "g:Int32", "@value" : 1 }, { "@type" : "g:Int32", "@value" : 2 }, { "@type" : "g:Int32", "@value" : 3 } ] }, { "@type" : "g:Date", "@value" : 1481750076295u64 }, "test", { "@type" : "g:Int32", "@value" : 123 } ]}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref SET: TestCase = TestCase {
            serial: json!({ "@type" : "g:Set", "@value" : [ { "@type" : "g:Int32", "@value" : 1u32 }, "person", true ]}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref TIMESTAMP: TestCase = TestCase {
            serial: json!({ "@type" : "g:Timestamp", "@value" : 1481750076295u64 }),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref UUID: TestCase = TestCase {
            serial: json!({ "@type" : "g:UUID", "@value" : "41d2e28a-20a4-4ab0-b379-d810dede3786"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref EDGE: TestCase = TestCase {
            serial: json!({ "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 13 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 1 }, "properties" : { "since" : { "@type" : "g:Property", "@value" : { "key" : "since", "value" : { "@type" : "g:Int32", "@value" : 2009 } } } } }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref PATH: TestCase = TestCase {
            serial: json!({ "@type" : "g:Path", "@value" : { "labels" : { "@type" : "g:List", "@value" : [ { "@type" : "g:Set", "@value" : [ ] }, { "@type" : "g:Set", "@value" : [ ] }, { "@type" : "g:Set", "@value" : [ ] } ] }, "objects" : { "@type" : "g:List", "@value" : [ { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person" } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 10 }, "label" : "software" } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 11 }, "label" : "software" } } ] } }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref PROPERTY: TestCase = TestCase {
            serial: json!({ "@type" : "g:Property", "@value" : { "key" : "since", "value" : { "@type" : "g:Int32", "@value" : 2009 } }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref TINKERGRAPH: TestCase = TestCase {
            serial: json!({ "@type" : "tinker:graph", "@value" : { "vertices" : [ { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 6 }, "value" : "san diego", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1997 }, "endTime" : { "@type" : "g:Int32", "@value" : 2001 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 7 }, "value" : "santa cruz", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2001 }, "endTime" : { "@type" : "g:Int32", "@value" : 2004 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 8 }, "value" : "brussels", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 9 }, "value" : "santa fe", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 } } } } ] } } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 7 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 1 }, "value" : "stephen", "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 10 }, "value" : "centreville", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1990 }, "endTime" : { "@type" : "g:Int32", "@value" : 2000 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 11 }, "value" : "dulles", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2000 }, "endTime" : { "@type" : "g:Int32", "@value" : 2006 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 12 }, "value" : "purcellville", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2006 } } } } ] } } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 8 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 2 }, "value" : "matthias", "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 13 }, "value" : "bremen", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2007 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 14 }, "value" : "baltimore", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2007 }, "endTime" : { "@type" : "g:Int32", "@value" : 2011 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 15 }, "value" : "oakland", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2011 }, "endTime" : { "@type" : "g:Int32", "@value" : 2014 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 16 }, "value" : "seattle", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2014 } } } } ] } } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 9 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 3 }, "value" : "daniel", "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 17 }, "value" : "spremberg", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1982 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 18 }, "value" : "kaiserslautern", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 }, "endTime" : { "@type" : "g:Int32", "@value" : 2009 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 19 }, "value" : "aachen", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2009 } } } } ] } } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 10 }, "label" : "software", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 4 }, "value" : "gremlin", "label" : "name" } } ] } } }, { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 11 }, "label" : "software", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 5 }, "value" : "tinkergraph", "label" : "name" } } ] } } } ], "edges" : [ { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 13 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 1 }, "properties" : { "since" : { "@type" : "g:Property", "@value" : { "key" : "since", "value" : { "@type" : "g:Int32", "@value" : 2009 } } } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 14 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 1 }, "properties" : { "since" : { "@type" : "g:Property", "@value" : { "key" : "since", "value" : { "@type" : "g:Int32", "@value" : 2010 } } } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 15 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 1 }, "properties" : { "skill" : { "@type" : "g:Property", "@value" : { "key" : "skill", "value" : { "@type" : "g:Int32", "@value" : 4 } } } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 16 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 1 }, "properties" : { "skill" : { "@type" : "g:Property", "@value" : { "key" : "skill", "value" : { "@type" : "g:Int32", "@value" : 5 } } } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 17 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 7 }, "properties" : { "since" : { "@type" : "g:Property", "@value" : { "key" : "since", "value" : { "@type" : "g:Int32", "@value" : 2010 } } } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 18 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 7 }, "properties" : { "since" : { "@type" : "g:Property", "@value" : { "key" : "since", "value" : { "@type" : "g:Int32", "@value" : 2011 } } } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 19 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 7 }, "properties" : { "skill" : { "@type" : "g:Property", "@value" : { "key" : "skill", "value" : { "@type" : "g:Int32", "@value" : 5 } } } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 20 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 7 }, "properties" : { "skill" : { "@type" : "g:Property", "@value" : { "key" : "skill", "value" : { "@type" : "g:Int32", "@value" : 4 } } } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 21 }, "label" : "develops", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 8 }, "properties" : { "since" : { "@type" : "g:Property", "@value" : { "key" : "since", "value" : { "@type" : "g:Int32", "@value" : 2012 } } } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 22 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 8 }, "properties" : { "skill" : { "@type" : "g:Property", "@value" : { "key" : "skill", "value" : { "@type" : "g:Int32", "@value" : 3 } } } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 23 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 8 }, "properties" : { "skill" : { "@type" : "g:Property", "@value" : { "key" : "skill", "value" : { "@type" : "g:Int32", "@value" : 3 } } } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 24 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 10 }, "outV" : { "@type" : "g:Int32", "@value" : 9 }, "properties" : { "skill" : { "@type" : "g:Property", "@value" : { "key" : "skill", "value" : { "@type" : "g:Int32", "@value" : 5 } } } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 25 }, "label" : "uses", "inVLabel" : "software", "outVLabel" : "person", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 9 }, "properties" : { "skill" : { "@type" : "g:Property", "@value" : { "key" : "skill", "value" : { "@type" : "g:Int32", "@value" : 3 } } } } } }, { "@type" : "g:Edge", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 26 }, "label" : "traverses", "inVLabel" : "software", "outVLabel" : "software", "inV" : { "@type" : "g:Int32", "@value" : 11 }, "outV" : { "@type" : "g:Int32", "@value" : 10 } } } ] }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref VERTEX: TestCase = TestCase {
            serial: json!({ "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 6 }, "value" : "san diego", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1997 }, "endTime" : { "@type" : "g:Int32", "@value" : 2001 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 7 }, "value" : "santa cruz", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2001 }, "endTime" : { "@type" : "g:Int32", "@value" : 2004 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 8 }, "value" : "brussels", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 9 }, "value" : "santa fe", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 } } } } ] } }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref VERTEXPROPERTY: TestCase = TestCase {
            serial: json!({ "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "label" : "name" }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref BARRIER: TestCase = TestCase {
            serial: json!({ "@type" : "g:Barrier", "@value" : "normSack"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref BINDING: TestCase = TestCase {
            serial: json!({ "@type" : "g:Binding", "@value" : { "key" : "x", "value" : { "@type" : "g:Int32", "@value" : 1 } }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref BULKSET: TestCase = TestCase {
            serial: json!({ "@type" : "g:BulkSet", "@value" : [ "marko", { "@type" : "g:Int64", "@value" : 1 }, "josh", { "@type" : "g:Int64", "@value" : 2 } ]}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref BYTECODE: TestCase = TestCase {
            serial: json!({ "@type" : "g:Bytecode", "@value" : { "step" : [ [ "V" ], [ "hasLabel", "person" ], [ "out" ], [ "in" ], [ "tree" ] ] }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref CARDINALITY: TestCase = TestCase {
            serial: json!({ "@type" : "g:Cardinality", "@value" : "list"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref COLUMN: TestCase = TestCase {
            serial: json!({ "@type" : "g:Column", "@value" : "keys"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref DIRECTION: TestCase = TestCase {
            serial: json!({ "@type" : "g:Direction", "@value" : "OUT"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref OPERATOR: TestCase = TestCase {
            serial: json!({ "@type" : "g:Operator", "@value" : "sum"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref ORDER: TestCase = TestCase {
            serial: json!({ "@type" : "g:Order", "@value" : "shuffle"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref PICK: TestCase = TestCase {
            serial: json!({ "@type" : "g:Pick", "@value" : "any"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref POP: TestCase = TestCase {
            serial: json!({ "@type" : "g:Pop", "@value" : "all"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref LAMBDA: TestCase = TestCase {
            serial: json!({ "@type" : "g:Lambda", "@value" : { "script" : "{ it.get() }", "language" : "gremlin-groovy", "arguments" : 1 }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref METRICS: TestCase = TestCase {
            serial: json!({ "@type" : "g:Metrics", "@value" : { "@type" : "g:Map", "@value" : [ "dur", { "@type" : "g:Double", "@value" : 100.0 }, "counts", { "@type" : "g:Map", "@value" : [ "traverserCount", { "@type" : "g:Int64", "@value" : 4 }, "elementCount", { "@type" : "g:Int64", "@value" : 4 } ] }, "name", "TinkerGraphStep(vertex,[~label.eq(person)])", "annotations", { "@type" : "g:Map", "@value" : [ "percentDur", { "@type" : "g:Double", "@value" : 25.0 } ] }, "id", "7.0.0()", "metrics", { "@type" : "g:List", "@value" : [ { "@type" : "g:Metrics", "@value" : { "@type" : "g:Map", "@value" : [ "dur", { "@type" : "g:Double", "@value" : 100.0 }, "counts", { "@type" : "g:Map", "@value" : [ "traverserCount", { "@type" : "g:Int64", "@value" : 7 }, "elementCount", { "@type" : "g:Int64", "@value" : 7 } ] }, "name", "VertexStep(OUT,vertex)", "annotations", { "@type" : "g:Map", "@value" : [ "percentDur", { "@type" : "g:Double", "@value" : 25.0 } ] }, "id", "3.0.0()" ] } } ] } ] }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref P: TestCase = TestCase {
            serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "gt", "value" : { "@type" : "g:Int32", "@value" : 0 } }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref P_WITHIN: TestCase = TestCase {
            serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "within", "value" : [ { "@type" : "g:Int32", "@value" : 1 } ] }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref P_WITHOUT: TestCase = TestCase {
            serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "without", "value" : [ { "@type" : "g:Int32", "@value" : 1 }, { "@type" : "g:Int32", "@value" : 2 } ] }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref P_AND: TestCase = TestCase {
            serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "and", "value" : [ { "@type" : "g:P", "@value" : { "predicate" : "gt", "value" : { "@type" : "g:Int32", "@value" : 0 } } }, { "@type" : "g:P", "@value" : { "predicate" : "lt", "value" : { "@type" : "g:Int32", "@value" : 10 } } } ] }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref P_OR: TestCase = TestCase {
            serial: json!({ "@type" : "g:P", "@value" : { "predicate" : "or", "value" : [ { "@type" : "g:P", "@value" : { "predicate" : "gt", "value" : { "@type" : "g:Int32", "@value" : 0 } } }, { "@type" : "g:P", "@value" : { "predicate" : "within", "value" : { "@type" : "g:List", "@value" : [ { "@type" : "g:Int32", "@value" : -1 }, { "@type" : "g:Int32", "@value" : -10 }, { "@type" : "g:Int32", "@value" : -100 } ] } } } ] }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref SCOPE: TestCase = TestCase {
            serial: json!({ "@type" : "g:Scope", "@value" : "local"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref T: TestCase = TestCase {
            serial: json!({ "@type" : "g:T", "@value" : "label"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref TEXTP: TestCase = TestCase {
            serial: json!({ "@type" : "g:TextP", "@value" : { "predicate" : "containing", "value" : "ark" }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref TRAVERSALMETRICS: TestCase = TestCase {
            serial: json!({ "@type" : "g:TraversalMetrics", "@value" : { "@type" : "g:Map", "@value" : [ "dur", { "@type" : "g:Double", "@value" : 0.004 }, "metrics", { "@type" : "g:List", "@value" : [ { "@type" : "g:Metrics", "@value" : { "@type" : "g:Map", "@value" : [ "dur", { "@type" : "g:Double", "@value" : 100.0 }, "counts", { "@type" : "g:Map", "@value" : [ "traverserCount", { "@type" : "g:Int64", "@value" : 4 }, "elementCount", { "@type" : "g:Int64", "@value" : 4 } ] }, "name", "TinkerGraphStep(vertex,[~label.eq(person)])", "annotations", { "@type" : "g:Map", "@value" : [ "percentDur", { "@type" : "g:Double", "@value" : 25.0 } ] }, "id", "7.0.0()" ] } }, { "@type" : "g:Metrics", "@value" : { "@type" : "g:Map", "@value" : [ "dur", { "@type" : "g:Double", "@value" : 100.0 }, "counts", { "@type" : "g:Map", "@value" : [ "traverserCount", { "@type" : "g:Int64", "@value" : 13 }, "elementCount", { "@type" : "g:Int64", "@value" : 13 } ] }, "name", "VertexStep(OUT,vertex)", "annotations", { "@type" : "g:Map", "@value" : [ "percentDur", { "@type" : "g:Double", "@value" : 25.0 } ] }, "id", "2.0.0()" ] } }, { "@type" : "g:Metrics", "@value" : { "@type" : "g:Map", "@value" : [ "dur", { "@type" : "g:Double", "@value" : 100.0 }, "counts", { "@type" : "g:Map", "@value" : [ "traverserCount", { "@type" : "g:Int64", "@value" : 7 }, "elementCount", { "@type" : "g:Int64", "@value" : 7 } ] }, "name", "VertexStep(OUT,vertex)", "annotations", { "@type" : "g:Map", "@value" : [ "percentDur", { "@type" : "g:Double", "@value" : 25.0 } ] }, "id", "3.0.0()" ] } }, { "@type" : "g:Metrics", "@value" : { "@type" : "g:Map", "@value" : [ "dur", { "@type" : "g:Double", "@value" : 100.0 }, "counts", { "@type" : "g:Map", "@value" : [ "traverserCount", { "@type" : "g:Int64", "@value" : 1 }, "elementCount", { "@type" : "g:Int64", "@value" : 1 } ] }, "name", "TreeStep", "annotations", { "@type" : "g:Map", "@value" : [ "percentDur", { "@type" : "g:Double", "@value" : 25.0 } ] }, "id", "4.0.0()" ] } } ] } ] }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref TRAVERSER: TestCase = TestCase {
            serial: json!({ "@type" : "g:Traverser", "@value" : { "bulk" : { "@type" : "g:Int64", "@value" : 1 }, "value" : { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 6 }, "value" : "san diego", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1997 }, "endTime" : { "@type" : "g:Int32", "@value" : 2001 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 7 }, "value" : "santa cruz", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2001 }, "endTime" : { "@type" : "g:Int32", "@value" : 2004 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 8 }, "value" : "brussels", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 9 }, "value" : "santa fe", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 } } } } ] } } } }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref AUTHENTICATION_RESPONSE: TestCase = TestCase {
            serial: json!({ "requestId" : "cb682578-9d92-4499-9ebc-5c6aa73c5397", "op" : "authentication", "processor" : "", "args" : { "@type" : "g:Map", "@value" : [ "saslMechanism", "PLAIN", "sasl", "AHN0ZXBocGhlbgBwYXNzd29yZA==" ] }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref SESSION_EVAL: TestCase = TestCase {
            serial: json!({ "requestId" : "cb682578-9d92-4499-9ebc-5c6aa73c5397", "op" : "eval", "processor" : "session", "args" : { "@type" : "g:Map", "@value" : [ "gremlin", "g.V(x)", "language", "gremlin-groovy", "session", { "@type" : "g:UUID", "@value" : "41d2e28a-20a4-4ab0-b379-d810dede3786" }, "bindings", { "@type" : "g:Map", "@value" : [ "x", { "@type" : "g:Int32", "@value" : 1 } ] } ] }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref SESSION_EVAL_ALIASED: TestCase = TestCase {
            serial: json!({ "requestId" : "cb682578-9d92-4499-9ebc-5c6aa73c5397", "op" : "eval", "processor" : "session", "args" : { "@type" : "g:Map", "@value" : [ "gremlin", "social.V(x)", "language", "gremlin-groovy", "aliases", { "@type" : "g:Map", "@value" : [ "g", "social" ] }, "session", { "@type" : "g:UUID", "@value" : "41d2e28a-20a4-4ab0-b379-d810dede3786" }, "bindings", { "@type" : "g:Map", "@value" : [ "x", { "@type" : "g:Int32", "@value" : 1 } ] } ] }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref SESSION_CLOSE: TestCase = TestCase {
            serial: json!({ "requestId" : "cb682578-9d92-4499-9ebc-5c6aa73c5397", "op" : "close", "processor" : "session", "args" : { "@type" : "g:Map", "@value" : [ "session", { "@type" : "g:UUID", "@value" : "41d2e28a-20a4-4ab0-b379-d810dede3786" } ] }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref SESSIONLESS_EVAL: TestCase = TestCase {
            serial: json!({ "requestId" : "cb682578-9d92-4499-9ebc-5c6aa73c5397", "op" : "eval", "processor" : "", "args" : { "@type" : "g:Map", "@value" : [ "gremlin", "g.V(x)", "language", "gremlin-groovy", "bindings", { "@type" : "g:Map", "@value" : [ "x", { "@type" : "g:Int32", "@value" : 1 } ] } ] }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref SESSIONLESS_EVAL_ALIASED: TestCase = TestCase {
            serial: json!({ "requestId" : "cb682578-9d92-4499-9ebc-5c6aa73c5397", "op" : "eval", "processor" : "", "args" : { "@type" : "g:Map", "@value" : [ "gremlin", "social.V(x)", "language", "gremlin-groovy", "aliases", { "@type" : "g:Map", "@value" : [ "g", "social" ] }, "bindings", { "@type" : "g:Map", "@value" : [ "x", { "@type" : "g:Int32", "@value" : 1 } ] } ] }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref AUTHENTICATION_CHALLENGE: TestCase = TestCase {
            serial: json!({ "requestId" : "41d2e28a-20a4-4ab0-b379-d810dede3786", "status" : { "message" : "", "code" : 407, "attributes" : { "@type" : "g:Map", "@value" : [ ] } }, "result" : { "data" : null, "meta" : { "@type" : "g:Map", "@value" : [ ] } }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref STANDARD_RESULT: TestCase = TestCase {
            serial: json!({ "requestId" : "41d2e28a-20a4-4ab0-b379-d810dede3786", "status" : { "message" : "", "code" : 200, "attributes" : { "@type" : "g:Map", "@value" : [ ] } }, "result" : { "data" : { "@type" : "g:List", "@value" : [ { "@type" : "g:Vertex", "@value" : { "id" : { "@type" : "g:Int32", "@value" : 1 }, "label" : "person", "properties" : { "name" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 0 }, "value" : "marko", "label" : "name" } } ], "location" : [ { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 6 }, "value" : "san diego", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 1997 }, "endTime" : { "@type" : "g:Int32", "@value" : 2001 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 7 }, "value" : "santa cruz", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2001 }, "endTime" : { "@type" : "g:Int32", "@value" : 2004 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 8 }, "value" : "brussels", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2004 }, "endTime" : { "@type" : "g:Int32", "@value" : 2005 } } } }, { "@type" : "g:VertexProperty", "@value" : { "id" : { "@type" : "g:Int64", "@value" : 9 }, "value" : "santa fe", "label" : "location", "properties" : { "startTime" : { "@type" : "g:Int32", "@value" : 2005 } } } } ] } } } ] }, "meta" : { "@type" : "g:Map", "@value" : [ ] } }}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref BIGDECIMAL: TestCase = TestCase {
            serial: json!({ "@type" : "gx:BigDecimal", "@value" : 123456789987654321123456789987654321u128 }),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref BIGINTEGER: TestCase = TestCase {
            serial: json!({ "@type" : "gx:BigInteger", "@value" : 123456789987654321123456789987654321u128 }),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref BYTE: TestCase = TestCase {
            serial: json!({ "@type" : "gx:Byte", "@value" : 1}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref BYTEBUFFER: TestCase = TestCase {
            serial: json!({ "@type" : "gx:ByteBuffer", "@value" : "c29tZSBieXRlcyBmb3IgeW91"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref CHAR: TestCase = TestCase {
            serial: json!({ "@type" : "gx:Char", "@value" : "x"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref DURATION: TestCase = TestCase {
            serial: json!({ "@type" : "gx:Duration", "@value" : "PT120H"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref INETADDRESS: TestCase = TestCase {
            serial: json!({ "@type" : "gx:InetAddress", "@value" : "localhost"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref INSTANT: TestCase = TestCase {
            serial: json!({ "@type" : "gx:Instant", "@value" : "2016-12-14T16:39:19.349Z"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref LOCALDATE: TestCase = TestCase {
            serial: json!({ "@type" : "gx:LocalDate", "@value" : "2016-01-01"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref LOCALDATETIME: TestCase = TestCase {
            serial: json!({ "@type" : "gx:LocalDateTime", "@value" : "2016-01-01T12:30"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref LOCALTIME: TestCase = TestCase {
            serial: json!({ "@type" : "gx:LocalTime", "@value" : "12:30:45"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref MONTHDAY: TestCase = TestCase {
            serial: json!({ "@type" : "gx:MonthDay", "@value" : "--01-01"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref OFFSETDATETIME: TestCase = TestCase {
            serial: json!({ "@type" : "gx:OffsetDateTime", "@value" : "2007-12-03T10:15:30+01:00"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref OFFSETTIME: TestCase = TestCase {
            serial: json!({ "@type" : "gx:OffsetTime", "@value" : "10:15:30+01:00"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref PERIOD: TestCase = TestCase {
            serial: json!({ "@type" : "gx:Period", "@value" : "P1Y6M15D"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref SHORT: TestCase = TestCase {
            serial: json!({ "@type" : "gx:Int16", "@value" : 100}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref YEAR: TestCase = TestCase {
            serial: json!({ "@type" : "gx:Year", "@value" : "2016"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref YEARMONTH: TestCase = TestCase {
            serial: json!({ "@type" : "gx:YearMonth", "@value" : "2016-06"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref ZONEDDATETIME: TestCase = TestCase {
            serial: json!({ "@type" : "gx:ZonedDateTime", "@value" : "2016-12-23T12:12:24.000000036+02:00[GMT+02:00]"}),
            object: GValue::Null,
        };
    }

    lazy_static::lazy_static! {
        pub static ref ZONEOFFSET: TestCase = TestCase {
            serial: json!({ "@type" : "gx:ZoneOffset", "@value" : "+03:06:09"}),
            object: GValue::Null,
        };
    }
}
