#[derive(Debug, PartialEq, Clone)]
pub struct FeatureMetadata {
  id: i64,
  user: String,
  uid: u64,
  version: u32,
  changeset: u64,
  timestamp: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Node {
  lat: f64,
  lon: f64,
  metadata: FeatureMetadata,
  tags: Vec<Tag>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Way {
  nodes: Vec<ReferencedNode>,
  metadata: FeatureMetadata,
  tags: Vec<Tag>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Relation {
  members: Vec<RelationMember>,
  metadata: FeatureMetadata,
  tags: Vec<Tag>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReferencedNode {
  id: i64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Tag {
  key: String,
  value: String
}

#[derive(Debug, PartialEq, Clone)]
pub enum ReferencedFeature {
  Node(i64),
  Way(i64),
  Relation(i64)
}

#[derive(Debug, PartialEq, Clone)]
pub struct RelationMember {
  role: String,
  feature: ReferencedFeature,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Feature {
  Node(Node),
  Way(Way),
  Relation(Relation),
}

impl Feature {
  fn metadata(&self) -> &FeatureMetadata {
    match self {
      Feature::Node(me) => &me.metadata,
      Feature::Way(me) => &me.metadata,
      Feature::Relation(me) => &me.metadata,
    }
  }
}

