/// All tokens of the Holy language.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Testament, Revealing, Scripture, Sin, Covenant, Salm, Upon, Receiving, Reveals,
    Let, There, Be, Of, Become, Hail, Praying, Reveal, Whether, Otherwise,
    So, Litany, For, Confess, Answer, Absolve, As, Transgress, Manifest,
    From, Its, Discern, Amen, Forsake, Ascend, Bearing,
    // Grouping / context markers
    After, Thus,
    // Operators
    Plus, Minus, Times, Over, Remainder, Negate,
    Is, Not, Greater, Lesser, Than, No,
    Blessed, Forsaken, And,
    // Primitive types
    Void, Atom, Fractional, Word, Dogma,
    // Punctuation and indentation
    Comma, Indent, Dedent,
    // Literals
    IntLit(i64),
    FloatLit(f64),
    StrLit(String),
    // Identifier
    Ident(String),
    Eof,
}

/// A token together with its source position.
#[derive(Debug, Clone)]
pub struct Spanned {
    pub token: Token,
    pub line:  usize, // 1-indexed
    pub col:   usize, // 1-indexed
}

impl Spanned {
    pub(super) fn new(token: Token, line: usize, col: usize) -> Self {
        Spanned { token, line, col }
    }
}

/// Human-readable token name used in error messages.
pub fn token_name(t: &Token) -> String {
    match t {
        Token::Ident(n)    => format!("'{}'", n),
        Token::IntLit(n)   => format!("{}", n),
        Token::FloatLit(f) => format!("{}", f),
        Token::StrLit(s)   => format!("\"{}\"", s),
        Token::Eof         => "end of file".into(),
        Token::Indent      => "block start (indent)".into(),
        Token::Dedent      => "block end (dedent)".into(),
        Token::Comma       => "','".into(),
        Token::Testament   => "'testament'".into(),
        Token::Revealing   => "'revealing'".into(),
        Token::Scripture   => "'scripture'".into(),
        Token::Sin         => "'sin'".into(),
        Token::Covenant    => "'covenant'".into(),
        Token::Salm        => "'salm'".into(),
        Token::Upon        => "'upon'".into(),
        Token::Receiving   => "'receiving'".into(),
        Token::Reveals     => "'reveals'".into(),
        Token::Let         => "'let'".into(),
        Token::There       => "'there'".into(),
        Token::Be          => "'be'".into(),
        Token::Of          => "'of'".into(),
        Token::Become      => "'become'".into(),
        Token::Hail        => "'hail'".into(),
        Token::Praying     => "'praying'".into(),
        Token::Reveal      => "'reveal'".into(),
        Token::Whether     => "'whether'".into(),
        Token::Otherwise   => "'otherwise'".into(),
        Token::So          => "'so'".into(),
        Token::Litany      => "'litany'".into(),
        Token::For         => "'for'".into(),
        Token::Confess     => "'confess'".into(),
        Token::Answer      => "'answer'".into(),
        Token::Absolve     => "'absolve'".into(),
        Token::As          => "'as'".into(),
        Token::Transgress  => "'transgress'".into(),
        Token::Manifest    => "'manifest'".into(),
        Token::From        => "'from'".into(),
        Token::Its         => "'its'".into(),
        Token::Discern     => "'discern'".into(),
        Token::Amen        => "'amen'".into(),
        Token::Forsake     => "'forsake'".into(),
        Token::Ascend      => "'ascend'".into(),
        Token::Bearing     => "'bearing'".into(),
        Token::After       => "'after'".into(),
        Token::Thus        => "'thus'".into(),
        Token::Plus        => "'plus'".into(),
        Token::Minus       => "'minus'".into(),
        Token::Times       => "'times'".into(),
        Token::Over        => "'over'".into(),
        Token::Remainder   => "'remainder'".into(),
        Token::Negate      => "'negate'".into(),
        Token::Is          => "'is'".into(),
        Token::Not         => "'not'".into(),
        Token::Greater     => "'greater'".into(),
        Token::Lesser      => "'lesser'".into(),
        Token::Than        => "'than'".into(),
        Token::No          => "'no'".into(),
        Token::Blessed     => "'blessed'".into(),
        Token::Forsaken    => "'forsaken'".into(),
        Token::And         => "'and'".into(),
        Token::Void        => "'void'".into(),
        Token::Atom        => "'atom'".into(),
        Token::Fractional  => "'fractional'".into(),
        Token::Word        => "'word'".into(),
        Token::Dogma       => "'dogma'".into(),
    }
}
