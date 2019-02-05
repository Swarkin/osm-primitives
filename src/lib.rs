#[derive(Debug, PartialEq, Clone)]
pub struct FeatureMetadata {
  pub id: i64,
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

#[derive(Debug, PartialEq, Clone)]
pub struct Way {
  pub nodes: Vec<ReferencedNode>,
  pub metadata: FeatureMetadata,
  pub tags: Vec<Tag>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Relation {
  pub members: Vec<RelationMember>,
  pub metadata: FeatureMetadata,
  pub tags: Vec<Tag>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReferencedNode {
  pub id: i64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Tag {
  pub key: String,
  pub value: String
}

#[derive(Debug, PartialEq, Clone)]
pub enum ReferencedFeature {
  Node(i64),
  Way(i64),
  Relation(i64)
}

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
  fn metadata(&self) -> &FeatureMetadata {
    match self {
      Feature::Node(me) => &me.metadata,
      Feature::Way(me) => &me.metadata,
      Feature::Relation(me) => &me.metadata,
    }
  }
}

