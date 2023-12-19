use std::collections::{BTreeMap, BTreeSet};

#[derive(Default, Debug, PartialEq)]
pub struct Artifact {}

impl Artifact {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, PartialEq)]
pub struct Abstract {
    artifact: Artifact,
    claims: BTreeMap<Persona, BTreeSet<Claim>>,
}

impl Abstract {
    pub fn new(artifact: Artifact) -> Self {
        Self {
            artifact,
            claims: BTreeMap::new(),
        }
    }
    pub fn add_claim(&mut self, persona: Persona, claim: Claim) {
        // e.g., marriage creates bi-directional claims
    }

    pub fn merge(&mut self, other: Abstract) -> Self {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
pub enum Honorific {
    Mister,
    Misus,
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
pub struct Name {
    honorific: Option<Honorific>,
    first: Option<String>,
    middle: Option<String>,
    last: Option<String>,
}

impl Name {
    pub fn first_last(first: &str, last: &str) -> Self {
        Self {
            honorific: None,
            first: Some(first.into()),
            middle: None,
            last: Some(last.into()),
        }
    }

    pub fn misus_first_last(first: &str, last: &str) -> Self {
        Self {
            honorific: Some(Honorific::Misus),
            ..Name::first_last(first, last)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Persona {
    pub name: Option<Name>,
    pub gender: Option<Gender>,
}

impl Persona {
    pub fn new(name: Name) -> Self {
        Self {
            name: Some(name),
            gender: None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Marriage {
    spouse: Persona,
}

impl Marriage {
    pub fn new(spouse: Persona) -> Self {
        Self { spouse }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Time {}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Place {}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Residence {
    place: Place,
    time: Option<Time>,
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct Death {
    place: Option<Place>,
    time: Option<Time>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Burial {
    place: Option<Place>,
    time: Option<Time>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Birth {
    time: Option<Time>,
    place: Option<Place>,
    mother: Option<Persona>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Claim {
    Birth(Birth),
    Marriage(Marriage),
    Death(Death),
    Residence(Residence),
    Burial(Burial),
}

impl Claim {
    pub fn death() -> Self {
        Self::Death(Death::default())
    }

    pub fn marriage(spouse: Persona) -> Self {
        Self::Marriage(Marriage::new(spouse))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn foo() {
        let mary_holloway = Persona::new(Name::misus_first_last("Mary", "Holloway"));
        let ransom_holloway = Persona::new(Name::first_last("Ransom", "Holloway"));
        let artifact = Artifact::new();
        let mut r#abstract = Abstract::new(artifact);

        r#abstract.add_claim(
            mary_holloway.clone(),
            Claim::marriage(ransom_holloway.clone()),
        );
        r#abstract.add_claim(mary_holloway, Claim::death());
        r#abstract.add_claim(ransom_holloway, Claim::death());
    }
}
