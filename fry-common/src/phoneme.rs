/// A single unit of sound.
///
/// There will be examples for each sound in general American English.
#[repr(u8)]
pub enum Phoneme {
    /// Unknown value
    Epsilon,
    /// odd: `AA D`
    Aa, 
    /// at: `AE T`
    Ae, 
    /// hut: `HH AH T`
    Ah, 
    /// ought: `AO T`
    Ao, 
    /// cow: `K AW`
    Aw, 
    /// hide: `HH AY D`
    Ay, 
    /// be: `B IY`
    B,  
    /// cheese: `CH IY Z`
    Ch, 
    /// deep: `D IY P`
    D,  
    /// thee: `DH IY`
    Dh, 
    /// ed: `EH D`
    Eh, 
    /// hurt: `HH ER T`
    Er, 
    /// ate: `EY T`
    Ey, 
    /// fee: `F IY`
    F,  
    /// green: `G R IY N`
    G,  
    /// he: `HH IY`
    Hh, 
    /// it: `IH T`
    Ih, 
    /// eat: `IY T`
    Iy, 
    /// gee: `JH IY`
    Jh, 
    /// key: `K IY`
    K,  
    /// lee: `L IY`
    L,  
    /// me: `M IY`
    M,  
    /// knee: `N IY`
    N,  
    /// ping: `P IH NG`
    Ng, 
    /// oat: `OW T`
    Ow, 
    /// toy: `T OY`
    Oy, 
    /// pee: `P IY`
    P,  
    /// read: `R IY D`
    R,  
    /// sea: `S IY`
    S,  
    /// she: `SH IY`
    Sh, 
    /// tea: `T IY`
    T,  
    /// theta: `TH EY T AH`
    Th, 
    /// hood: `HH UH D`
    Uh, 
    /// two: `T UW`
    Uw, 
    /// vee: `V IY`
    V,  
    /// we: `W IY`
    W,  
    /// yield: `Y IY L D`
    Y,  
    /// zee: `Z IY`
    Z,  
    /// seizure: `S IY ZH ER`
    Zh, 
}

/// Stress pattern to use (if the language supports it).
#[repr(u8)]
pub enum Stress {
    /// No stress.
    No = 0,
    /// Primary stress.
    Primary = 1,
    /// Secondary stress.
    Secondary = 2,
}

