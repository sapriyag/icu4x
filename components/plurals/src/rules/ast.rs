// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! [`AST`](self) provides a set of Syntax Tree Nodes used to store
//! the output of [`parse`] method that is used in [`test_condition`] method
//! to evaluate whether a given [`PluralCategory`] should be used.
//!
//! # Examples
//!
//! ```
//! use icu::plurals::rules::parse_condition;
//! use icu::plurals::rules::ast::*;
//!
//! let input = "i = 1";
//!
//! let ast = parse_condition(input.as_bytes())
//!     .expect("Parsing failed.");
//!
//! assert_eq!(ast, Condition(Box::new([
//!     AndCondition(Box::new([
//!         Relation {
//!             expression: Expression {
//!                 operand: Operand::I,
//!                 modulus: None,
//!             },
//!             operator: Operator::Eq,
//!             range_list: RangeList(Box::new([
//!                 RangeListItem::Value(
//!                     Value(1)
//!                 )
//!             ]))
//!         }
//!     ]))
//! ])));
//! ```
//!
//! [`PluralCategory`]: crate::PluralCategory
//! [`parse`]: super::parse()
//! [`test_condition`]: super::test_condition()
use alloc::boxed::Box;
use alloc::string::String;
use core::ops::RangeInclusive;

/// A complete AST representation of a plural rule.
/// Comprises a vector of [`AndConditions`] and optionally a set of [`Samples`].
///
/// # Examples
///
/// ```
/// use icu::plurals::rules::ast::*;
/// use icu::plurals::rules::{parse, parse_condition};
///
/// let condition = parse_condition(b"i = 5 or v = 2")
///     .expect("Parsing failed.");
///
/// let samples = Samples {
///     integer: Some(SampleList {
///         sample_ranges: Box::new([SampleRange {
///             lower_val: DecimalValue("2".to_string()),
///             upper_val: None,
///         }]),
///         ellipsis: true
///     }),
///     decimal: Some(SampleList {
///         sample_ranges: Box::new([SampleRange {
///             lower_val: DecimalValue("2.5".to_string()),
///             upper_val: None,
///         }]),
///         ellipsis: false
///     }),
/// };
///
/// let rule = Rule {
///     condition,
///     samples: Some(samples),
/// };
///
/// assert_eq!(
///     rule,
///     parse("i = 5 or v = 2 @integer 2, … @decimal 2.5".as_bytes())
///          .expect("Parsing failed")
/// )
/// ```
///
/// [`AndConditions`]: AndCondition
#[derive(Debug, Clone, PartialEq)]
#[allow(missing_docs)] // TODO(#1029) - Add missing docs.
pub struct Rule {
    pub condition: Condition,
    pub samples: Option<Samples>,
}

/// A complete AST representation of a plural rule's condition. Comprises a vector of [`AndConditions`].
///
/// # Examples
///
/// ```
/// use icu::plurals::rules::ast::*;
/// use icu::plurals::rules::parse_condition;
///
/// let condition = Condition(Box::new([
///     AndCondition(Box::new([Relation {
///         expression: Expression {
///             operand: Operand::I,
///             modulus: None,
///         },
///         operator: Operator::Eq,
///         range_list: RangeList(Box::new([RangeListItem::Value(Value(5))])),
///     }])),
///     AndCondition(Box::new([Relation {
///         expression: Expression {
///             operand: Operand::V,
///             modulus: None,
///         },
///         operator: Operator::Eq,
///         range_list: RangeList(Box::new([RangeListItem::Value(Value(2))])),
///     }])),
/// ]));
///
/// assert_eq!(
///     condition,
///     parse_condition(b"i = 5 or v = 2")
///          .expect("Parsing failed")
/// )
/// ```
///
/// [`AndConditions`]: AndCondition
#[derive(Debug, Clone, PartialEq)]
pub struct Condition(pub Box<[AndCondition]>);

/// An incomplete AST representation of a plural rule. Comprises a vector of [`Relations`].
///
/// # Examples
///
/// All AST nodes can be built explicitly, as seen in the example. However, due to its complexity, it is preferred to build the
/// AST using the [`parse()`](crate::rules::parser::parse()) function.
///
/// ```text
/// "i = 3 and v = 0"
/// ```
///
/// Can be represented by the AST:
///
/// ```
/// use icu::plurals::rules::ast::*;
///
/// AndCondition(Box::new([
///     Relation {
///         expression: Expression {
///             operand: Operand::I,
///             modulus: None,
///         },
///         operator: Operator::Eq,
///         range_list: RangeList(Box::new([RangeListItem::Value(Value(5))])),
///     },
///     Relation {
///         expression: Expression {
///             operand: Operand::V,
///             modulus: None,
///         },
///         operator: Operator::NotEq,
///         range_list: RangeList(Box::new([RangeListItem::Value(Value(2))])),
///     },
/// ]));
///
/// ```
///
/// [`Relations`]: Relation
#[derive(Debug, Clone, PartialEq)]
pub struct AndCondition(pub Box<[Relation]>);

/// An incomplete AST representation of a plural rule. Comprises an [`Expression`], an [`Operator`], and a [`RangeList`].
///
/// # Examples
///
/// All AST nodes can be built explicitly, as seen in the example. However, due to its complexity, it is preferred to build the
/// AST using the [`parse()`](crate::rules::parser::parse()) function.
///
/// ```text
/// "i = 3"
/// ```
///
/// Can be represented by the AST:
///
/// ```
/// use icu::plurals::rules::ast::*;
///
/// Relation {
///     expression: Expression {
///         operand: Operand::I,
///         modulus: None,
///     },
///     operator: Operator::Eq,
///     range_list: RangeList(Box::new([RangeListItem::Value(Value(3))])),
/// };
///
/// ```
#[derive(Debug, Clone, PartialEq)]
#[allow(missing_docs)] // TODO(#1029) - Add missing docs.
pub struct Relation {
    pub expression: Expression,
    pub operator: Operator,
    pub range_list: RangeList,
}

/// An enum of [`Relation`] operators for plural rules.
///
/// Each Operator enumeration belongs to the corresponding symbolic operators:
///
/// | Enum Operator | Symbolic Operator |
/// | - | - |
/// | `Eq` | "=" |
/// | `NotEq` | "!=" |
///
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(missing_docs)] // TODO(#1029) - Add missing docs.
pub enum Operator {
    Eq,
    NotEq,
}

/// An incomplete AST representation of a plural rule. Comprises an [`Operand`] and an optional modulo.
///
/// # Examples
///
/// All AST nodes can be built explicitly, as seen in the example. However, due to its complexity, it is preferred to build the
/// AST using the [`parse()`](crate::rules::parser::parse()) function.
///
/// ```text
/// "i % 100"
/// ```
///
/// Can be represented by the AST:
///
/// ```
/// use icu::plurals::rules::ast::*;
///
/// Expression {
///     operand: Operand::I,
///     modulus: Some(Value(100)),
/// };
///
/// ```
#[derive(Debug, Clone, PartialEq)]
#[allow(missing_docs)] // TODO(#1029) - Add missing docs.
pub struct Expression {
    pub operand: Operand,
    pub modulus: Option<Value>,
}

/// An incomplete AST representation of a plural rule. Comprises a [`char`].
///
/// # Examples
///
/// All AST nodes can be built explicitly, as seen in the example. However, due to its complexity, it is preferred to build the
/// AST using the [`parse()`](crate::rules::parser::parse()) function.
///
/// ```text
/// "i"
/// ```
///
/// Can be represented by the AST:
///
/// ```
/// use icu::plurals::rules::ast::Operand;
///
/// Operand::I;
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(missing_docs)] // TODO(#1029) - Add missing docs.
pub enum Operand {
    /// Absolute value of input
    N,
    /// An integer value of input with the fraction part truncated off
    I,
    /// Number of visible fraction digits with trailing zeros
    V,
    /// Number of visible fraction digits without trailing zeros
    W,
    /// Visible fraction digits with trailing zeros
    F,
    /// Visible fraction digits without trailing zeros
    T,
    /// Compact decimal exponent value:
    ///   exponent of the power of 10 used in compact decimal formatting
    C,
    /// Currently, synonym for ‘c’. however, may be redefined in the future
    E,
}

/// An incomplete AST representation of a plural rule. Comprises a vector of [`RangeListItems`].
///
/// # Examples
///
/// All AST nodes can be built explicitly, as seen in the example. However, due to its complexity, it is preferred to build the
/// AST using the [`parse()`](crate::rules::parser::parse()) function.
///
/// ```text
/// "5, 7, 9"
/// ```
///
/// Can be represented by the AST:
///
/// ```
/// use icu::plurals::rules::ast::*;
///
/// RangeList(Box::new([
///     RangeListItem::Value(Value(5)),
///     RangeListItem::Value(Value(7)),
///     RangeListItem::Value(Value(9)),
/// ]));
/// ```
///
/// [`RangeListItems`]: RangeListItem
#[derive(Debug, Clone, PartialEq)]
pub struct RangeList(pub Box<[RangeListItem]>);

/// An enum of items that appear in a [`RangeList`]: `Range` or a `Value`.
///
/// See [`RangeInclusive`] and [`Value`] for additional details.
/// A range comprises two values: an inclusive lower and upper limit.
///
/// # Examples
///
/// ```text
/// 5
/// 11..15
/// ```
///
/// Can be represented by the AST:
///
/// ```
/// use icu::plurals::rules::ast::*;
///
/// let _ = RangeListItem::Value(Value(5));
/// let _ = RangeListItem::Range(Value(11)..=Value(15));
/// ```
#[derive(Debug, Clone, PartialEq)]
#[allow(missing_docs)] // TODO(#1029) - Add missing docs.
pub enum RangeListItem {
    Range(RangeInclusive<Value>),
    Value(Value),
}

/// An incomplete AST representation of a plural rule, representing one integer.
///
/// # Examples
///
/// All AST nodes can be built explicitly, as seen in the example. However, due to its complexity, it is preferred to build the
/// AST using the [`parse()`](crate::rules::parser::parse()) function.
///
/// ```text
/// "99"
/// ```
///
/// Can be represented by the AST:
///
/// ```
/// use icu::plurals::rules::ast::*;
///
/// RangeListItem::Value(Value(99));
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Value(pub u64);

/// A sample of example values that match the given rule.
///
/// # Examples
///
/// ```text
/// @integer 2, … @decimal 2.5
/// ```
///
/// ```
/// use icu::plurals::rules::ast::*;
/// Samples {
///     integer: Some(SampleList {
///         sample_ranges: Box::new([SampleRange {
///             lower_val: DecimalValue("2".to_string()),
///             upper_val: None,
///         }]),
///         ellipsis: true
///     }),
///     decimal: Some(SampleList {
///         sample_ranges: Box::new([SampleRange {
///             lower_val: DecimalValue("2.5".to_string()),
///             upper_val: None,
///         }]),
///         ellipsis: false
///     }),
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
#[allow(missing_docs)] // TODO(#1029) - Add missing docs.
pub struct Samples {
    pub integer: Option<SampleList>,
    pub decimal: Option<SampleList>,
}

/// A list of values used in samples.
///
/// # Examples
///
/// ```text
/// 0.0~1.5, …
/// ```
///
/// ```
/// use icu::plurals::rules::ast::*;
/// SampleList {
///     sample_ranges: Box::new([
///         SampleRange {
///             lower_val: DecimalValue("0.0".to_string()),
///             upper_val: Some(DecimalValue("1.5".to_string())),
///         }
///     ]),
///     ellipsis: true
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
#[allow(missing_docs)] // TODO(#1029) - Add missing docs.
pub struct SampleList {
    pub sample_ranges: Box<[SampleRange]>,
    pub ellipsis: bool,
}

/// A value range used in samples.
///
/// # Examples
///
/// ```text
/// 0.0~1.5
/// ```
///
/// ```
/// use icu::plurals::rules::ast::*;
/// SampleRange {
///     lower_val: DecimalValue("0.0".to_string()),
///     upper_val: Some(DecimalValue("1.5".to_string())),
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
#[allow(missing_docs)] // TODO(#1029) - Add missing docs.
pub struct SampleRange {
    pub lower_val: DecimalValue,
    pub upper_val: Option<DecimalValue>,
}

/// A decimal value used in samples.
///
/// # Examples
///
/// ```text
/// 1.00
/// ```
///
/// ```
/// use icu::plurals::rules::ast::*;
/// DecimalValue("1.00".to_string());
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct DecimalValue(pub String);
