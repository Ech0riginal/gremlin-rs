//! GraphSON V3 [docs](http://tinkerpop.apache.org/docs/3.4.1/dev/io/)

use crate::io::serde::v3::types::*;
use crate::io::serde::GraphSONDeserializer;
use crate::io::{V2, V3};
use crate::prelude::*;
use serde_json::Value;
use std::collections::HashMap;

pub(crate) use crate::io::serde::v2::de::*;

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
                    LIST => list::<Self>(&value["@value"]),
                    MAP => map::<Self>(&value["@value"]),
                    PATH => path::<Self>(&value["@value"]),
                    METRICS => metrics::<Self>(&value["@value"]),
                    TRAVERSAL_METRICS => traversal_metrics::<Self>(&value["@value"]),
                    SET => set::<Self>(&value["@value"]),
                    BULK_SET => bulkset::<Self>(&value["@value"]),
                    _ => V2::deserialize(value),
                }
            }
        }
    }
}

/// List deserializer [docs](http://tinkerpop.apache.org/docs/3.4.1/dev/io/#_list)
pub(crate) fn list<D: GraphSONDeserializer>(val: &Value) -> GremlinResult<GValue> {
    if val.to_string().contains("[null]") {
        // TODO Speak to the sKG lads about this
        return Ok(GValue::List(List::new(vec![])));
    }
    let val = get_value!(val, Value::Array)?;
    let _debug_val = val.iter().map(|v| format!("{:?}", v)).collect::<Vec<_>>();

    let mut elements = Vec::with_capacity(val.len());
    for item in val {
        let deserialized = D::deserialize(item)?;
        elements.push(deserialized);
    }
    Ok(elements.into())
}

/// Map deserializer [docs](http://tinkerpop.apache.org/docs/3.4.1/dev/io/#_map)
pub(crate) fn map<D: GraphSONDeserializer>(val: &Value) -> GremlinResult<GValue> {
    let val = get_value!(val, Value::Array)?;
    let mut map = HashMap::new();
    if !val.is_empty() {
        let mut x = 0;
        while x < val.len() {
            let key_value = D::deserialize(&val[x])?;
            let key: GKey = FromGValue::from_gvalue(key_value)?;
            let vald = &val[x + 1];
            let _debug_val = format!("{}", &vald);
            let value = D::deserialize(vald)?;
            map.insert(key, value);
            x += 2;
        }
    }
    Ok(map.into())
}

/// Bulkset deserializer [docs](https://tinkerpop.apache.org/docs/3.4.1/dev/io/#_bulkset)
pub(crate) fn bulkset<D: GraphSONDeserializer>(val: &Value) -> GremlinResult<GValue> {
    if val.to_string().contains("[null]") {
        // TODO Gremlin docs!
        return Ok(GValue::List(List::new(vec![])));
    }

    let val = get_value!(val, Value::Array)?;

    if val.len() % 2 != 0 {
        return Err(GremlinError::Cast(
            "Cannot construct BulkSet from value.".to_string(),
        ));
    }

    let mut data = vec![];
    let mut counts = vec![];

    for gval in val.clone().into_iter() {
        if data.len() > counts.len() {
            counts.push(D::deserialize(&gval)?);
        } else {
            data.push(D::deserialize(&gval)?);
        }
    }

    let hashmap = data
        .clone()
        .into_iter()
        .flat_map(|val| {
            let key_opt = match val {
                GValue::Map(ref map) => match map.get("id") {
                    Some(GValue::Int64(i)) => Some(GKey::String(i.to_string())),
                    Some(GValue::Int32(i)) => Some(GKey::String(i.to_string())),
                    Some(GValue::String(s)) => Some(GKey::String(s.to_string())),
                    _ => None,
                },
                _ => None,
            };
            // TODO very bad practice but it works so .. :D
            if key_opt.is_some() {
                Some((key_opt.unwrap(), val))
            } else {
                None
            }
        })
        .collect::<HashMap<GKey, GValue>>();

    Ok(GValue::BulkSet(Map::from(hashmap)))
}

/// Traversal Metrics deserializer [docs](http://tinkerpop.apache.org/docs/3.4.1/dev/io/#_traversalmetrics)
pub(crate) fn traversal_metrics<D: GraphSONDeserializer>(val: &Value) -> GremlinResult<GValue> {
    let mut metrics = D::deserialize(&val)?.take::<Map>()?;

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

/// Metrics deserializer [docs](http://tinkerpop.apache.org/docs/3.4.1/dev/io/#_metrics)
pub(crate) fn metrics<D: GraphSONDeserializer>(val: &Value) -> GremlinResult<GValue> {
    let mut metric = D::deserialize(&val)?.take::<Map>()?;

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
