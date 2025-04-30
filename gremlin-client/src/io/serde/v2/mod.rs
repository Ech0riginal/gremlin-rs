pub(crate) mod de;
pub(crate) mod ser;
pub(crate) mod types;

// #[cfg(test)]
// mod tests;

graphson!(V2);

mod impls {
    use super::*;

    use crate::io::serde::MessageHandler;
    use crate::io::{ContentType, GraphSONDeserializer, GraphSONSerializer, V2};
    use crate::message::RequestIdV2;
    use crate::prelude::{GValue, GremlinError, GremlinResult, Message};
    use serde_json::Value;

    impl ContentType for V2 {
        fn content_type() -> &'static str {
            "application/vnd.gremlin-v2.0+json"
        }
    }

    impl MessageHandler for V2 {
        fn message<T>(
            op: String,
            processor: String,
            args: T,
            id: Option<uuid::Uuid>,
        ) -> Message<T> {
            let request_id = id.unwrap_or_else(uuid::Uuid::new_v4);

            Message::V2 {
                request_id: RequestIdV2 {
                    id_type: "g:UUID".to_string(),
                    value: request_id,
                },
                op,
                processor,
                args,
            }
        }
    }

    impl GraphSONDeserializer for V2 {
        fn deserialize(value: &Value) -> GremlinResult<GValue> {
            let _debug = format!("{}", value);

            match value {
                Value::String(string) => Ok(string.into()),
                Value::Number(_) => de::g64(value),
                Value::Object(obj) => {
                    let _obj_debug = format!("{:?}", obj);
                    if obj.contains_key("@type") {
                        let _type = obj.get("@type").unwrap();
                        let value = &obj
                            .get("@value")
                            .ok_or_else(|| GremlinError::Generic("Value missing".to_string()))?;

                        match _type {
                            Value::String(tyype) => match tyype.as_str() {
                                types::G_INT_32 => de::g32(value),
                                types::G_INT_64 => de::g64(value),
                                types::G_FLOAT => de::float32(value),
                                types::G_DOUBLE => de::float64(value),
                                types::G_STRING => de::string(value),
                                types::G_DATE => de::date(value),
                                types::G_UUID => de::uuid(value),
                                types::G_LIST => de::list::<Self>(value),
                                types::G_SET => de::set::<Self>(value),
                                types::G_MAP => de::map::<Self>(value),
                                types::G_T => de::token(value),
                                types::G_VERTEX => de::vertex::<Self>(value),
                                types::G_VERTEX_PROPERTY => de::vertex_property::<Self>(value),
                                types::G_PROPERTY => de::property::<Self>(value),
                                types::G_EDGE => de::edge::<Self>(value),
                                types::G_PATH => de::path::<Self>(value),
                                types::G_METRICS => de::traversal_metrics::<Self>(value),
                                types::G_TRAVERSAL_METRICS => de::metric::<Self>(value),
                                types::G_TRAVERSAL_EXPLANATION => de::explain::<Self>(value),
                                types::G_TRAVERSER => de::traverser::<Self>(value),
                                types::G_DIRECTION => de::direction(value),
                                &_ => Err(GremlinError::Generic("Unexpected Object".to_string())),
                            },

                            _ => Err(GremlinError::Generic("Malformed Object".to_string())),
                        }
                    } else {
                        de::map::<Self>(value)
                    }
                }
                Value::Array(_) => de::list::<Self>(value),
                Value::Bool(_) => de::map::<Self>(value),
                Value::Null => Ok(GValue::Null),
            }
        }
    }

    impl GraphSONSerializer for V2 {
        fn serialize(value: &GValue) -> GremlinResult<Value> {
            match value {
                GValue::Double(_) => ser::double(value),
                GValue::Float(_) => ser::float(value),
                GValue::Int32(_) => ser::int32(value),
                GValue::Int64(_) => ser::int64(value),
                GValue::String(_) => ser::string(value),
                GValue::Uuid(_) => ser::uuid(value),
                GValue::Date(_) => ser::date(value),
                GValue::List(_) => ser::list::<Self>(value),
                GValue::Set(_) => ser::set::<Self>(value),
                GValue::P(_) => ser::p::<Self>(value),
                GValue::Bytecode(_) => ser::bytecode::<Self>(value),
                GValue::Vertex(_) => ser::vertex::<Self>(value),
                GValue::VertexProperty(_) => ser::vertex_property::<Self>(value),
                GValue::Edge(_) => ser::edge::<Self>(value),
                GValue::Map(_) => ser::map::<Self>(value),
                GValue::T(_) => ser::t(value),
                GValue::Scope(_) => ser::scope(value),
                GValue::Order(_) => ser::order(value),
                GValue::Bool(_) => ser::bool(value),
                GValue::TextP(_) => ser::text_p::<Self>(value),
                GValue::Pop(_) => ser::pop(value),
                GValue::Cardinality(_) => ser::cardinality(value),
                GValue::Merge(_) => ser::merge(value),
                GValue::Direction(_) => ser::direction(value),
                GValue::Column(_) => ser::column(value),
                GValue::Path(_) => ser::path::<Self>(value),
                _ => panic!("Type {:?} not supported.", value),
            }
        }
    }
}
