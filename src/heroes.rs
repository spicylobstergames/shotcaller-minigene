use crate::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HeroDefinition {
    pub key: Heroes,
    pub name: String,
    pub skills: Vec<Skills>,
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
pub struct HeroDefinitions {
    pub definitions: HashMap<Heroes, HeroDefinition>,
}

impl From<Vec<HeroDefinition>>
    for HeroDefinitions
{
    fn from(t: Vec<HeroDefinition>) -> Self {
        let defs = t
            .into_iter()
            .map(|s| (s.key.clone(), s))
            .collect::<HashMap<_, _>>();
        Self::new(defs)
    }
}
