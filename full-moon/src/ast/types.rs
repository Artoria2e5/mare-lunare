//! Contains the types necessary to parse [Roblox's typed Lua](https://devforum.roblox.com/t/luau-type-checking-beta/435382).
//! Only usable when the "roblox" feature flag is enabled.
use super::{punctuated::Punctuated, span::ContainedSpan, *};
use crate::util::display_option;
use derive_more::Display;

/// Any type, such as `string`, `boolean?`, `number | boolean`, etc.
#[derive(Clone, Debug, Display, PartialEq, Owned, Node)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[non_exhaustive]
pub enum TypeInfo<'a> {
    /// A shorthand type annotating the structure of an array: { number }
    #[display(fmt = "{}{}{}", "braces.tokens().0", "type_info", "braces.tokens().1")]
    Array {
        /// The braces (`{}`) containing the type info.
        #[cfg_attr(feature = "serde", serde(borrow))]
        braces: ContainedSpan<'a>,
        /// The type info for the values in the Array
        #[cfg_attr(feature = "serde", serde(borrow))]
        type_info: Box<TypeInfo<'a>>,
    },

    /// A standalone type, such as `string` or `Foo`.
    #[display(fmt = "{}", "_0")]
    Basic(#[cfg_attr(feature = "serde", serde(borrow))] TokenReference<'a>),

    /// A callback type, such as `(string, number) => boolean`.
    #[display(
        fmt = "{}{}{}{}{}",
        "parentheses.tokens().0",
        "arguments",
        "parentheses.tokens().1",
        "arrow",
        "return_type"
    )]
    Callback {
        /// The parentheses for the arguments.
        #[cfg_attr(feature = "serde", serde(borrow))]
        parentheses: ContainedSpan<'a>,
        /// The argument types: `(string, number)`.
        #[cfg_attr(feature = "serde", serde(borrow))]
        arguments: Punctuated<'a, TypeInfo<'a>>,
        /// The "thin arrow" (`->`) in between the arguments and the return type.
        #[cfg_attr(feature = "serde", serde(borrow))]
        arrow: TokenReference<'a>,
        /// The return type: `boolean`.
        #[cfg_attr(feature = "serde", serde(borrow))]
        return_type: Box<TypeInfo<'a>>,
    },

    /// A type using generics, such as `map<number, string>`.
    #[display(
        fmt = "{}{}{}{}",
        "base",
        "arrows.tokens().0",
        "generics",
        "arrows.tokens().1"
    )]
    Generic {
        /// The type that has generics: `map`.
        #[cfg_attr(feature = "serde", serde(borrow))]
        base: TokenReference<'a>,
        /// The arrows (`<>`) containing the type parameters.
        #[cfg_attr(feature = "serde", serde(borrow))]
        arrows: ContainedSpan<'a>,
        /// The type parameters: `number, string`.
        #[cfg_attr(feature = "serde", serde(borrow))]
        generics: Punctuated<'a, TypeInfo<'a>>,
    },

    /// An intersection type: `string & number`, denoting both types.
    #[display(fmt = "{}{}{}", "left", "ampersand", "right")]
    Intersection {
        /// The left hand side: `string`.
        #[cfg_attr(feature = "serde", serde(borrow))]
        left: Box<TypeInfo<'a>>,
        /// The ampersand (`&`) to separate the types.
        #[cfg_attr(feature = "serde", serde(borrow))]
        ampersand: TokenReference<'a>,
        /// The right hand side: `number`.
        #[cfg_attr(feature = "serde", serde(borrow))]
        right: Box<TypeInfo<'a>>,
    },

    /// A type coming from a module, such as `module.Foo`
    #[display(fmt = "{}{}{}", "module", "punctuation", "type_info")]
    Module {
        /// The module the type is coming from: `module`.
        #[cfg_attr(feature = "serde", serde(borrow))]
        module: TokenReference<'a>,
        /// The punctuation (`.`) to index the module.
        #[cfg_attr(feature = "serde", serde(borrow))]
        punctuation: TokenReference<'a>,
        /// The indexed type info: `Foo`.
        #[cfg_attr(feature = "serde", serde(borrow))]
        type_info: Box<IndexedTypeInfo<'a>>,
    },

    /// An optional type, such as `string?`.
    #[display(fmt = "{}{}", "base", "question_mark")]
    Optional {
        /// The type that is optional: `string`.
        #[cfg_attr(feature = "serde", serde(borrow))]
        base: Box<TypeInfo<'a>>,
        /// The question mark: `?`.
        #[cfg_attr(feature = "serde", serde(borrow))]
        question_mark: TokenReference<'a>,
    },

    /// A type annotating the structure of a table: { foo: number, bar: string }
    #[display(fmt = "{}{}{}", "braces.tokens().0", "fields", "braces.tokens().1")]
    Table {
        /// The braces (`{}`) containing the fields.
        #[cfg_attr(feature = "serde", serde(borrow))]
        braces: ContainedSpan<'a>,
        /// The fields: `foo: number, bar: string`.
        #[cfg_attr(feature = "serde", serde(borrow))]
        fields: Punctuated<'a, TypeField<'a>>,
    },

    /// A type in the form of `typeof(foo)`.
    #[display(
        fmt = "{}{}{}{}",
        "typeof_token",
        "parentheses.tokens().0",
        "inner",
        "parentheses.tokens().1"
    )]
    Typeof {
        /// The token `typeof`.
        #[cfg_attr(feature = "serde", serde(borrow))]
        typeof_token: TokenReference<'a>,
        /// The parentheses used to contain the expression.
        #[cfg_attr(feature = "serde", serde(borrow))]
        parentheses: ContainedSpan<'a>,
        /// The inner expression: `foo`.
        #[cfg_attr(feature = "serde", serde(borrow))]
        inner: Box<Expression<'a>>,
    },

    /// A tuple expression: `(string, number)`.
    #[display(
        fmt = "{}{}{}",
        "parentheses.tokens().0",
        "types",
        "parentheses.tokens().1"
    )]
    Tuple {
        /// The parentheses used to contain the types
        #[cfg_attr(feature = "serde", serde(borrow))]
        parentheses: ContainedSpan<'a>,
        /// The types: `(string, number)`.
        #[cfg_attr(feature = "serde", serde(borrow))]
        types: Punctuated<'a, TypeInfo<'a>>,
    },

    /// A union type: `string | number`, denoting one or the other.
    #[display(fmt = "{}{}{}", "left", "pipe", "right")]
    Union {
        /// The left hand side: `string`.
        #[cfg_attr(feature = "serde", serde(borrow))]
        left: Box<TypeInfo<'a>>,
        /// The pipe (`|`) to separate the types.
        #[cfg_attr(feature = "serde", serde(borrow))]
        pipe: TokenReference<'a>,
        /// The right hand side: `number`.
        #[cfg_attr(feature = "serde", serde(borrow))]
        right: Box<TypeInfo<'a>>,
    },

    /// A variadic type: `...number`.
    #[display(fmt = "{}{}", "ellipse", "type_info")]
    Variadic {
        /// The ellipse: `...`.
        #[cfg_attr(feature = "serde", serde(borrow))]
        ellipse: TokenReference<'a>,
        /// The type that is variadic: `number`.
        #[cfg_attr(feature = "serde", serde(borrow))]
        type_info: Box<TypeInfo<'a>>,
    },
}

/// A subset of TypeInfo that consists of items which can only be used as an index, such as `Foo` and `Foo<Bar>`,
#[derive(Clone, Debug, Display, PartialEq, Owned, Node)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[non_exhaustive]
pub enum IndexedTypeInfo<'a> {
    /// A standalone type, such as `string` or `Foo`.
    #[display(fmt = "{}", "_0")]
    Basic(#[cfg_attr(feature = "serde", serde(borrow))] TokenReference<'a>),

    /// A type using generics, such as `map<number, string>`.
    #[display(
        fmt = "{}{}{}{}",
        "base",
        "arrows.tokens().0",
        "generics",
        "arrows.tokens().1"
    )]
    Generic {
        /// The type that has generics: `map`.
        #[cfg_attr(feature = "serde", serde(borrow))]
        base: TokenReference<'a>,
        /// The arrows (`<>`) containing the type parameters.
        #[cfg_attr(feature = "serde", serde(borrow))]
        arrows: ContainedSpan<'a>,
        /// The type parameters: `number, string`.
        #[cfg_attr(feature = "serde", serde(borrow))]
        generics: Punctuated<'a, TypeInfo<'a>>,
    },
}

/// A type field used within table types.
/// The `foo: number` in `{ foo: number }`.
#[derive(Clone, Debug, Display, PartialEq, Owned, Node, Visit)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[display(fmt = "{}{}{}", "key", "colon", "value")]
pub struct TypeField<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub(crate) key: TypeFieldKey<'a>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub(crate) colon: TokenReference<'a>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub(crate) value: TypeInfo<'a>,
}

impl<'a> TypeField<'a> {
    /// Creates a new TypeField from the given key and value
    pub fn new(key: TypeFieldKey<'a>, value: TypeInfo<'a>) -> Self {
        Self {
            key,
            colon: TokenReference::symbol(": ").unwrap(),
            value,
        }
    }

    /// The key of the field, `foo` in `foo: number`.
    pub fn key(&self) -> &TypeFieldKey<'a> {
        &self.key
    }

    /// The colon in between the key name and the value type.
    pub fn colon_token(&self) -> &TokenReference<'a> {
        &self.colon
    }

    /// The type for the field, `number` in `foo: number`.
    pub fn value(&self) -> &TypeInfo<'a> {
        &self.value
    }

    /// Returns a new TypeField with the given key
    pub fn with_key(self, key: TypeFieldKey<'a>) -> Self {
        Self { key, ..self }
    }

    /// Returns a new TypeField with the `:` token
    pub fn with_colon_token(self, colon_token: TokenReference<'a>) -> Self {
        Self {
            colon: colon_token,
            ..self
        }
    }

    /// Returns a new TypeField with the `:` token
    pub fn with_value(self, value: TypeInfo<'a>) -> Self {
        Self { value, ..self }
    }
}

/// A key in a [`TypeField`]. Can either be a name or an index signature.
#[derive(Clone, Debug, Display, PartialEq, Owned, Node)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[non_exhaustive]
pub enum TypeFieldKey<'a> {
    /// A name, such as `foo`.
    #[display(fmt = "{}", "_0")]
    Name(TokenReference<'a>),

    /// An index signature, such as `[number]`.
    #[display(fmt = "{}{}{}", "brackets.tokens().0", "inner", "brackets.tokens().1")]
    IndexSignature {
        /// The brackets (`[]`) used to contain the type.
        #[cfg_attr(feature = "serde", serde(borrow))]
        brackets: ContainedSpan<'a>,

        /// The type for the index signature, `number` in `[number]`.
        #[cfg_attr(feature = "serde", serde(borrow))]
        inner: TypeInfo<'a>,
    },
}

/// A type assertion using `::`, such as `:: number`.
#[derive(Clone, Debug, Display, PartialEq, Owned, Node, Visit)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[display(fmt = "{}{}", "assertion_op", "cast_to")]
pub struct TypeAssertion<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub(crate) assertion_op: TokenReference<'a>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub(crate) cast_to: TypeInfo<'a>,
}

impl<'a> TypeAssertion<'a> {
    /// Creates a new TypeAssertion from the given cast to TypeInfo
    pub fn new(cast_to: TypeInfo<'a>) -> Self {
        Self {
            assertion_op: TokenReference::symbol("::").unwrap(),
            cast_to,
        }
    }

    /// The token `::`.
    pub fn assertion_op(&self) -> &TokenReference<'a> {
        &self.assertion_op
    }

    /// The type to cast the expression into, `number` in `:: number`.
    pub fn cast_to(&self) -> &TypeInfo<'a> {
        &self.cast_to
    }

    /// Returns a new TypeAssertion with the given `::` token
    pub fn with_assertion_op(self, assertion_op: TokenReference<'a>) -> Self {
        Self {
            assertion_op,
            ..self
        }
    }

    /// Returns a new TypeAssertion with the given TypeInfo to cast to
    pub fn with_cast_to(self, cast_to: TypeInfo<'a>) -> Self {
        Self { cast_to, ..self }
    }
}

/// A type declaration, such as `type Meters = number`
#[derive(Clone, Debug, Display, PartialEq, Owned, Node, Visit)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[display(
    fmt = "{}{}{}{}{}",
    "type_token",
    "base",
    "display_option(generics)",
    "equal_token",
    "declare_as"
)]
pub struct TypeDeclaration<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub(crate) type_token: TokenReference<'a>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub(crate) base: TokenReference<'a>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub(crate) generics: Option<GenericDeclaration<'a>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub(crate) equal_token: TokenReference<'a>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub(crate) declare_as: TypeInfo<'a>,
}

impl<'a> TypeDeclaration<'a> {
    /// Creates a new TypeDeclaration from the given type name and type declaration
    pub fn new(type_name: TokenReference<'a>, type_definition: TypeInfo<'a>) -> Self {
        Self {
            type_token: TokenReference::symbol("type ").unwrap(),
            base: type_name,
            generics: None,
            equal_token: TokenReference::symbol(" = ").unwrap(),
            declare_as: type_definition,
        }
    }

    /// The token `type`.
    pub fn type_token(&self) -> &TokenReference<'a> {
        &self.type_token
    }

    /// The name of the type, `Meters` in `type Meters = number`.
    pub fn type_name(&self) -> &TokenReference<'a> {
        &self.base
    }

    /// The generics of the type, if there are any. `<T>` in `type Foo<T> = T`.
    pub fn generics(&self) -> Option<&GenericDeclaration<'a>> {
        self.generics.as_ref()
    }

    /// The `=` token in between the type name and the definition.
    pub fn equal_token(&self) -> &TokenReference<'a> {
        &self.equal_token
    }

    /// The definition of the type, `number` in `type Meters = number`.
    pub fn type_definition(&self) -> &TypeInfo<'a> {
        &self.declare_as
    }

    /// Returns a new TypeDeclaration with the given `type` token
    pub fn with_type_token(self, type_token: TokenReference<'a>) -> Self {
        Self { type_token, ..self }
    }

    /// Returns a new TypeDeclaration with the given type name
    pub fn with_type_name(self, type_name: TokenReference<'a>) -> Self {
        Self {
            base: type_name,
            ..self
        }
    }

    /// Returns a new TypeDeclaration with the given generics of the type
    pub fn with_generics(self, generics: Option<GenericDeclaration<'a>>) -> Self {
        Self { generics, ..self }
    }

    /// Returns a new TypeDeclaration with the given generics of the type
    pub fn with_equal_token(self, equal_token: TokenReference<'a>) -> Self {
        Self {
            equal_token,
            ..self
        }
    }

    /// Returns a new TypeDeclaration with the given generics of the type
    pub fn with_type_definition(self, type_definition: TypeInfo<'a>) -> Self {
        Self {
            declare_as: type_definition,
            ..self
        }
    }
}

/// The generics used in a [`TypeDeclaration`].
#[derive(Clone, Debug, Display, PartialEq, Owned, Node, Visit)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[display(fmt = "{}{}{}", "arrows.tokens().0", "generics", "arrows.tokens().1")]
pub struct GenericDeclaration<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[visit(contains = "generics")]
    pub(crate) arrows: ContainedSpan<'a>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub(crate) generics: Punctuated<'a, TokenReference<'a>>,
}

impl<'a> GenericDeclaration<'a> {
    /// Creates a new GenericDeclaration
    pub fn new() -> Self {
        Self {
            arrows: ContainedSpan::new(
                TokenReference::symbol("<").unwrap(),
                TokenReference::symbol(">").unwrap(),
            ),
            generics: Punctuated::new(),
        }
    }

    /// The arrows (`<>`) containing the types.
    pub fn arrows(&self) -> &ContainedSpan<'a> {
        &self.arrows
    }

    /// The names of the generics: `T, U` in `<T, U>`.
    pub fn generics(&self) -> &Punctuated<'a, TokenReference<'a>> {
        &self.generics
    }

    /// Returns a new GenericDeclaration with the given arrows containing the types
    pub fn with_arrows(self, arrows: ContainedSpan<'a>) -> Self {
        Self { arrows, ..self }
    }

    /// Returns a new TypeDeclaration with the given names of the generics
    pub fn with_generics(self, generics: Punctuated<'a, TokenReference<'a>>) -> Self {
        Self { generics, ..self }
    }
}

/// A type specifier, the `: number` in `local foo: number`
#[derive(Clone, Debug, Display, PartialEq, Owned, Node, Visit)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[display(fmt = "{}{}", "punctuation", "type_info")]
pub struct TypeSpecifier<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub(crate) punctuation: TokenReference<'a>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub(crate) type_info: TypeInfo<'a>,
}

impl<'a> TypeSpecifier<'a> {
    /// Creates a new TypeSpecifier with the given type info
    pub fn new(type_info: TypeInfo<'a>) -> Self {
        Self {
            punctuation: TokenReference::symbol(": ").unwrap(),
            type_info,
        }
    }

    /// The punctuation being used.
    /// `:` for `local foo: number`.
    pub fn punctuation(&self) -> &TokenReference<'a> {
        &self.punctuation
    }

    /// The type being specified: `number` in `local foo: number`.
    pub fn type_info(&self) -> &TypeInfo<'a> {
        &self.type_info
    }

    /// Returns a new TypeSpecifier with the given punctuation
    pub fn with_punctuation(self, punctuation: TokenReference<'a>) -> Self {
        Self {
            punctuation,
            ..self
        }
    }

    /// Returns a new TypeSpecifier with the given type being specified
    pub fn with_type_info(self, type_info: TypeInfo<'a>) -> Self {
        Self { type_info, ..self }
    }
}

/// An exported type declaration, such as `export type Meters = number`
#[derive(Clone, Debug, Display, PartialEq, Owned, Node, Visit)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[display(fmt = "{}{}", "export_token", "type_declaration")]
pub struct ExportedTypeDeclaration<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub(crate) export_token: TokenReference<'a>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub(crate) type_declaration: TypeDeclaration<'a>,
}

impl<'a> ExportedTypeDeclaration<'a> {
    /// Creates a new ExportedTypeDeclaration with the given type declaration
    pub fn new(type_declaration: TypeDeclaration<'a>) -> Self {
        Self {
            export_token: TokenReference::new(
                vec![],
                Token::new(TokenType::Identifier {
                    identifier: Cow::Owned(String::from("export")),
                }),
                vec![Token::new(TokenType::spaces(1))],
            ),
            type_declaration,
        }
    }

    /// The token `export`.
    pub fn export_token(&self) -> &TokenReference<'a> {
        &self.export_token
    }

    /// The type declaration, `type Meters = number`.
    pub fn type_declaration(&self) -> &TypeDeclaration<'a> {
        &self.type_declaration
    }

    /// Returns a new ExportedTypeDeclaration with the `export` token
    pub fn with_export_token(self, export_token: TokenReference<'a>) -> Self {
        Self {
            export_token,
            ..self
        }
    }

    /// Returns a new TypeDeclaration with the given type declaration
    pub fn with_type_declaration(self, type_declaration: TypeDeclaration<'a>) -> Self {
        Self {
            type_declaration,
            ..self
        }
    }
}

make_op!(CompoundOp,
    #[doc = "Compound operators, such as X += Y or X -= Y"]
    {
        PlusEqual,
        MinusEqual,
        StarEqual,
        SlashEqual,
        PercentEqual,
        CaretEqual,
        TwoDotsEqual,
    }
);

/// A Compound Assignment statement, such as `x += 1` or `x -= 1`
#[derive(Clone, Debug, Display, PartialEq, Owned, Node, Visit)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[display(fmt = "{}{}{}", "lhs", "compound_operator", "rhs")]
pub struct CompoundAssignment<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub(crate) lhs: Var<'a>,
    pub(crate) compound_operator: CompoundOp<'a>,
    pub(crate) rhs: Expression<'a>,
}

impl<'a> CompoundAssignment<'a> {
    /// Creates a new CompoundAssignment from the left and right hand side
    pub fn new(lhs: Var<'a>, compound_operator: CompoundOp<'a>, rhs: Expression<'a>) -> Self {
        Self {
            lhs,
            compound_operator,
            rhs,
        }
    }

    /// The variable assigned to, the `x` part of `x += 1`
    pub fn lhs(&self) -> &Var<'a> {
        &self.lhs
    }

    /// The operator used, the `+=` part of `x += 1`
    pub fn compound_operator(&self) -> &CompoundOp<'a> {
        &self.compound_operator
    }

    /// The value being assigned, the `1` part of `x += 1`
    pub fn rhs(&self) -> &Expression<'a> {
        &self.rhs
    }

    /// Returns a new CompoundAssignment with the given variable being assigned to
    pub fn with_lhs(self, lhs: Var<'a>) -> Self {
        Self { lhs, ..self }
    }

    /// Returns a new CompoundAssignment with the given operator used
    pub fn with_compound_operator(self, compound_operator: CompoundOp<'a>) -> Self {
        Self {
            compound_operator,
            ..self
        }
    }

    /// Returns a new CompoundAssignment with the given value being assigned
    pub fn with_rhs(self, rhs: Expression<'a>) -> Self {
        Self { rhs, ..self }
    }
}
