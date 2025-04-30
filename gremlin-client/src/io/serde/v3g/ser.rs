use crate::prelude::{GValue, GremlinResult};
use serde_json::{json, Value};

pub fn geometry(value: &GValue) -> GremlinResult<Value> {
    let g = get_value!(value, GValue::Geometry)?;
    Ok(json!({
        "@type" : "skg",
        "@value" : geojson::Geometry::from(g)
    }))
}
