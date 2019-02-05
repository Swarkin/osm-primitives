#[derive(Debug, PartialEq, Clone)]
pub struct FeatureMetadata {
  pub id: u64,
  pub user: String,
  pub uid: u64,
  pub version: u32,
  pub changeset: u64,
  pub timestamp: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Node {
  pub lat: f64,
  pub lon: f64,
  pub metadata: FeatureMetadata,
  pub tags: Vec<Tag>,
}

/// A way is an ordered list of nodes which normally also has at least one tag or is included within a Relation.
/// It can be open or closed, a closed way is one whose last nodes on the way is also the first on that way.
/// A closed way may be interpreted as a closed polyline, a polygon, or both.
/// [Openstreetmap wiki](https://wiki.openstreetmap.org/wiki/Way)
/// Note that this library does not impose any upper limit for the size of a way, as the 2000-node-limit is considered an API limit
#[derive(Debug, PartialEq, Clone)]
pub struct Way {
  pub nodes: Vec<ReferencedNode>,
  pub metadata: FeatureMetadata,
  pub tags: Vec<Tag>,
}

/// [Openstreetmap wiki](https://wiki.openstreetmap.org/wiki/Relation)
#[derive(Debug, PartialEq, Clone)]
pub struct Relation {
  pub members: Vec<RelationMember>,
  pub metadata: FeatureMetadata,
  pub tags: Vec<Tag>,
}

/// A reference to a Node, by wrapping it's `id`.
#[derive(Debug, PartialEq, Clone)]
pub struct ReferencedNode {
  pub id: u64,
}

/// A key value pair.
/// [Openstreetmap wiki](https://wiki.openstreetmap.org/wiki/Tags)
#[derive(Debug, PartialEq, Clone)]
pub struct Tag {
  pub key: String,
  pub value: String
}

/// A reference to a feature, by wrapping it's `id`.
#[derive(Debug, PartialEq, Clone)]
pub enum ReferencedFeature {
  Node(u64),
  Way(u64),
  Relation(u64)
}

/// An entry from a Relation which links to another feature.
#[derive(Debug, PartialEq, Clone)]
pub struct RelationMember {
  pub role: String,
  pub feature: ReferencedFeature,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Feature {
  Node(Node),
  Way(Way),
  Relation(Relation),
}

impl Feature {
  pub fn metadata(&self) -> &FeatureMetadata {
    match self {
      Feature::Node(me) => &me.metadata,
      Feature::Way(me) => &me.metadata,
      Feature::Relation(me) => &me.metadata,
    }
  }
}

