#![feature(slice_split_once)]

mod word;
use word::Word;

use std::{
    collections::VecDeque,
    error::Error,
    fmt::{Display, Write},
    io::stdin,
};

fn main() {
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let tokens: Vec<Word> = input
            .split_whitespace()
            .map(|word| {
                word.trim_matches(|c: char| !c.is_ascii_alphabetic())
                    .parse()
                    .unwrap()
            })
            .collect();
        let output = parse_sentence(&tokens);
        match output {
            Ok(result) => println!("{result}"),
            Err(error) => println!("{error}"),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum ParseTPError {
    EmptyNounPhrase,
    EmptyContext,
    EmptySubject,
    EmptyPredicate,
    EmptyVerbPhrase,
    EmptyPreposition,
    InvalidSubject,
    InvalidPreposition(Word),
    InvalidNoun(Word),
}

impl Display for ParseTPError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for ParseTPError {}

fn parse_sentence(mut tokens: &[Word]) -> Result<Sentence, ParseTPError> {
    let context = if let Some(index) = tokens.iter().position(|&word| word == Word::La) {
        let context = parse_fragment(&tokens[..index])?;
        tokens = &tokens[index + 1..];
        Some(Box::new(context))
    } else {
        None
    };

    let subjects = if let Some(index) = tokens.iter().position(Word::is_predicate_marker) {
        if index == 0 && tokens[index] != Word::O {
            return Err(ParseTPError::EmptySubject);
        }
        let subjects = tokens[..index]
            .split(|&word| word == Word::En)
            .map(parse_noun_phrase)
            .collect::<Result<Vec<NounPhrase>, ParseTPError>>()?;
        tokens = &tokens[index..];
        subjects
    } else if tokens
        .first()
        .is_some_and(|word| [Word::Mi, Word::Sina].contains(word))
    {
        let subject = tokens[0];
        tokens = &tokens[1..];
        vec![NounPhrase {
            noun: subject,
            modifiers: Vec::new(),
            of: None,
        }]
    } else {
        return Err(ParseTPError::InvalidSubject);
    };

    let mut predicates = Vec::new();

    while !tokens.is_empty() {
        let index = tokens[1..]
            .iter()
            .position(Word::is_predicate_marker)
            .map(|index| index + 1)
            .unwrap_or(tokens.len());

        predicates.push(parse_predicate(&tokens[..index])?);
        tokens = &tokens[index..];
    }

    Ok(Sentence {
        context,
        subjects,
        predicates,
    })
}

fn parse_fragment(input: &[Word]) -> Result<Context, ParseTPError> {
    if input.is_empty() {
        return Err(ParseTPError::EmptyContext);
    }
    let fragment = if input.iter().any(Word::is_predicate_marker) {
        Context::Sentence(parse_sentence(input)?)
    } else {
        Context::NounPhrase(parse_noun_phrase(input)?)
    };
    Ok(fragment)
}

fn parse_noun_phrase(mut tokens: &[Word]) -> Result<NounPhrase, ParseTPError> {
    if tokens.is_empty() {
        return Err(ParseTPError::EmptyNounPhrase);
    }

    let noun = tokens[0];
    tokens = &tokens[1..];

    if noun.is_particle() {
        return Err(ParseTPError::InvalidNoun(noun));
    }

    let (modifiers, of) = if let Some((modifiers, of)) = tokens.split_once(|&word| word == Word::Pi)
    {
        (modifiers, Some(Box::new(parse_noun_phrase(of)?)))
    } else {
        (tokens, None)
    };
    let modifiers = modifiers.to_vec();

    Ok(NounPhrase {
        noun,
        modifiers,
        of,
    })
}

fn parse_predicate(mut tokens: &[Word]) -> Result<Predicate, ParseTPError> {
    if tokens.is_empty() {
        return Err(ParseTPError::EmptyPredicate);
    }

    let imperative = tokens[0] == Word::O;
    if tokens[0].is_predicate_marker() {
        tokens = &tokens[1..];
    }

    let mut prepositions = VecDeque::new();

    let is_valid_preposition = |last_preposition| {
        let last_e = tokens
            .iter()
            .rev()
            .position(|&word| word == Word::E)
            .unwrap_or(tokens.len());
        last_e > last_preposition + 1 && last_preposition < tokens.len() - 1
    };

    while tokens
        .iter()
        .rev()
        .position(Word::is_preposition)
        .is_some_and(is_valid_preposition)
    {
        let last_preposition_start =
            tokens.len() - 1 - tokens.iter().rev().position(Word::is_preposition).unwrap();
        let preposition = parse_preposition(&tokens[last_preposition_start..])?;
        prepositions.push_front(preposition);
        tokens = &tokens[..last_preposition_start];
    }

    let (verb, objects) = if let Some(index) = tokens.iter().position(|&word| word == Word::E) {
        let verb = parse_verb_phrase(&tokens[..index])?;
        tokens = &tokens[index + 1..];
        let objects = tokens
            .split(|&token| token == Word::E)
            .map(parse_noun_phrase)
            .collect::<Result<Vec<NounPhrase>, ParseTPError>>()?;
        (verb, objects)
    } else {
        (parse_verb_phrase(tokens)?, Vec::new())
    };

    Ok(Predicate {
        imperative,
        verb,
        objects,
        prepositions: prepositions.into(),
    })
}

fn parse_verb_phrase(mut tokens: &[Word]) -> Result<VerbPhrase, ParseTPError> {
    if tokens.is_empty() {
        return Err(ParseTPError::EmptyVerbPhrase);
    }
    let mut preverbs = Vec::new();
    while tokens[0].is_preverb() && tokens.len() > 1 {
        preverbs.push(tokens[0]);
        tokens = &tokens[1..];
    }
    let verb = tokens[0];
    let modifiers = tokens[1..].to_vec();
    Ok(VerbPhrase {
        preverbs,
        verb,
        modifiers,
    })
}

fn parse_preposition(tokens: &[Word]) -> Result<Preposition, ParseTPError> {
    if tokens.is_empty() {
        return Err(ParseTPError::EmptyPreposition);
    }

    let preposition = tokens[0];
    let noun = parse_noun_phrase(&tokens[1..])?;

    if !preposition.is_preposition() {
        return Err(ParseTPError::InvalidPreposition(preposition));
    }

    Ok(Preposition { preposition, noun })
}

#[derive(Debug)]
enum Context {
    Sentence(Sentence),
    NounPhrase(NounPhrase),
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
struct Sentence {
    context: Option<Box<Context>>,
    subjects: Vec<NounPhrase>,
    predicates: Vec<Predicate>,
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
struct Predicate {
    imperative: bool,
    verb: VerbPhrase,
    objects: Vec<NounPhrase>,
    prepositions: Vec<Preposition>,
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
struct VerbPhrase {
    preverbs: Vec<Word>,
    verb: Word,
    modifiers: Vec<Word>,
}

impl Display for VerbPhrase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}({}){}{}",
            "(".repeat(self.modifiers.len()),
            self.preverbs
                .iter()
                .map(|preverb| "(".to_string() + &preverb.to_string())
                .collect::<String>(),
            self.verb,
            ")".repeat(self.preverbs.len()),
            self.modifiers
                .iter()
                .map(|modifier| modifier.to_string() + ")")
                .collect::<String>()
        )
    }
}

#[derive(Debug)]
struct NounPhrase {
    noun: Word,
    modifiers: Vec<Word>,
    of: Option<Box<NounPhrase>>,
}

impl Display for NounPhrase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}({}){}{}",
            if self.of.is_some() { "(" } else { "" },
            "(".repeat(self.modifiers.len()),
            self.noun,
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
struct Preposition {
    preposition: Word,
    noun: NounPhrase,
}

impl Display for Preposition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.preposition, self.noun)
    }
}
