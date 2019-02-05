/// [Openstreetmap wiki](https://wiki.openstreetmap.org/wiki/Elements#Common_attributes)
#[derive(Debug, PartialEq, Clone)]
pub struct ElementMetadata {
  pub id: i64,

  /// The display name of the user who last modified the object; exclusively informative and may be empty.
  /// A user can change their display name at any time (existing elements will reflect the new user name without needing any version change).
  pub user: String,

  /// The user-id of the user who last modified the object.
  pub uid: u64,

  /// The version of the object; newly created objects start at version 1 and the value
  /// is incremented by the server when a client uploads a new version of the object.
  pub version: u32,

  /// The changeset number in which the object was created or last updated.
  pub changeset: u64,

  /// Time of the last modification,
  /// encoded in any [W3C standard date and time format](https://www.w3.org/TR/NOTE-datetime)
  pub timestamp: String,
}

/// [Openstreetmap wiki](https://wiki.openstreetmap.org/wiki/Node)
#[derive(Debug, PartialEq, Clone)]
pub struct Node {
  pub lat: f64,
  pub lon: f64,
  pub metadata: ElementMetadata,
  pub tags: Vec<Tag>,
}

/// A way is an ordered list of nodes which normally also has at least one tag or is included within a Relation.
/// It can be open or closed, a closed way is one whose last nodes on the way is also the first on that way.
/// A closed way may be interpreted as a closed polyline, a polygon, or both.
///
/// [Openstreetmap wiki](https://wiki.openstreetmap.org/wiki/Way)
///
/// Note that this library does not impose any upper limit for the size of a way, as the 2000-node-limit is considered an API limit
#[derive(Debug, PartialEq, Clone)]
pub struct Way {
  pub nodes: Vec<ReferencedNode>,
  pub metadata: ElementMetadata,
  pub tags: Vec<Tag>,
}

/// A collection of Elements
///
/// [Openstreetmap wiki](https://wiki.openstreetmap.org/wiki/Relation)
#[derive(Debug, PartialEq, Clone)]
pub struct Relation {
  pub members: Vec<RelationMember>,
  pub metadata: ElementMetadata,
  pub tags: Vec<Tag>,
}

/// A reference to a Node, by wrapping it's `id`.
#[derive(Debug, PartialEq, Clone)]
pub struct ReferencedNode {
  pub id: i64,
}

/// A key value pair.
///
/// [Openstreetmap wiki](https://wiki.openstreetmap.org/wiki/Tags)
#[derive(Debug, PartialEq, Clone)]
pub struct Tag {
  pub key: String,
  pub value: String
}

/// A reference to an element, by wrapping it's `id`.
#[derive(Debug, PartialEq, Clone)]
pub enum ReferencedElement {
  Node(i64),
  Way(i64),
  Relation(i64)
}

/// An entry from a Relation which references another element by id and role.
#[derive(Debug, PartialEq, Clone)]
pub struct RelationMember {
  pub role: String,
  pub element: ReferencedElement,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Element {
  Node(Node),
  Way(Way),
  Relation(Relation),
}

impl Element {
  pub fn metadata(&self) -> &ElementMetadata {
    match self {
      Element::Node(me) => &me.metadata,
      Element::Way(me) => &me.metadata,
      Element::Relation(me) => &me.metadata,
    }
  }
}

