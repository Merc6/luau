//! the types of tokens produced by the lexer

/// Enum representing common literal kinds.
pub enum LiteralKind {
    /// float literals
    Float {
        /// the base of the number
        base: Base,
    },
    /// integer literals
    Int {
        /// the base of the number
        base: Base,
    },
    /// string literal
    Str {
        /// whether or not the string was terminated
        terminated: bool,
    },
}

/// Enum representing the different bases that can be used in number literals
pub enum Base {
    Binary = 2,
    Decimal = 10,
    Hexademimal = 16,
    Octal = 8,
}

/// Enum representing common lexeme types.
#[non_exhaustive]
#[expect(clippy::arbitrary_source_item_ordering, reason = "semantics")]
pub enum TokenType {
    /// A line comment, e.g. `-- comment`
    LineComment,

    /// Any whitespace character sequence.
    Whitespace,

    /// An identifier or keyword, e.g. `ident` or `continue`.
    Ident,

    /// Literals, e.g. `12`, `1.0e-40`, `"123"`.
    Literal {
        /// The kind of the literal.
        kind: LiteralKind,
        /// Where the suffix may or may not start.
        suffix_start: u32,
    },

    /// `;`
    Semi,
    /// `,`
    Comma,
    /// `.`
    Dot,

    /// `(`
    OpenParen,
    /// `)`
    ClosedParen,
    /// `{`
    OpenBrace,
    /// `}`
    ClosedBrace,
    /// `[`
    OpenBracket,
    /// `]`
    ClosedBracket,
    /// `<`
    OpenAngle,
    /// `>`
    ClosedAngle,

    /// `@`
    At,
    /// `#`
    Pound,
    /// `~`
    Tilde,
    /// `?`
    Question,
    /// `:`
    Colon,
    /// `=`
    Equal,
    /// `-`
    Dash,
    /// `&`
    Ampersand,
    /// `|`
    Bar,
    /// `+`
    Plus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `^`
    Caret,
    /// `%`
    Percent,
}
