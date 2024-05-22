/*
Only errors or lack thereof are tested, as the output string is not expected to be stable
*/
#[cfg(test)]
mod tests {
    use crate::parsing::parse_toki_pona;

    #[test]
    fn should_parse() {
        // basic sentences
        assert!(parse_toki_pona("mi moku").is_ok());
        assert!(parse_toki_pona("mi li moku").is_ok());
        assert!(parse_toki_pona("mi moku e pan").is_ok());
        assert!(parse_toki_pona("mi li moku e pan").is_ok());

        // imperative
        assert!(parse_toki_pona("sina o moku").is_ok());
        assert!(parse_toki_pona("o moku").is_ok());

        // multiple verbs/objects
        assert!(parse_toki_pona("jan li moku e pan e telo").is_ok());
        assert!(parse_toki_pona("jan li moku e pan li lanpan e telo").is_ok());
        assert!(parse_toki_pona("jan li moku e pan li lanpan").is_ok());
        assert!(parse_toki_pona("jan li moku li lanpan e telo").is_ok());

        // prepositions
        assert!(parse_toki_pona("ona li moku kepeken uta").is_ok());
        assert!(parse_toki_pona("ona li moku kepeken uta lon tomo").is_ok());

        // modifiers & "of"
        assert!(parse_toki_pona("pilin mi li pona").is_ok());
        assert!(parse_toki_pona("pilin mi li pona").is_ok());
        assert!(parse_toki_pona("pilin li pona mute").is_ok());
        assert!(parse_toki_pona("kule mi taso li kule laso loje walo").is_ok());
        assert!(parse_toki_pona("pilin mi li pona mute tan olin sina").is_ok());

        // context
        assert!(parse_toki_pona("mi la pan li pona").is_ok());
        assert!(parse_toki_pona("mi la pan suwi li pona tan kiwen suwi").is_ok());
        assert!(parse_toki_pona("ona li utala e jan pona mi la mi pilin ike").is_ok());
    }

    #[test]
    // not the end of the word if these stop working, but I want to know if so
    fn edge_cases() {
        // recursive context
        assert!(parse_toki_pona("mi la sina la ona la jan li lon").is_ok());

        // ignoring punctuation
        assert!(parse_toki_pona("mi la, pan li pona.").is_ok());
        assert!(parse_toki_pona("...mi, la- -pan li pona¡").is_ok());
    }

    #[test]
    fn should_not_parse() {
        assert!(parse_toki_pona("Hello World!").is_err());
        assert!(parse_toki_pona("la mi moku").is_err());
        assert!(parse_toki_pona("li nasa").is_err());
        assert!(parse_toki_pona("mi moku e").is_err());
    }
}
