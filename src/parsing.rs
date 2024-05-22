use std::collections::VecDeque;

use crate::{
    types::{Context, Group, ParseTPError, Predicate, Preposition, Sentence, VerbPhrase},
    word::{ParseWordError, Word},
};

pub fn parse_toki_pona(input: &str) -> Result<Sentence, ParseTPError> {
    let tokens: Vec<Word> = input
        .split_whitespace()
        .map(|word| {
            word.trim_matches(|c: char| !c.is_ascii_alphabetic())
                .parse::<Word>()
        })
        .collect::<Result<Vec<Word>, ParseWordError>>()?;
    parse_sentence(&tokens)
}

pub fn parse_sentence(mut tokens: &[Word]) -> Result<Sentence, ParseTPError> {
    let context = if let Some(index) = tokens.iter().rev().position(|&word| word == Word::La) {
        let index = tokens.len() - 1 - index;
        let context = parse_fragment(&tokens[..index])?;
        tokens = &tokens[index + 1..];
        Some(Box::new(context))
    } else {
        None
    };

    let subjects = if let Some(index) = tokens.iter().position(Word::is_predicate_marker) {
        let subjects = if index == 0 {
            if tokens[index] == Word::O {
                vec![Group::from_word(Word::Sina)]
            } else {
                // if not `o`, then is `li`
                return Err(ParseTPError::EmptySubject);
            }
        } else {
            tokens[..index]
                .split(|&word| word == Word::En)
                .map(parse_group)
                .collect::<Result<Vec<Group>, ParseTPError>>()?
        };
        tokens = &tokens[index..];
        subjects
    } else if tokens
        .first()
        .is_some_and(|word| [Word::Mi, Word::Sina].contains(word))
    {
        let subject = tokens[0];
        tokens = &tokens[1..];
        vec![Group::from_word(subject)]
    } else {
        let subject = parse_group(tokens)?;
        tokens = &[];
        vec![subject]
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
        Context::NounPhrase(parse_group(input)?)
    };
    Ok(fragment)
}

fn parse_group(mut tokens: &[Word]) -> Result<Group, ParseTPError> {
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
        (modifiers, Some(Box::new(parse_group(of)?)))
    } else {
        (tokens, None)
    };
    let modifiers = modifiers.to_vec();

    Ok(Group {
        word: noun,
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
        let start_of_predicate = tokens
            .iter()
            .rev()
            .position(|&word| word == Word::E)
            .map(|i| tokens.len() - 1 - i)
            .unwrap_or(0);
        (start_of_predicate + 2..=tokens.len().saturating_sub(2)).contains(&last_preposition)
    };

    while tokens
        .iter()
        .rev()
        .position(Word::is_preposition)
        .map(|i| tokens.len() - 1 - i)
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
            .map(parse_group)
            .collect::<Result<Vec<Group>, ParseTPError>>()?;
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
    let verb = parse_group(tokens)?;
    Ok(VerbPhrase { preverbs, verb })
}

fn parse_preposition(tokens: &[Word]) -> Result<Preposition, ParseTPError> {
    if tokens.is_empty() {
        return Err(ParseTPError::EmptyPreposition);
    }

    let preposition = tokens[0];
    let noun = parse_group(&tokens[1..])?;

    if !preposition.is_preposition() {
        return Err(ParseTPError::InvalidPreposition(preposition));
    }

    Ok(Preposition { preposition, noun })
}
