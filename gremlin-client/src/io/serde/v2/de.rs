use crate::io::serde::v2::types::*;
use crate::io::serde::v2::{de, types};
use crate::io::{GraphSONDeserializer, V2};
use crate::prelude::{
    Edge, GKey, GValue, GremlinError, GremlinResult, IntermediateRepr, Map, Metric, Path, Property,
    Token, TraversalExplanation, TraversalMetrics, Traverser, Vertex, VertexProperty, GID,
};
use crate::structure::List;
use chrono::{TimeZone, Utc};
use serde_json::Value;
use std::collections::HashMap;

impl GraphSONDeserializer for V2 {
    fn deserialize(value: &Value) -> GremlinResult<GValue> {
        let _debug = format!("{}", value);

        match value {
            Value::String(string) => Ok(string.into()),
            Value::Number(_) => g64::<Self>(value),
            Value::Object(obj) => {
                let _obj_debug = format!("{:?}", obj);
                if obj.contains_key("@type") {
                    let _type = obj.get("@type").unwrap();
                    let inner_value = obj
                        .get("@value")
                        .ok_or_else(|| GremlinError::Generic("Value missing".to_string()))?;

                    match _type {
                        Value::String(tyype) => match tyype.as_str() {
                            CLASS => class::<Self>(inner_value),
                            types::core::INT => de::g32::<Self>(inner_value),
                            types::core::LONG => de::g64::<Self>(inner_value),
                            types::core::FLOAT => de::float32::<Self>(inner_value),
                            types::core::DOUBLE => de::float64::<Self>(inner_value),
                            // types::core::STRING => de::string(value),
                            types::core::DATE => de::date::<Self>(inner_value),
                            types::core::TIMESTAMP => de::timestamp::<Self>(inner_value),
                            types::core::UUID => de::uuid::<Self>(inner_value),
                            types::structure::EDGE => de::edge::<Self>(inner_value),
                            types::structure::PATH => de::path::<Self>(inner_value),
                            types::structure::PROPERTY => de::property::<Self>(inner_value),
                            types::structure::STAR_GRAPH => todo!("support"),
                            types::structure::TINKER_GRAPH => todo!("support"),
                            types::structure::TREE => todo!("support"),
                            types::structure::VERTEX => de::vertex::<Self>(inner_value),
                            types::structure::VERTEX_PROPERTY => {
                                de::vertex_property::<Self>(inner_value)
                            }
                            types::process::BARRIER => todo!("support"),
                            types::process::BINDING => todo!("support"),
                            types::process::BYTECODE => todo!("support"),
                            types::process::CARDINALITY => todo!("support"),
                            types::process::COLUMN => todo!("support"),
                            types::process::DIRECTION => de::direction(inner_value),
                            types::process::DT => todo!("support"),
                            types::process::LAMBDA => todo!("support"),
                            types::process::MERGE => todo!("support"),
                            types::process::METRICS => todo!("support"),
                            types::process::OPERATOR => todo!("support"),
                            types::process::ORDER => todo!("support"),
                            types::process::P => todo!("support"),
                            types::process::PICK => todo!("support"),
                            types::process::POP => todo!("support"),
                            types::process::SCOPE => todo!("support"),
                            types::process::T => de::token(inner_value),
                            types::process::TEXT_P => todo!("support"),
                            types::process::TRAVERSAL_METRICS => de::metrics::<Self>(inner_value),
                            types::process::TRAVERSER => de::traverser::<Self>(inner_value),

                            type_tag => Err({
                                let msg = format!("Unexpected type-tag `{type_tag}`");
                                GremlinError::Generic(msg)
                            }),
                        },

                        _ => Err(GremlinError::Generic("Malformed Object".to_string())),
                    }
                } else {
                    map::<Self>(value)
                }
            }
            Value::Array(values) => {
                let collection = values
                    .iter()
                    .map(Self::deserialize)
                    .collect::<Result<Vec<_>, GremlinError>>()?;
                Ok(GValue::List(List(collection)))
            }
            Value::Bool(_) => map::<Self>(value),
            Value::Null => Ok(GValue::Null),
        }
    }
}

/// Deserialize a JSON value to a GID
pub(crate) fn id<D: GraphSONDeserializer>(val: &Value) -> GremlinResult<GID> {
    match D::deserialize(val) {
        Ok(result) => match result {
            GValue::String(d) => Ok(GID::String(d)),
            GValue::Int32(d) => Ok(GID::Int32(d)),
            GValue::Int64(d) => Ok(GID::Int64(d)),
            _ => Err(GremlinError::Json(format!("{} cannot be an id", val))),
        },
        Err(e) => match e {
            GremlinError::Json(_e) => Ok(GID::String(val.to_string())),
            _ => Err(e),
        },
    }
}

pub(crate) fn class<D: GraphSONDeserializer>(val: &Value) -> GremlinResult<GValue> {
    let class = get_value!(val, Value::String)?;
    Ok(GValue::Class(class.into()))
}

/// Date deserializer [docs](http://tinkerpop.apache.org/docs/current/dev/io/#_date_2)
pub(crate) fn date<D: GraphSONDeserializer>(val: &Value) -> GremlinResult<GValue> {
    let val = expect_i64!(val);
    let datetime = Utc.timestamp_millis_opt(val).unwrap();
    Ok(GValue::Date(datetime))
}

pub(crate) fn timestamp<D: GraphSONDeserializer>(val: &Value) -> GremlinResult<GValue> {
    let val = expect_i64!(val);
    let datetime = Utc.timestamp_millis_opt(val).unwrap();
    Ok(GValue::Timestamp(datetime))
}

/// Integer deserializer [docs](http://tinkerpop.apache.org/docs/current/dev/io/#_integer_2)
pub(crate) fn g32<D: GraphSONDeserializer>(val: &Value) -> GremlinResult<GValue> {
    let val = expect_i32!(val);
    Ok(GValue::Int32(val))
}

/// Long deserializer [docs](http://tinkerpop.apache.org/docs/current/dev/io/#_long_2)
pub(crate) fn g64<D: GraphSONDeserializer>(val: &Value) -> GremlinResult<GValue> {
    let val = expect_i64!(val);
    Ok(GValue::Int64(val))
}

/// String deserializer [docs](http://tinkerpop.apache.org/docs/current/dev/io/#_long_2)
pub(crate) fn string<D: GraphSONDeserializer>(val: &Value) -> GremlinResult<GValue> {
    let val = match val {
        Value::String(str) => str.to_string(),
        _ => panic!("Invalid JSON"),
    };

    Ok(GValue::String(val))
}

/// UUID deserializer [docs](http://tinkerpop.apache.org/docs/current/dev/io/#_uuid_2)
pub(crate) fn uuid<D: GraphSONDeserializer>(val: &Value) -> GremlinResult<GValue> {
    let val = get_value!(val, Value::String)?;
    let uuid = uuid::Uuid::parse_str(&val)?;
    Ok(GValue::Uuid(uuid))
}

/// Float deserializer [docs](http://tinkerpop.apache.org/docs/current/dev/io/#_float_2)
pub(crate) fn float32<D: GraphSONDeserializer>(val: &Value) -> GremlinResult<GValue> {
    let val = expect_float!(val);
    Ok(GValue::Float(val))
}
/// Double deserializer [docs](http://tinkerpop.apache.org/docs/current/dev/io/#_double_2)
pub(crate) fn float64<D: GraphSONDeserializer>(val: &Value) -> GremlinResult<GValue> {
    let val = expect_double!(val);
    Ok(GValue::Double(val))
}

/// List deserializer [docs](http://tinkerpop.apache.org/docs/current/dev/io/#_list)
pub(crate) fn list<D: GraphSONDeserializer>(val: &Value) -> GremlinResult<GValue> {
    let val = get_value!(val, Value::Array)?;
    let mut elements = Vec::with_capacity(val.len());
    for item in val {
        let _debug = format!("{}", item);
        let deserialization = D::deserialize(item);
        elements.push(deserialization?)
    }
    Ok(GValue::List(elements.into()))
}

/// Set deserializer [docs](http://tinkerpop.apache.org/docs/current/dev/io/#_set)
pub(crate) fn set<D: GraphSONDeserializer>(val: &Value) -> GremlinResult<GValue> {
    match list::<D>(val)? {
        GValue::List(List(values)) => Ok(GValue::Set(values.into())),
        _ => panic!("Infallible"),
    }
}

/// Map deserializer [docs](http://tinkerpop.apache.org/docs/current/dev/io/#_map)
pub(crate) fn map<D: GraphSONDeserializer>(val: &Value) -> GremlinResult<GValue> {
    let val = get_value!(val, Value::Object)?;
    let mut map = HashMap::new();
    for (k, v) in val {
        map.insert(GKey::String(k.to_string()), D::deserialize(v)?);
    }
    Ok(map.into())
}

/// Token deserializer [docs](http://tinkerpop.apache.org/docs/current/dev/io/#_t_2)
pub(crate) fn token(val: &Value) -> GremlinResult<GValue> {
    let val = get_value!(val, Value::String)?;
    let token = Token::new(val.clone());
    Ok(GValue::Token(token))
}

/// https://tinkerpop.apache.org/docs/current/dev/io/#_direction
pub(crate) fn direction(val: &Value) -> GremlinResult<GValue> {
    let val = get_value!(val, Value::String)?;
    match val.as_str() {
        "OUT" => Ok(GValue::Direction(crate::structure::Direction::Out)),
        "IN" => Ok(GValue::Direction(crate::structure::Direction::In)),
        other => Err(GremlinError::Cast(format!(
            "Unknown direction literal {other}"
        ))),
    }
}

/// Vertex deserializer [docs](http://tinkerpop.apache.org/docs/current/dev/io/#_vertex_3)
pub(crate) fn vertex<D: GraphSONDeserializer>(val: &Value) -> GremlinResult<GValue> {
    let label = val
        .get("label")
        .map(|f| get_value!(f, Value::String).map(Clone::clone))
        .unwrap_or_else(|| Ok(String::from("vertex")))?;

    let id = id::<D>(&val["id"])?;

    Ok(Vertex::new(id, label, vertex_properties::<D>(&val["properties"])?).into())
}

/// Edge deserializer [docs](http://tinkerpop.apache.org/docs/current/dev/io/#_edge_3)
pub(crate) fn edge<D: GraphSONDeserializer>(val: &Value) -> GremlinResult<GValue> {
    let label = val
        .get("label")
        .map(|f| get_value!(f, Value::String).map(Clone::clone))
        .unwrap_or_else(|| Ok(String::from("edge")))?;

    let edge_id = id::<D>(&val["id"])?;

    let in_v_id = id::<D>(&val["inV"])?;
    // This is intentional, there is no clear guidance on the discrepancies in 2.0.
    // let in_v_label = get_value!(&val["inVLabel"], Value::String)?.clone();
    let in_v_label = val
        .get("inVLabel")
        .map(|label| {
            get_value!(&val["inVLabel"], Value::String)
                .map(Clone::clone)
                .unwrap()
        })
        .unwrap_or("Unavailable".into());

    let out_v_id = id::<D>(&val["outV"])?;
    // If we don't account for it we can't ser/de Property types.
    let out_v_label = val
        .get("outVLabel")
        .map(|label| {
            get_value!(&val["inVLabel"], Value::String)
                .map(Clone::clone)
                .unwrap()
        })
        .unwrap_or("Unavailable".into());
    Ok(Edge::new(
        edge_id,
        label,
        in_v_id,
        in_v_label,
        out_v_id,
        out_v_label,
        HashMap::new(),
    )
    .into())
}

/// Path deserializer [docs](http://tinkerpop.apache.org/docs/current/dev/io/#_path_3)
pub(crate) fn path<D: GraphSONDeserializer>(val: &Value) -> GremlinResult<GValue> {
    let labels = D::deserialize(&val["labels"])?;

    let objects = D::deserialize(&val["objects"])?;

    Ok(Path::new(labels, objects).into())
}

/// Traversal Metrics deserializer [docs](http://tinkerpop.apache.org/docs/current/dev/io/#_traversalmetrics)
pub(crate) fn traversal_metrics<D: GraphSONDeserializer>(val: &Value) -> GremlinResult<GValue> {
    let mut metrics = D::deserialize(val)?.take::<Map>()?;

    let duration = remove_or_else(&mut metrics, "dur", TRAVERSAL_METRICS)?.take::<f64>()?;

    let m = remove_or_else(&mut metrics, "metrics", TRAVERSAL_METRICS)?
        .take::<List>()?
        .take()
        .drain(0..)
        .map(|e| e.take::<Metric>())
        .filter_map(Result::ok)
        .collect();

    Ok(TraversalMetrics::new(duration, m).into())
}

/// Metrics deserializer [docs](http://tinkerpop.apache.org/docs/current/dev/io/#_metrics)
pub(crate) fn metrics<D: GraphSONDeserializer>(val: &Value) -> GremlinResult<GValue> {
    let mut metric = D::deserialize(val)?.take::<Map>()?;

    let duration = remove_or_else(&mut metric, "dur", METRICS)?.take::<f64>()?;
    let id = remove_or_else(&mut metric, "id", METRICS)?.take::<String>()?;
    let name = remove_or_else(&mut metric, "name", METRICS)?.take::<String>()?;

    let mut counts = remove_or_else(&mut metric, "counts", METRICS)?.take::<Map>()?;
    let traversers = remove_or_else(&mut counts, "traverserCount", METRICS)?.take::<i64>()?;
    let count = remove_or_else(&mut counts, "elementCount", METRICS)?.take::<i64>()?;

    let mut annotations = remove(&mut metric, "annotations", METRICS)
        .map(|e| e.take::<Map>())
        .unwrap_or_else(|| Ok(Map::empty()))?;

    let perc_duration = remove(&mut annotations, "percentDur", METRICS)
        .map(|e| e.take::<f64>())
        .unwrap_or_else(|| Ok(0.0))?;

    let nested: GremlinResult<Vec<Metric>> = remove(&mut metric, "metrics", METRICS)
        .map(|e| e.take::<List>())
        .unwrap_or_else(|| Ok(List::new(vec![])))?
        .take()
        .into_iter()
        .map(|e| e.take::<Metric>())
        .collect();

    Ok(Metric::new(
        id,
        name,
        duration,
        count,
        traversers,
        perc_duration,
        nested?,
    )
    .into())
}

pub(crate) fn explain<D: GraphSONDeserializer>(val: &Value) -> GremlinResult<GValue> {
    let mut explain = D::deserialize(val)?.take::<Map>()?;

    let original = remove_or_else(&mut explain, "original", TRAVERSAL_EXPLANATION)?
        .take::<List>()?
        .take()
        .drain(0..)
        .map(|s| s.take::<String>())
        .filter_map(Result::ok)
        .collect();

    let finals = remove_or_else(&mut explain, "final", TRAVERSAL_EXPLANATION)?
        .take::<List>()?
        .take()
        .drain(0..)
        .map(|s| s.take::<String>())
        .filter_map(Result::ok)
        .collect();

    let intermediate = remove_or_else(&mut explain, "intermediate", TRAVERSAL_EXPLANATION)?
        .take::<List>()?
        .take()
        .drain(0..)
        .map(|s| s.take::<Map>())
        .filter_map(Result::ok)
        .map(map_intermediate)
        .filter_map(Result::ok)
        .collect();

    Ok(TraversalExplanation::new(original, finals, intermediate).into())
}

/// Vertex Property deserializer [docs](http://tinkerpop.apache.org/docs/current/dev/io/#_vertexproperty_3)
pub(crate) fn vertex_property<D: GraphSONDeserializer>(val: &Value) -> GremlinResult<GValue> {
    let label = val
        .get("label")
        .map(|f| get_value!(f, Value::String).map(Clone::clone))
        .unwrap_or_else(|| Ok(String::from("vertex_property")))?;

    let id = id::<D>(&val["id"])?;
    let v = D::deserialize(&val["value"])?;
    let property = VertexProperty::new(id, label, v);
    Ok(property.into())
}

/// Property deserializer [docs](http://tinkerpop.apache.org/docs/current/dev/io/#_property_3)
pub(crate) fn property<D: GraphSONDeserializer>(val: &Value) -> GremlinResult<GValue> {
    let key = val
        .get("key")
        .map(|v| get_value!(v, Value::String).map(Clone::clone))
        .ok_or(GremlinError::Json("Missing Property 'key' key".into()))??;
    let value = val
        .get("value")
        .ok_or(GremlinError::Json("Missing Property 'element' key".into()))?;
    let element = val
        .get("element")
        .ok_or(GremlinError::Json("Missing Property 'element' key".into()))?;

    let value_obj = D::deserialize(&value)?;
    let element_obj = D::deserialize(&element)?;
    let property = Property {
        key: key,
        value: Box::new(value_obj),
        element: Box::new(element_obj),
    };
    Ok(GValue::Property(property))
}

/// Traverser deserializer [docs](http://tinkerpop.apache.org/docs/3.4.1/dev/io/#_traverser_2)
pub(crate) fn traverser<D: GraphSONDeserializer>(val: &Value) -> GremlinResult<GValue> {
    let bulk = D::deserialize(&val["bulk"])?.take::<i64>()?;
    let v = D::deserialize(&val["value"])?;
    Ok(Traverser::new(bulk, v).into())
}

pub(crate) fn vertex_properties<D: GraphSONDeserializer>(
    properties: &Value,
) -> GremlinResult<HashMap<String, Vec<VertexProperty>>> {
    match properties {
        Value::Object(o) => {
            let mut p = HashMap::new();
            for (k, v) in o {
                match v {
                    Value::Array(arr) => {
                        let mut vec = vec![];
                        for elem in arr {
                            vec.push(D::deserialize(elem)?.take()?);
                        }
                        p.insert(k.clone(), vec);
                    }
                    _ => {
                        return Err(GremlinError::Json(format!(
                            "Expected object or null for properties. Found {}",
                            properties
                        )));
                    }
                };
            }
            Ok(p)
        }

        Value::Null => Ok(HashMap::new()),
        _ => Err(GremlinError::Json(format!(
            "Expected object or null for properties. Found {}",
            properties
        ))),
    }
}

pub(crate) fn map_intermediate(mut m: Map) -> GremlinResult<IntermediateRepr> {
    let traversal = remove_or_else(&mut m, "traversal", TRAVERSAL_EXPLANATION)?
        .take::<List>()?
        .take()
        .drain(0..)
        .map(|s| s.take::<String>())
        .filter_map(Result::ok)
        .collect();

    let strategy = remove_or_else(&mut m, "strategy", TRAVERSAL_EXPLANATION)?.take::<String>()?;

    let category = remove_or_else(&mut m, "category", TRAVERSAL_EXPLANATION)?.take::<String>()?;

    Ok(IntermediateRepr::new(traversal, strategy, category))
}

pub(crate) fn remove_or_else(map: &mut Map, field: &str, owner: &str) -> GremlinResult<GValue> {
    remove(map, field, owner)
        .ok_or_else(|| GremlinError::Json(format!("Field {} not found in {}", field, owner)))
}

pub(crate) fn remove(map: &mut Map, field: &str, _owner: &str) -> Option<GValue> {
    map.remove(field)
}
