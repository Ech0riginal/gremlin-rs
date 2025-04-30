use crate::io::GraphSONSerializer;
use crate::prelude::{
    traversal::{Order, Scope},
    Cardinality, Direction, GValue, GremlinError, GremlinResult, Merge, ToGValue, T,
};
use serde_json::{json, Map, Value};
use std::collections::HashMap;

pub(crate) fn double(value: &GValue) -> GremlinResult<Value> {
    let double = get_value!(value, GValue::Double)?;
    Ok(json!({
        "@type" : "g:Double",
        "@value" : double,
    }))
}

pub(crate) fn float(value: &GValue) -> GremlinResult<Value> {
    let float = get_value!(value, GValue::Float)?;
    Ok(json!({
        "@type" : "g:Float",
        "@value" : float,
    }))
}

pub(crate) fn int32(value: &GValue) -> GremlinResult<Value> {
    let int32 = get_value!(value, GValue::Int32)?;
    Ok(json!({
        "@type" : "g:Int32",
        "@value" : int32,
    }))
}

pub(crate) fn int64(value: &GValue) -> GremlinResult<Value> {
    let int64 = get_value!(value, GValue::Int64)?;
    Ok(json!({
        "@type" : "g:Int64",
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
        "@type" : "g:UUID",
        "@value" : uuid.to_string()
    }))
}

pub(crate) fn date(value: &GValue) -> GremlinResult<Value> {
    let date = get_value!(value, GValue::Date)?;
    Ok(json!({
        "@type" : "g:Date",
        "@value" : date.timestamp_millis()
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
        "@type" : "g:P",
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
        "@type" : "g:Bytecode",
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
            "@type" : "g:Vertex",
            "@value" : {
                "id" :  id,
                "label": v.label(),
            }
        })
    } else {
        json!({
            "@type" : "g:Vertex",
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
            "@type" : "g:VertexProperty",
            "@value" : {
                "id" : S::serialize(&property.id.to_gvalue())?,
                "value" : S::serialize(&property.value)?,
                "label" : S::serialize(&property.label.to_gvalue())?,
            },
            "vertex" : S::serialize(&vertex_id.to_gvalue())?,
        })
    } else {
        json!({
            "@type" : "g:VertexProperty",
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
        .map(|(label, property)| (label, S::serialize(&GValue::Property(property.clone()))))
        .filter(|(_, r)| r.is_ok())
        .map(|(p, r)| (p.clone(), r.unwrap()))
        .collect::<HashMap<String, Value>>();

    let value = if properties.is_empty() {
        json!({
            "@type" : "g:Edge",
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
            "@type" : "g:Edge",
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
        "@type" : "g:Path",
        "@value": {
            "labels" : S::serialize(&path.labels)?,
            "objects" : S::serialize(&path.objects)?,
        }
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
        "@type" : "g:T",
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
        "@type" : "g:Scope",
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
        "@type" : "g:Order",
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
        "@type" : "g:TextP",
        "@value" : {
            "predicate" : text_p.operator(),
            "value" : S::serialize(text_p.value())?
        }
    }))
}

pub(crate) fn pop(value: &GValue) -> GremlinResult<Value> {
    let pop = get_value!(value, GValue::Pop)?;
    Ok(json!({
        "@type": "g:Pop",
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
        "@type" : "g:Cardinality",
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
        "@type" : "g:Merge",
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
        "@type" : "g:Direction",
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
        "@type" : "g:Column",
        "@value" : column,
    }))
}
