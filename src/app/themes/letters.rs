pub struct Letters {
    pub font: Font,
    pub letter_sizes: LetterSizes,
    pub letter_weigts: LetterWeigts
}

pub enum Font {
    Custom(String),
    System
}
pub struct LetterSizes {
    text: u16,
    window_title: u16,
    section_title: u16,
    highlight: u16,
    secondary_text: u16,
    help_text: u16,
    control_label: u16,
    monospace_text: u16,
}
pub struct LetterWeigts {
        text: Weight,
        window_title: Weight,
        section_title: Weight,
        highlight: Weight,
        secondary_text: Weight,
        help_text: Weight,
        control_label: Weight,
        monospace_text: Weight,
}

pub enum Weight {
    Normal,
    Medium,
    Bold,
    Semibold
}