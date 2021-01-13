use crate::*;

/// Specifies the archetype of a leader.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LeaderDefinition {
    /// The leader id.
    pub key: Leaders,
    /// The leader name.
    pub name: String,
    /// The leader skill set.
    pub skills: Vec<Skills>,
}

/// Contains the definitions of each leader.
#[derive(Serialize, Deserialize, Debug, Clone, new)]
pub struct LeaderDefinitions {
    /// The leader definitions.
    pub defs: HashMap<Leaders, LeaderDefinition>,
}

impl Default for LeaderDefinitions {
    fn default() -> Self {
        Self {
            defs: HashMap::default(),
        }
    }
}

impl From<Vec<LeaderDefinition>> for LeaderDefinitions {
    fn from(t: Vec<LeaderDefinition>) -> Self {
        let defs = t
            .into_iter()
            .map(|s| (s.key.clone(), s))
            .collect::<HashMap<_, _>>();
        Self::new(defs)
    }
}
