use super::{Direction, T};
use crate::error::GremlinError;
use crate::prelude::{GremlinResult, Token};
use crate::structure::{Edge, GValue, Vertex};
use std::collections::hash_map::IntoIter;
use std::collections::{BTreeMap, HashMap};
use std::convert::{TryFrom, TryInto};
use std::fmt::Formatter;

/// Represent a Map<[GKey](struct.GKey),[GValue](struct.GValue)> which has ability to allow for non-String keys.
/// TinkerPop type [here](http://tinkerpop.apache.org/docs/current/dev/io/#_map)
#[derive(PartialEq, Clone)]
pub struct Map(HashMap<GKey, GValue>);

impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ ")?;

        if !self.0.is_empty() {
            let mut iter = self.0.iter();
            if let Some((k, v)) = iter.next() {
                write!(f, "({:?}, {:?}), ", k, v)?;
            }
            while let Some((k, v)) = iter.next() {
                write!(f, ", ({:?}, {:?})", k, v)?;
            }
        }

        write!(f, "}}")
    }
}

impl Map {
    pub(crate) fn empty() -> Map {
        Map(HashMap::default())
    }
}

impl From<HashMap<GKey, GValue>> for Map {
    fn from(val: HashMap<GKey, GValue>) -> Self {
        Map(val)
    }
}

impl From<Map> for HashMap<GKey, GValue> {
    fn from(map: Map) -> Self {
        map.0
    }
}

impl From<HashMap<String, GValue>> for Map {
    fn from(val: HashMap<String, GValue>) -> Self {
        let map = val.into_iter().map(|(k, v)| (GKey::String(k), v)).collect();
        Map(map)
    }
}

impl TryFrom<Map> for HashMap<String, GValue> {
    type Error = GremlinError;

    fn try_from(map: Map) -> Result<Self, Self::Error> {
        map.into_iter()
            .map(|(k, v)| Ok((k.try_into()?, v)))
            .collect()
    }
}

impl From<BTreeMap<String, GValue>> for Map {
    fn from(val: BTreeMap<String, GValue>) -> Self {
        let map = val.into_iter().map(|(k, v)| (GKey::String(k), v)).collect();
        Map(map)
    }
}

impl TryFrom<Map> for BTreeMap<String, GValue> {
    type Error = GremlinError;

    fn try_from(map: Map) -> Result<Self, Self::Error> {
        map.into_iter()
            .map(|(k, v)| Ok((k.try_into()?, v)))
            .collect()
    }
}

impl Map {
    pub(crate) fn remove<T>(&mut self, key: T) -> Option<GValue>
    where
        T: Into<GKey>,
    {
        self.0.remove(&key.into())
    }
    /// Iterate all key-value pairs
    pub fn iter(&self) -> impl Iterator<Item = (&GKey, &GValue)> {
        self.0.iter()
    }

    ///Returns a reference to the value corresponding to the key.
    pub fn get<T>(&self, key: T) -> Option<&GValue>
    where
        T: Into<GKey>,
    {
        self.0.get(&key.into())
    }

    ///Returns try_get and conversion
    pub fn try_get<K, V>(&self, key: K) -> GremlinResult<V>
    where
        K: Into<GKey>,
        V: std::convert::TryFrom<GValue, Error = GremlinError>,
    {
        self.0
            .get(&key.into())
            .cloned()
            .or_else(|| Some(GValue::Null))
            .map(V::try_from)
            .ok_or_else(|| GremlinError::Cast(String::from("field not found")))?
    }

    /// Returns the number of elements in the map.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the map contains no elements.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T: Into<GKey>> std::ops::Index<T> for Map {
    type Output = GValue;

    fn index(&self, key: T) -> &GValue {
        self.0.get(&key.into()).expect("no entry found for key")
    }
}

impl std::ops::Deref for Map {
    type Target = HashMap<GKey, GValue>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoIterator for Map {
    type Item = (GKey, GValue);
    type IntoIter = IntoIter<GKey, GValue>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl std::iter::FromIterator<(String, GValue)> for Map {
    fn from_iter<I: IntoIterator<Item = (String, GValue)>>(iter: I) -> Self {
        Map(iter
            .into_iter()
            .map(|(k, v)| (GKey::String(k), v))
            .collect())
    }
}
/// Possible key types in a [Map](struct.Map)
#[allow(clippy::large_enum_variant)]
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum GKey {
    T(T),
    String(String),
    Token(Token),
    Vertex(Vertex),
    Edge(Edge),
    Direction(Direction),
}

impl From<T> for GKey {
    fn from(val: T) -> Self {
        GKey::T(val)
    }
}

impl From<Direction> for GKey {
    fn from(value: Direction) -> Self {
        GKey::Direction(value)
    }
}

impl From<&str> for GKey {
    fn from(val: &str) -> Self {
        GKey::String(String::from(val))
    }
}

impl From<String> for GKey {
    fn from(val: String) -> Self {
        GKey::String(val)
    }
}

impl TryFrom<GKey> for String {
    type Error = GremlinError;

    fn try_from(k: GKey) -> Result<Self, Self::Error> {
        if let GKey::String(s) = k {
            Ok(s)
        } else {
            Err(GremlinError::Cast(String::from(format!(
                "Cannot cast from {:?} to String",
                k
            ))))
        }
    }
}

impl From<&Vertex> for GKey {
    fn from(val: &Vertex) -> Self {
        GKey::Vertex(val.clone())
    }
}

impl From<&Edge> for GKey {
    fn from(val: &Edge) -> Self {
        GKey::Edge(val.clone())
    }
}
