//! Structs and algorithms related to diphones (a collection of two adjacent phones).

/// A diphone entry.
pub struct DiphoneEntry<'a> {
    /// The name of the entry (must be two phones separated by a dash)
    /// TODO: stricten the string
    pub name: &'a str,
    /// Starting pitch mark
    pub start_pm: u16,
    /// Pitch boundry pitch mark
    pub pb_pm: u8,
    /// Ending pitch mark
    pub end_pm: u8,
}
