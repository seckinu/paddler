use crate::ipa::{Feature, FeatureState};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModifierPosition {
    Pre,
    Post,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Modifier {
    Ejective,
    Voiceless,
    Aspirated,
    Advanced,
    Retracted,
    Centralized,
    Syllabic,
    NonSyllabic,
    Rhotacized,
    BreathyVoiced,
    CreakyVoiced,
    Glottalized,
    Preglottalized,
    Linguolabial,
    Labialized,
    Palatalized,
    Labiopalatalized,
    Velarized,
    Pharyngealized,
    VelarizedOrPharyngealized,
    Raised,
    Lowered,
    ATR,
    RTR,
    Apical,
    Laminal,
    Nasalized,
    NasalRelease,
    LateralRelease,
    Long,
    ExtraShort,
}

impl Modifier {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "ʼ" => Some(Modifier::Ejective),
            "̥" => Some(Modifier::Voiceless),
            "ʰ" => Some(Modifier::Aspirated),
            "̟" => Some(Modifier::Advanced),
            "̠" => Some(Modifier::Retracted),
            "̈" => Some(Modifier::Centralized),
            "̩" => Some(Modifier::Syllabic),
            "̯" => Some(Modifier::NonSyllabic),
            "˞" => Some(Modifier::Rhotacized),
            "̤" => Some(Modifier::BreathyVoiced),
            "̰" => Some(Modifier::CreakyVoiced),
            "ˀ" => Some(Modifier::Glottalized),
            // "ˀ" => Ok(Modifier::Preglottalized),
            "̼" => Some(Modifier::Linguolabial),
            "ʷ" => Some(Modifier::Labialized),
            "ʲ" => Some(Modifier::Palatalized),
            "ᶣ" => Some(Modifier::Labiopalatalized),
            "ˠ" => Some(Modifier::Velarized),
            "ˤ" => Some(Modifier::Pharyngealized),
            "̴" => Some(Modifier::VelarizedOrPharyngealized),
            "̝" => Some(Modifier::Raised),
            "̞" => Some(Modifier::Lowered),
            "̘" => Some(Modifier::ATR),
            "̙" => Some(Modifier::RTR),
            "̺" => Some(Modifier::Apical),
            "̻" => Some(Modifier::Laminal),
            "̃" => Some(Modifier::Nasalized),
            "ⁿ" => Some(Modifier::NasalRelease),
            "ˡ" => Some(Modifier::LateralRelease),

            // allow colon for easier typing
            "ː" => Some(Modifier::Long),
            ":" => Some(Modifier::Long),

            "̆" => Some(Modifier::ExtraShort),
            _ => None,
        }
    }
}

impl Modifier {
    pub fn marker(&self) -> &'static str {
        match self {
            Modifier::Ejective => "ʼ",
            Modifier::Voiceless => "̥",
            Modifier::Aspirated => "ʰ",
            Modifier::Advanced => "̟",
            Modifier::Retracted => "̠",
            Modifier::Centralized => "̈",
            Modifier::Syllabic => "̩",
            Modifier::NonSyllabic => "̯",
            Modifier::Rhotacized => "˞",
            Modifier::BreathyVoiced => "̤",
            Modifier::CreakyVoiced => "̰",
            Modifier::Glottalized => "ˀ",
            Modifier::Preglottalized => "ˀ",
            Modifier::Linguolabial => "̼",
            Modifier::Labialized => "ʷ",
            Modifier::Palatalized => "ʲ",
            Modifier::Labiopalatalized => "ᶣ",
            Modifier::Velarized => "ˠ",
            Modifier::Pharyngealized => "ˤ",
            Modifier::VelarizedOrPharyngealized => "̴",
            Modifier::Raised => "̝",
            Modifier::Lowered => "̞",
            Modifier::ATR => "̘",
            Modifier::RTR => "̙",
            Modifier::Apical => "̺",
            Modifier::Laminal => "̻",
            Modifier::Nasalized => "̃",
            Modifier::NasalRelease => "ⁿ",
            Modifier::LateralRelease => "ˡ",
            Modifier::Long => "ː",
            Modifier::ExtraShort => "̆",
        }
    }

    pub const fn position(&self) -> ModifierPosition {
        match self {
            Self::Preglottalized => ModifierPosition::Pre,
            _ => ModifierPosition::Post,
        }
    }

    pub fn is_pre(&self) -> bool {
        self.position() == ModifierPosition::Pre
    }

    pub fn is_post(&self) -> bool {
        self.position() == ModifierPosition::Post
    }

    pub const fn feature(&self) -> &'static [(Feature, FeatureState)] {
        match self {
            Modifier::Ejective => &[(Feature::ConstrictedGlottis, FeatureState::Positive)],
            Modifier::Voiceless => &[(Feature::Voice, FeatureState::Negative)],
            Modifier::Aspirated => &[(Feature::SpreadGlottis, FeatureState::Positive)],
            Modifier::Advanced => &[],
            Modifier::Retracted => &[],
            Modifier::Centralized => &[],
            Modifier::Syllabic => &[(Feature::Syllabic, FeatureState::Positive)],
            Modifier::NonSyllabic => &[(Feature::Syllabic, FeatureState::Negative)],
            Modifier::Rhotacized => &[
                (Feature::Anterior, FeatureState::Negative),
                (Feature::High, FeatureState::Positive),
                (Feature::Round, FeatureState::Positive),
            ],
            Modifier::BreathyVoiced => &[(Feature::SpreadGlottis, FeatureState::Positive)],
            Modifier::CreakyVoiced => &[(Feature::ConstrictedGlottis, FeatureState::Positive)],
            Modifier::Glottalized => &[(Feature::ConstrictedGlottis, FeatureState::Positive)],
            Modifier::Preglottalized => &[(Feature::ConstrictedGlottis, FeatureState::Positive)],
            Modifier::Linguolabial => &[(Feature::Labial, FeatureState::Positive)],
            Modifier::Labialized => &[
                (Feature::Round, FeatureState::Positive),
                (Feature::Back, FeatureState::Positive),
                (Feature::High, FeatureState::Positive),
            ],
            Modifier::Palatalized => &[
                (Feature::High, FeatureState::Positive),
                (Feature::Back, FeatureState::Negative),
            ],
            Modifier::Labiopalatalized => &[
                (Feature::High, FeatureState::Positive),
                (Feature::Back, FeatureState::Negative),
                (Feature::Round, FeatureState::Positive),
            ],
            Modifier::Velarized => &[
                (Feature::High, FeatureState::Positive),
                (Feature::Back, FeatureState::Positive),
            ],
            Modifier::Pharyngealized => &[
                (Feature::Low, FeatureState::Positive),
                (Feature::Back, FeatureState::Positive),
            ],
            Modifier::VelarizedOrPharyngealized => &[
                (Feature::High, FeatureState::Positive),
                (Feature::Back, FeatureState::Positive),
            ],
            Modifier::Raised => &[],
            Modifier::Lowered => &[],
            Modifier::ATR => &[(Feature::Tense, FeatureState::Positive)],
            Modifier::RTR => &[(Feature::Tense, FeatureState::Negative)],
            Modifier::Apical => &[(Feature::Distributed, FeatureState::Negative)],
            Modifier::Laminal => &[(Feature::Distributed, FeatureState::Positive)],
            Modifier::Nasalized => &[(Feature::Nasal, FeatureState::Positive)],
            Modifier::NasalRelease => &[(Feature::Nasal, FeatureState::Positive)],
            Modifier::LateralRelease => &[
                (Feature::Lateral, FeatureState::Positive),
                (Feature::DelayedRelease, FeatureState::Positive),
            ],
            Modifier::Long => &[(Feature::Long, FeatureState::Positive)],
            Modifier::ExtraShort => &[(Feature::Long, FeatureState::Negative)],
        }
    }

    pub const fn conditions(&self) -> &'static [&[(Feature, FeatureState)]] {
        match self {
            Modifier::Ejective => &[&[
                (Feature::Sonorant, FeatureState::Negative),
                (Feature::Voice, FeatureState::Negative),
            ]],
            Modifier::Voiceless => &[&[
                (Feature::Sonorant, FeatureState::Positive),
                (Feature::Voice, FeatureState::Positive),
            ]],
            Modifier::Aspirated => &[
                &[
                    (Feature::Sonorant, FeatureState::Negative),
                    (Feature::ConstrictedGlottis, FeatureState::Negative),
                    (Feature::Continuant, FeatureState::Negative),
                ],
                &[
                    (Feature::Continuant, FeatureState::Positive),
                    (Feature::Sonorant, FeatureState::Negative),
                    (Feature::Voice, FeatureState::Negative),
                ],
            ],
            Modifier::Advanced => &[
                &[(Feature::Syllabic, FeatureState::Negative)],
                &[(Feature::Syllabic, FeatureState::Positive)],
            ],
            Modifier::Retracted => &[
                &[(Feature::Syllabic, FeatureState::Negative)],
                &[(Feature::Syllabic, FeatureState::Positive)],
            ],
            Modifier::Centralized => &[&[(Feature::Syllabic, FeatureState::Positive)]],
            Modifier::Syllabic => &[
                &[
                    (Feature::Syllabic, FeatureState::Negative),
                    (Feature::Continuant, FeatureState::Positive),
                    (Feature::DelayedRelease, FeatureState::Negative),
                ],
                &[
                    (Feature::Syllabic, FeatureState::Negative),
                    (Feature::Sonorant, FeatureState::Positive),
                ],
            ],
            Modifier::NonSyllabic => &[&[(Feature::Syllabic, FeatureState::Positive)]],
            Modifier::Rhotacized => &[&[(Feature::Syllabic, FeatureState::Positive)]],
            Modifier::BreathyVoiced => &[&[(Feature::Voice, FeatureState::Positive)]],
            Modifier::CreakyVoiced => &[&[(Feature::Voice, FeatureState::Positive)]],
            Modifier::Glottalized => &[&[(Feature::ConstrictedGlottis, FeatureState::Negative)]],
            Modifier::Preglottalized => &[&[
                (Feature::ConstrictedGlottis, FeatureState::Negative),
                (Feature::Syllabic, FeatureState::Negative),
            ]],
            Modifier::Linguolabial => &[
                &[
                    (Feature::Coronal, FeatureState::Positive),
                    (Feature::Anterior, FeatureState::Positive),
                    (Feature::Continuant, FeatureState::Negative),
                    (Feature::DelayedRelease, FeatureState::Negative),
                ],
                &[
                    (Feature::Continuant, FeatureState::Positive),
                    (Feature::Sonorant, FeatureState::Negative),
                    (Feature::Coronal, FeatureState::Positive),
                    (Feature::Anterior, FeatureState::Positive),
                    (Feature::Strident, FeatureState::Negative),
                    (Feature::DelayedRelease, FeatureState::Negative),
                ],
            ],
            Modifier::Labialized => &[&[
                (Feature::Round, FeatureState::Positive),
                (Feature::Back, FeatureState::Positive),
                (Feature::High, FeatureState::Positive),
            ]],
            Modifier::Palatalized => &[&[(Feature::Syllabic, FeatureState::Negative)]],
            Modifier::Labiopalatalized => &[&[
                (Feature::Labial, FeatureState::Negative),
                (Feature::Syllabic, FeatureState::Negative),
            ]],
            Modifier::Velarized => &[
                &[(Feature::Syllabic, FeatureState::Negative)],
                &[(Feature::High, FeatureState::Negative)],
                &[(Feature::Back, FeatureState::Negative)],
            ],
            Modifier::Pharyngealized => &[],
            Modifier::VelarizedOrPharyngealized => &[&[
                (Feature::Coronal, FeatureState::Positive),
                (Feature::Lateral, FeatureState::Positive),
                (Feature::DelayedRelease, FeatureState::Negative),
            ]],
            Modifier::Raised => &[&[(Feature::Continuant, FeatureState::Positive)]],
            Modifier::Lowered => &[&[(Feature::Continuant, FeatureState::Positive)]],
            Modifier::ATR => &[&[(Feature::Syllabic, FeatureState::Positive)]],
            Modifier::RTR => &[&[(Feature::Syllabic, FeatureState::Positive)]],
            Modifier::Apical => &[&[(Feature::Coronal, FeatureState::Positive)]],
            Modifier::Laminal => &[&[(Feature::Coronal, FeatureState::Positive)]],
            Modifier::Nasalized => &[&[
                (Feature::Voice, FeatureState::Positive),
                (Feature::Nasal, FeatureState::Negative),
            ]],
            Modifier::NasalRelease => &[&[
                (Feature::Sonorant, FeatureState::Negative),
                (Feature::Continuant, FeatureState::Negative),
                (Feature::DelayedRelease, FeatureState::Negative),
            ]],
            Modifier::LateralRelease => &[&[
                (Feature::Sonorant, FeatureState::Negative),
                (Feature::Continuant, FeatureState::Negative),
                (Feature::Coronal, FeatureState::Positive),
                (Feature::DelayedRelease, FeatureState::Negative),
            ]],
            Modifier::Long => &[&[(Feature::Long, FeatureState::Negative)]],
            Modifier::ExtraShort => &[&[(Feature::Syllabic, FeatureState::Positive)]],
        }
    }
}
