pub enum Note {
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B,
}

impl Note {
    pub fn freq_base(&self) -> f64 {
        match self {
            Note::C => 16.35,
            Note::CSharp => 17.32,
            Note::D => 18.35,
            Note::DSharp => 19.45,
            Note::E => 20.60,
            Note::F => 21.83,
            Note::FSharp => 23.12,
            Note::G => 24.50,
            Note::GSharp => 25.96,
            Note::A => 27.50,
            Note::ASharp => 29.14,
            Note::B => 30.87,
        }
    }

    pub fn freq(&self, octave: i32) -> f64 {
        self.freq_base() * 2.0f64.powi(octave)
    }
}
