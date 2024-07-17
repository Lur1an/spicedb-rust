use super::consistency::Requirement;

pub enum Consistency {
    MinimizeLatency,
    AtLeastAsFresh(super::ZedToken),
    AtExactSnapshot(super::ZedToken),
    FullyConsistent,
}

impl From<Consistency> for super::Consistency {
    fn from(consistency: Consistency) -> Self {
        let requirement = match consistency {
            Consistency::MinimizeLatency => Requirement::MinimizeLatency(true),
            Consistency::AtLeastAsFresh(token) => Requirement::AtLeastAsFresh(token),
            Consistency::AtExactSnapshot(token) => Requirement::AtExactSnapshot(token),
            Consistency::FullyConsistent => Requirement::FullyConsistent(true),
        };
        super::Consistency {
            requirement: Some(requirement),
        }
    }
}
