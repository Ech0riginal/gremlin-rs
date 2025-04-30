pub(crate) mod de;
pub(crate) mod ser;
pub(crate) mod types;

// #[cfg(test)]
// mod tests;

graphson!(V3);

mod impls {
    use super::*;
    use crate::io::serde::types::{v2, v3};
    use crate::io::serde::{
        ContentType, GraphSONDeserializer, GraphSONSerializer, MessageHandler, V2,
    };
    use crate::prelude::{GValue, GremlinError, GremlinResult};
    use serde_json::Value;

    impl ContentType for V3 {
        fn content_type() -> &'static str {
            "application/vnd.gremlin-v3.0+json"
        }
    }

    impl MessageHandler for V3 {
        fn message<T>(
            op: String,
            processor: String,
            args: T,
            id: Option<uuid::Uuid>,
        ) -> crate::prelude::Message<T> {
            let request_id = id.unwrap_or_else(uuid::Uuid::new_v4);
            crate::prelude::Message::V3 {
                request_id,
                op,
                processor,
                args,
            }
        }
    }

    impl GraphSONDeserializer for V3 {
        fn deserialize(value: &Value) -> GremlinResult<GValue> {
            match value {
                Value::Bool(_) | Value::String(_) => V2::deserialize(value),
                _ => {
                    let _type = match &value["@type"] {
                        Value::String(e) => Ok(e),
                        _type => Err(GremlinError::Json(format!("Unexpected type: {:?}", _type))),
                    }?;

                    match _type.as_str() {
                        v2::G_LIST => de::list::<Self>(&value["@value"]),
                        v2::G_MAP => de::map::<Self>(&value["@value"]),
                        v2::G_PATH => de::path::<Self>(&value["@value"]),
                        v2::G_METRICS => de::metrics::<Self>(&value["@value"]),
                        v2::G_TRAVERSAL_METRICS => de::traversal_metrics::<Self>(&value["@value"]),
                        v3::G_SET => de::set::<Self>(&value["@value"]),
                        v3::G_BULKSET => de::bulkset::<Self>(&value["@value"]),
                        _ => V2::deserialize(value),
                    }
                }
            }
        }
    }

    impl GraphSONSerializer for V3 {
        fn serialize(value: &GValue) -> GremlinResult<Value> {
            match value {
                GValue::List(_) => ser::list::<Self>(value),
                GValue::Set(_) => ser::set::<Self>(value),
                GValue::Map(_) => ser::map::<Self>(value),
                GValue::P(_) => ser::p::<Self>(value),
                GValue::Bytecode(_) => ser::bytecode::<Self>(value),
                GValue::Vertex(_) => ser::vertex::<Self>(value),
                GValue::VertexProperty(_) => ser::vertex_property::<Self>(value),
                GValue::Edge(_) => ser::edge::<Self>(value),
                GValue::TextP(_) => ser::text_p::<Self>(value),
                GValue::Path(_) => ser::path::<Self>(value),
                GValue::Merge(_) => ser::merge(value),
                GValue::T(_) => ser::t(value),
                _ => V2::serialize(value),
            }
        }
    }
}
