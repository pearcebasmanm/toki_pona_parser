use std::fmt::{Display, Write};

use crate::word::{ParseWordError, Word};

use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum ParseTPError {
    EmptyNounPhrase,
    EmptyContext,
    EmptySubject,
    EmptyPredicate,
    EmptyVerbPhrase,
    EmptyPreposition,
    InvalidSubject,
    InvalidPreposition(Word),
    InvalidNoun(Word),
    InvalidWord(#[from] ParseWordError),
}

impl Display for ParseTPError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug)]
pub enum Context {
    Sentence(Sentence),
    NounPhrase(Group),
}

impl Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sentence(sentence) => write!(f, "({})", sentence),
            Self::NounPhrase(noun_phrase) => write!(f, "{}", noun_phrase),
        }
    }
}

#[derive(Debug)]
pub struct Sentence {
    pub context: Option<Box<Context>>,
    pub subjects: Vec<Group>,
    pub predicates: Vec<Predicate>,
}

impl Display for Sentence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            if let Some(context) = self.context.as_ref() {
                format!("{context}la")
            } else {
                String::new()
            },
            self.subjects
                .iter()
                .map(|subject| subject.to_string())
                .collect::<Vec<String>>()
                .join("en"),
            self.predicates
                .iter()
                .map(|predicate| predicate.to_string())
                .collect::<String>()
        )
    }
}

#[derive(Debug)]
pub struct Predicate {
    pub imperative: bool,
    pub verb: VerbPhrase,
    pub objects: Vec<Group>,
    pub prepositions: Vec<Preposition>,
}

impl Display for Predicate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            if self.imperative { "o" } else { "li" },
            self.verb,
            self.objects
                .iter()
                .fold(String::new(), |mut output, object| {
                    _ = write!(output, "e{object}");
                    output
                }),
            self.prepositions
                .iter()
                .map(|preposition| preposition.to_string())
                .collect::<String>()
        )
    }
}

#[derive(Debug)]
pub struct VerbPhrase {
    pub preverbs: Vec<Word>,
    pub verb: Group,
}

impl Display for VerbPhrase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.preverbs
                .iter()
                .map(|preverb| "(".to_string() + &preverb.to_string())
                .collect::<String>(),
            self.verb,
            ")".repeat(self.preverbs.len()),
        )
    }
}

#[derive(Debug)]
pub struct Group {
    pub word: Word,
    pub modifiers: Vec<Word>,
    pub of: Option<Box<Group>>,
}

impl Group {
    pub fn from_word(word: Word) -> Self {
        Self {
            word,
            modifiers: Vec::new(),
            of: None,
        }
    }
}

impl Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}({}){}{}",
            if self.of.is_some() { "(" } else { "" },
            "(".repeat(self.modifiers.len()),
            self.word,
            self.modifiers
                .iter()
                .map(|modifier| modifier.to_string() + ")")
                .collect::<String>(),
            if let Some(of) = self.of.as_ref() {
                format!("pi{of})")
            } else {
                String::new()
            },
        )
    }
}

#[derive(Debug)]
pub struct Preposition {
    pub preposition: Word,
    pub noun: Group,
}

impl Display for Preposition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.preposition, self.noun)
    }
}
