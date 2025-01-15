use std::fmt;


#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ControlLine {
    /// Clock
    CLK,
    /// Halt the computer.
    HLT,
    /// Load address into MAR.
    MI,
    /// Load data into RAM.
    RI,
    /// Output data from RAM.
    RO,
    /// Load instruction into IR.
    II,
    /// Output instruction from IR.
    IO,
    /// Load data into Register A.
    AI,
    /// Output data from Register A.
    AO,
    /// Output ALU result.
    EO,
    /// Set ALU to subtract.
    SU,
    /// Load data into Register B.
    BI,
    /// Output data from Register B.
    BO,
    /// Load data into Output Register.
    OI,
    /// Increment Program Counter.
    CE,
    /// Output Program Counter.
    CO,
    /// Load bus address into PC.
    J,
    /// Load data into Flags Register.
    FI,
}

impl fmt::Display for ControlLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ControlLine::CLK => "CLK",
            ControlLine::HLT => "HLT",
            ControlLine::MI => "MI",
            ControlLine::RI => "RI",
            ControlLine::RO => "RO",
            ControlLine::II => "II",
            ControlLine::IO => "IO",
            ControlLine::AI => "AI",
            ControlLine::AO => "AO",
            ControlLine::EO => "EO",
            ControlLine::SU => "SU",
            ControlLine::BI => "BI",
            ControlLine::BO => "BO",
            ControlLine::OI => "OI",
            ControlLine::CE => "CE",
            ControlLine::CO => "CO",
            ControlLine::J => "J",
            ControlLine::FI => "FI",
        };
        write!(f, "{}", s)
    }
}
