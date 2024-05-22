use casey::lower;
use std::{fmt::Display, str::FromStr};

macro_rules! word_repetition {
    ($($word:ident)*) => {
        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        pub enum Word {
            $( $word, )*
        }

        impl FromStr for Word {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $( lower!(stringify!($word)) => Ok(Self::$word), )*
                    _ => Err(())
                }
            }
        }
    };
}

impl Word {
    pub fn is_predicate_marker(&self) -> bool {
        [Self::Li, Self::O].contains(self)
    }

    pub fn is_preposition(&self) -> bool {
        [Self::Tawa, Self::Kepeken, Self::Lon].contains(self)
    }

    pub fn is_preverb(&self) -> bool {
        [Self::Kama, Self::Wile].contains(self)
    }

    pub fn is_particle(&self) -> bool {
        [Self::Li, Self::O, Self::La, Self::E, Self::A].contains(self)
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{self:?}").to_lowercase())
    }
}

word_repetition!(
    A
    Akesi
    Ala
    Alasa
    Ale
    Anpa
    Ante
    Anu
    Awen
    E
    En
    Esun
    Ijo
    Ike
    Ilo
    Insa
    Jaki
    Jan
    Jelo
    Jo
    Kala
    Kalama
    Kama
    Kasi
    Ken
    Kepeken
    Kili
    Kiwen
    Ko
    Kon
    Kule
    Kulupu
    Kute
    La
    Lape
    Laso
    Lawa
    Len
    Lete
    Li
    Lili
    Linja
    Lipu
    Loje
    Lon
    Luka
    Lukin
    Lupa
    Ma
    Mama
    Mani
    Mi
    Moku
    Moli
    Monsi
    Mu
    Mun
    Musi
    Mute
    Nanpa
    Nasa
    Nasin
    Nena
    Ni
    Nimi
    Noka
    O
    Olin
    Ona
    Open
    Pakala
    Pali
    Palisa
    Pan
    Pana
    Pi
    Pilin
    Pimeja
    Pini
    Pipi
    Poka
    Poki
    Pona
    Pu
    Sama
    Seli
    Selo
    Seme
    Sewi
    Sijelo
    Sike
    Sin
    Sina
    Sinpin
    Sitelen
    Sona
    Soweli
    Suli
    Suno
    Supa
    Suwi
    Tan
    Taso
    Tawa
    Telo
    Tenpo
    Toki
    Tomo
    Tu
    Unpa
    Uta
    Utala
    Walo
    Wan
    Waso
    Wawa
    Weka
    Wile
);
