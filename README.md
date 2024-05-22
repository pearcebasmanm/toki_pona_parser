# Toki Pona Parser

An experimental parser for the Toki Pona conlang.

## Sentences

A few sentence structures that should be supported:

- [sentence] la [group] li [predicate]
- [group] la [group] li [predicate]
- [group] li [predicate] li [predicate]
- (mi | sina) [predicate]

A few sentence structures that are currently not supported:

- [group] li ni: [sentence]
- (mi | sina) [predicate] li [predicate]

The latter is interpreted as (mi | sina) [modifiers] li [predicate]

## Predicates

Predicates support preverbs (kinda) and prepositions (kinda):

- [group]
- [preverbs] [group]
- [preverbs] [group] e [object]
- [preverbs] [group] [prepositions]
- [preverbs] [group] e [object] e [object] [prepositions]

This form is not currently supported:

- [group] e [object] [prepositions] e [object]
- [group] e [object] [prepositions] e [object] [prepositions]

## Groups

- [word]
- [word] [modifiers]
- [word] pi [group]
- [word] [modifiers] pi [group]

## Other

Other notes:

- Preverbs to not support modifiers (like the "taso" in "mi wile taso pali")
- Ignores punctuation
- Does not support "loaned-modifiers" (and only a limited number of nimi pi pu ala)
- There are likely other flaws I'm missing, as I have no comprehensive tests

