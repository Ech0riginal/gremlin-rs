use crate::io::serde::types::v2::*;
use crate::io::serde::v2::ser;
use crate::io::{GraphSONSerializer, V2};
use crate::prelude::{
    traversal::{Order, Scope},
    Cardinality, Direction, GValue, GremlinError, GremlinResult, Merge, ToGValue, T,
};
use serde_json::{json, Map, Value};
use std::collections::HashMap;

impl GraphSONSerializer for V2 {
    fn serialize(value: &GValue) -> GremlinResult<Value> {
        match value {
            // Core
            GValue::Class(_) => class(value),
            GValue::Int32(_) => int32(value),
            GValue::Int64(_) => int64(value),
            GValue::Float(_) => float(value),
            GValue::Double(_) => double(value),
            GValue::String(_) => string(value),
            GValue::Date(_) => date(value),
            GValue::Timestamp(_) => timestamp(value),
            GValue::Uuid(_) => uuid(value),
            // Structure
            GValue::Edge(_) => edge::<Self>(value),
            GValue::Path(_) => path::<Self>(value),
            GValue::Property(_) => property::<Self>(value),
            GValue::StarGraph(_) => star_graph::<Self>(value),
            // GValue::TinkerGraph(_) => todo!("v2::tinkergraph"),
            // GValue::Tree(_) => todo!("v2::tree"),
            GValue::Vertex(_) => vertex::<Self>(value),
            GValue::VertexProperty(_) => vertex_property::<Self>(value),
            // Process
            // GValue::Barrier(_) => todo!("v2::barrier"),
            // GValue::Binding(_) => todo!("v2::binding"),
            GValue::Bytecode(_) => bytecode::<Self>(value),
            GValue::Cardinality(_) => cardinality(value),
            GValue::Column(_) => column(value),
            GValue::Direction(_) => direction(value),
            // GValue::DT(_) => todo!("v2::dt"),
            // GValue::Lambda(_) => todo!("v2::lambda"),
            GValue::Merge(_) => merge(value),
            // GValue::Metrics(_) => todo!("v2::metrics"),
            // GValue::Operator(_) => todo!("v2::operator"),
            GValue::Order(_) => order(value),
            GValue::P(_) => p::<Self>(value),
            // GValue::Pick(_) => todo!("v2::pick"),
            GValue::Pop(_) => pop(value),
            GValue::Scope(_) => scope(value),
            GValue::T(_) => t(value),
            GValue::TextP(_) => text_p::<Self>(value),
            GValue::TraversalMetrics(_) => todo!("v2::traversalmetrics"),
            GValue::Traverser(_) => todo!("v2::traverser"),

            // GValue::List(_) => list::<Self>(value),
            // GValue::Set(_) => set::<Self>(value),
            // GValue::P(_) => p::<Self>(value),

            // GValue::Map(_) => map::<Self>(value),
            // GValue::Bool(_) => bool(value),
            GValue::Null => Ok(serde_json::Value::Null),
            value => panic!("Unsupported type {:?}", value),
        }
    }
}

pub(crate) fn double(value: &GValue) -> GremlinResult<Value> {
    let double = get_value!(value, GValue::Double)?;
    Ok(json!({
        "@type" : DOUBLE,
        "@value" : double,
    }))
}

pub(crate) fn float(value: &GValue) -> GremlinResult<Value> {
    let float = get_value!(value, GValue::Float)?;
    Ok(json!({
        "@type" : FLOAT,
        "@value" : float,
    }))
}

pub(crate) fn class(value: &GValue) -> GremlinResult<Value> {
    let class = get_value!(value, GValue::Class)?;
    Ok(json!({
        "@type" : CLASS,
        "@value" : class,
    }))
}

pub(crate) fn int32(value: &GValue) -> GremlinResult<Value> {
    let int32 = get_value!(value, GValue::Int32)?;
    Ok(json!({
        "@type" : INT,
        "@value" : int32,
    }))
}

pub(crate) fn int64(value: &GValue) -> GremlinResult<Value> {
    let int64 = get_value!(value, GValue::Int64)?;
    Ok(json!({
        "@type" : LONG,
        "@value" : int64,
    }))
}

pub(crate) fn string(value: &GValue) -> GremlinResult<Value> {
    let string = get_value!(value, GValue::String)?;
    Ok(Value::String(string.clone()))
    // Ok(json!({
    //     "@type" : "g:String",
    //     "@value" : string,
    // }))
}

pub(crate) fn uuid(value: &GValue) -> GremlinResult<Value> {
    let uuid = get_value!(value, GValue::Uuid)?;
    Ok(json!({
        "@type" : UUID,
        "@value" : uuid.to_string()
    }))
}

pub(crate) fn date(value: &GValue) -> GremlinResult<Value> {
    let date = get_value!(value, GValue::Date)?;
    let millis = date.timestamp_millis();

    Ok(json!({
        "@type" : DATE,
        "@value" : millis
    }))
}

pub(crate) fn timestamp(value: &GValue) -> GremlinResult<Value> {
    let date = get_value!(value, GValue::Timestamp)?;
    let millis = date.timestamp_millis();
    Ok(json!({
        "@type" : TIMESTAMP,
        "@value" : millis
    }))
}

pub(crate) fn list<S: GraphSONSerializer>(value: &GValue) -> GremlinResult<Value> {
    let list = get_value!(value, GValue::List)?;
    let elements = list
        .iter()
        .map(|e| S::serialize(e))
        .collect::<GremlinResult<Vec<Value>>>()?;

    Ok(json!({
        "@type" : "g:List",
        "@value" : elements,
    }))
}

pub(crate) fn set<S: GraphSONSerializer>(value: &GValue) -> GremlinResult<Value> {
    let list = get_value!(value, GValue::Set)?;
    let elements = list
        .iter()
        .map(|e| S::serialize(e))
        .collect::<GremlinResult<Vec<Value>>>()?;

    Ok(json!({
        "@type" : "g:Set",
        "@value" : elements,
    }))
}

pub(crate) fn p<S: GraphSONSerializer>(value: &GValue) -> GremlinResult<Value> {
    let p = get_value!(value, GValue::P)?;
    Ok(json!({
        "@type" : P,
        "@value" : {
            "predicate" : p.operator(),
            "value" : S::serialize(p.value())?
        }
    }))
}

pub(crate) fn bytecode<S: GraphSONSerializer>(value: &GValue) -> GremlinResult<Value> {
    let code = get_value!(value, GValue::Bytecode)?;

    let steps: GremlinResult<Vec<Value>> = code
        .steps()
        .iter()
        .map(|m| {
            let mut instruction = vec![];
            instruction.push(Value::String(m.operator().clone()));

            let arguments: GremlinResult<Vec<Value>> =
                m.args().iter().map(|a| S::serialize(a)).collect();

            instruction.extend(arguments?);
            Ok(Value::Array(instruction))
        })
        .collect();

    let sources: GremlinResult<Vec<Value>> = code
        .sources()
        .iter()
        .map(|m| {
            let mut instruction = vec![];
            instruction.push(Value::String(m.operator().clone()));

            let arguments: GremlinResult<Vec<Value>> =
                m.args().iter().map(|a| S::serialize(a)).collect();

            instruction.extend(arguments?);
            Ok(Value::Array(instruction))
        })
        .collect();

    Ok(json!({
        "@type" : BYTECODE,
        "@value" : {
            "step" : steps?,
            "source" : sources?
        }
    }))
}

pub(crate) fn vertex<S: GraphSONSerializer>(value: &GValue) -> GremlinResult<Value> {
    let v = get_value!(value, GValue::Vertex)?;
    let id = S::serialize(&v.id().to_gvalue())?;
    let properties = v
        .iter()
        .map(|(label, properties)| {
            (
                label.clone(),
                properties
                    .into_iter()
                    .map(|p| (*p).clone())
                    .map(GValue::VertexProperty)
                    .flat_map(|p| S::serialize(&p))
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<HashMap<String, Vec<Value>>>();

    let value = if properties.is_empty() {
        json!({
            "@type" : VERTEX,
            "@value" : {
                "id" :  id,
                "label": v.label(),
            }
        })
    } else {
        json!({
            "@type" : VERTEX,
            "@value" : {
                "id" :  id,
                "label": v.label(),
                "properties": properties
            }
        })
    };

    Ok(value)
}

pub fn vertex_property<S: GraphSONSerializer>(value: &GValue) -> GremlinResult<Value> {
    let property = get_value!(value, GValue::VertexProperty)?;

    let blob = if let Some(vertex_id) = &property.vertex {
        json!({
            "@type" : VERTEX_PROPERTY,
            "@value" : {
                "id" : S::serialize(&property.id.to_gvalue())?,
                "value" : S::serialize(&property.value)?,
                "label" : S::serialize(&property.label.to_gvalue())?,
            },
            "vertex" : S::serialize(&vertex_id.to_gvalue())?,
        })
    } else {
        json!({
            "@type" : VERTEX_PROPERTY,
            "@value" : {
                "id" : S::serialize(&property.id.to_gvalue())?,
                "value" : S::serialize(&property.value)?,
                "label" : S::serialize(&property.label.to_gvalue())?,
            }
        })
    };

    Ok(blob)
}

pub(crate) fn edge<S: GraphSONSerializer>(value: &GValue) -> GremlinResult<Value> {
    let e = get_value!(value, GValue::Edge)?;

    let properties = e
        .iter()
        .map(|(label, property)| (label, S::serialize(&*property)))
        .filter(|(_, r)| r.is_ok())
        .map(|(p, r)| (p.clone(), r.unwrap()))
        .collect::<HashMap<String, Value>>();

    let value = if properties.is_empty() {
        json!({
            "@type" : EDGE,
            "@value" : {
                "id" :  S::serialize(&e.id().to_gvalue())?,
                "label": S::serialize(&e.label().to_gvalue())?,
                "inVLabel": S::serialize(&e.in_v.label().to_gvalue())?,
                "outVLabel": S::serialize(&e.out_v.label().to_gvalue())?,
                "inV": S::serialize(&e.in_v.id().to_gvalue())?,
                "outV": S::serialize(&e.out_v.id().to_gvalue())?,
            }
        })
    } else {
        json!({
            "@type" : EDGE,
            "@value" : {
                "id" :  S::serialize(&e.id().to_gvalue())?,
                "label": S::serialize(&e.label().to_gvalue())?,
                "inVLabel": S::serialize(&e.in_v.label().to_gvalue())?,
                "outVLabel": S::serialize(&e.out_v.label().to_gvalue())?,
                "inV": S::serialize(&e.in_v.id().to_gvalue())?,
                "outV": S::serialize(&e.out_v.id().to_gvalue())?,
                "properties": properties
            }
        })
    };

    Ok(value)
}

pub(crate) fn map<S: GraphSONSerializer>(value: &GValue) -> GremlinResult<Value> {
    let map = get_value!(value, GValue::Map)?;
    let mut params = Map::new();

    for (k, v) in map.iter() {
        let key = S::serialize(&k.clone().into())?
            .as_str()
            .ok_or_else(|| GremlinError::Generic(format!("Non-string key value for {:?}", k)))?
            .to_string();
        let value = S::serialize(&v)?;
        params.insert(key, value);
    }

    Ok(json!(params))
}
// deserialize_path
pub(crate) fn path<S: GraphSONSerializer>(value: &GValue) -> GremlinResult<Value> {
    let path = get_value!(value, GValue::Path)?;

    Ok(json!({
        "@type" : PATH,
        "@value": {
            "labels" : S::serialize(&path.labels)?,
            "objects" : S::serialize(&path.objects)?,
        }
    }))
}

pub(crate) fn property<S: GraphSONSerializer>(value: &GValue) -> GremlinResult<Value> {
    let property = get_value!(value, GValue::Property)?;

    Ok(json!({
        "@type": PROPERTY,
        "@value": {
            "key": property.key,
            "value": S::serialize(&*property.value)?,
            "element": S::serialize(&*property.element)?,
        }
    }))
}

pub(crate) fn star_graph<S: GraphSONSerializer>(value: &GValue) -> GremlinResult<Value> {
    let star = get_value!(value, GValue::StarGraph)?;
    let binding = GValue::Vertex(star.into());
    Ok(json!({
        "starVertex": vertex::<S>(&binding)?
    }))
}

pub(crate) fn t(value: &GValue) -> GremlinResult<Value> {
    let t = get_value!(value, GValue::T)?;
    let v = match t {
        T::Id => "id",
        T::Key => "key",
        T::Label => "label",
        T::Value => "value",
    };

    Ok(json!({
        "@type" : T,
        "@value" : v
    }))
}

pub(crate) fn scope(value: &GValue) -> GremlinResult<Value> {
    let s = get_value!(value, GValue::Scope)?;

    let v = match s {
        Scope::Global => "global",
        Scope::Local => "local",
    };

    Ok(json!({
        "@type" : SCOPE,
        "@value" : v
    }))
}

pub(crate) fn order(value: &GValue) -> GremlinResult<Value> {
    let order = get_value!(value, GValue::Order)?;

    let v = match order {
        Order::Asc => "asc",
        Order::Desc => "desc",
        Order::Shuffle => "shuffle",
    };

    Ok(json!({
        "@type" : ORDER,
        "@value" : v
    }))
}

pub(crate) fn bool(value: &GValue) -> GremlinResult<Value> {
    let b = get_value!(value, GValue::Bool)?;
    let string = match b {
        true => "true",
        false => "false",
    };
    Ok(serde_json::from_str(string).unwrap())
}

pub(crate) fn text_p<S: GraphSONSerializer>(value: &GValue) -> GremlinResult<Value> {
    let text_p = get_value!(value, GValue::TextP)?;
    Ok(json!({
        "@type" : TEXT_P,
        "@value" : {
            "predicate" : text_p.operator(),
            "value" : S::serialize(text_p.value())?
        }
    }))
}

pub(crate) fn pop(value: &GValue) -> GremlinResult<Value> {
    let pop = get_value!(value, GValue::Pop)?;
    Ok(json!({
        "@type": POP,
        "@value": *pop.to_string(),
    }))
}

pub(crate) fn cardinality(value: &GValue) -> GremlinResult<Value> {
    let cardinality = get_value!(value, GValue::Cardinality)?;
    let v = match cardinality {
        Cardinality::List => "list",
        Cardinality::Single => "single",
        Cardinality::Set => "set",
    };
    Ok(json!({
        "@type" : CARDINALITY,
        "@value" : v
    }))
}

pub(crate) fn merge(value: &GValue) -> GremlinResult<Value> {
    let merge = get_value!(value, GValue::Merge)?;
    let merge_option = match merge {
        Merge::OnCreate => "onCreate",
        Merge::OnMatch => "onMatch",
        Merge::OutV => "outV",
        Merge::InV => "inV",
    };
    Ok(json!({
        "@type" : MERGE,
        "@value" : merge_option
    }))
}

pub(crate) fn direction(value: &GValue) -> GremlinResult<Value> {
    let direction = get_value!(value, GValue::Direction)?;
    let direction_str = match direction {
        Direction::Out | Direction::From => "OUT",
        Direction::In | Direction::To => "IN",
    };
    Ok(json!({
        "@type" : DIRECTION,
        "@value" : direction_str,
    }))
}

pub(crate) fn column(value: &GValue) -> GremlinResult<Value> {
    let column = get_value!(value, GValue::Column)?;
    let column = match column {
        crate::structure::Column::Keys => "keys",
        crate::structure::Column::Values => "values",
    };
    Ok(json!({
        "@type" : COLUMN,
        "@value" : column,
    }))
}
