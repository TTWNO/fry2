//! Path components for representing relations between tree items.

/// A single path component (where to go to next) in a tree.
///
/// <http://www.festvox.org/docs/manual-2.4.0/festival_14.html#Features>
#[derive(PartialEq, Eq, Debug)]
#[allow(missing_docs)]
pub enum Path<'a> {
    Next,
    Previous,
    First,
    Last,
    Parent,
    NextNext,
    PreviousPrevious,
    Daughter,
    SecondDaughter,
    LastDaughter,
    Relation(&'a str),
}

impl<'a> TryFrom<&'a str> for Path<'a> {
    type Error = PathError<'a>;
    fn try_from(s: &'a str) -> Result<Path<'a>, Self::Error> {
        use Path::*;
        match s {
            "n" => return Ok(Next),
            "p" => return Ok(Previous),
            "first" => return Ok(First),
            "last" => return Ok(Last),
            "parent" => return Ok(Parent),
            "nn" => return Ok(NextNext),
            "pp" => return Ok(PreviousPrevious),
            "daughter" | "daughter1" => return Ok(Daughter),
            "daughter2" => return Ok(SecondDaughter),
            "daughtern" => return Ok(LastDaughter),
            "" => return Err(PathError::MissingFeature),
            _ => {}
        };
        let mut split = s.split(":");
        let Some(r_colon) = split.next() else {
            return Err(PathError::MissingFeature);
        };
        if r_colon != "R" {
            return Err(PathError::InvalidFeature(r_colon));
        }
        let Some(rel) = split.next() else {
            return Err(PathError::MissingRelation);
        };
        Ok(Relation(rel))
    }
}

/// An error serializing the path.
pub enum PathError<'a> {
    /// Missing a feature; blank string
    MissingFeature,
    /// Invalid feature (not one of the accepted features)
    InvalidFeature(&'a str),
    /// Missing relation; blank string after "R:"
    MissingRelation,
}
