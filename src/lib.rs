type Id = i64;
type Tags = std::collections::HashMap<String, String>;

/// [Openstreetmap wiki](https://wiki.openstreetmap.org/wiki/Elements#Common_attributes)
#[derive(Debug, PartialEq, Clone)]
pub struct Metadata {
	pub id: Id,
	pub user: String,
	pub uid: u64,
	pub timestamp: String,
	pub visible: bool,
	pub version: u32,
	pub changeset: u64,
}

/// [Openstreetmap wiki](https://wiki.openstreetmap.org/wiki/Node)
#[derive(Debug, PartialEq, Clone)]
pub struct Node {
	pub lat: f64,
	pub lon: f64,
	pub metadata: Metadata,
	pub tags: Tags,
}

/// [Openstreetmap wiki](https://wiki.openstreetmap.org/wiki/Way)
#[derive(Debug, PartialEq, Clone)]
pub struct Way {
	pub nodes: Vec<Id>,
	pub metadata: Metadata,
	pub tags: Tags,
}

/// [Openstreetmap wiki](https://wiki.openstreetmap.org/wiki/Relation)
#[derive(Debug, PartialEq, Clone)]
pub struct Relation {
	pub members: Vec<RelationMember>,
	pub metadata: Metadata,
	pub tags: Tags,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Reference {
	Node(Id),
	Way(Id),
	Relation(Id)
}

#[derive(Debug, PartialEq, Clone)]
pub struct RelationMember {
	pub role: String,
	pub element: Reference,
}
