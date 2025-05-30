// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::error::Error;
use std::fmt;

use std::num::{ParseFloatError, ParseIntError};

use pars_symb::token::Token;
////

#[derive(Debug, Clone)]
pub struct BlockError {
    source: String
}

impl BlockError {
    pub fn new(src: &str) -> BlockError {
        BlockError {
            source: src.to_string()
        }
    }
}


impl fmt::Display for BlockError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.source)
    }
}

impl Error for BlockError {}

////

#[derive(Debug, Clone)]
pub struct ParseError {
    details: String,
    token:   Token
}

impl ParseError {
    pub fn new(msg: &str, tok: Token) -> ParseError {
        ParseError {
            details: msg.to_string(),
            token:   tok
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.details, self.token)
    }
}

impl Error for ParseError {}

////

#[derive(Debug, Clone)]
pub struct TypeError {
    source: String,
    details: String
}

impl TypeError {
    pub fn new(src: &str, msg: &str) -> TypeError {
        TypeError {
            source: src.to_string(),
            details: msg.to_string()
        }
    }
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.source, self.details)
    }
}

impl Error for TypeError {}

////

#[derive(Debug, Clone)]
    pub struct UnboundVariableError {
    source: String
}

impl UnboundVariableError {
    pub fn new(src: &str) -> UnboundVariableError {
        UnboundVariableError {
            source: src.to_string() }
    }
}

impl fmt::Display for UnboundVariableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.source)
    }
}

impl Error for UnboundVariableError {}

////

#[derive(Debug, Clone)]
    pub struct UndefinedFuncError {
    details: String
}

impl UndefinedFuncError {
    pub fn new(msg: &str) -> UndefinedFuncError {
        UndefinedFuncError {
            details: msg.to_string() }
    }
}

impl fmt::Display for UndefinedFuncError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for UndefinedFuncError {}

////

#[derive(Debug, Clone)]
    pub struct CustomParseFloatError {
    source: String,
    details: String,
    error: ParseFloatError
}

impl CustomParseFloatError {
    pub fn new(src: &str, msg: &str, err: ParseFloatError) ->
        CustomParseFloatError {
            CustomParseFloatError {
                source:  src.to_string(),
                details: msg.to_string(),
                error:   err
            }
       }
}

impl fmt::Display for CustomParseFloatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.source, self.details, self.error)
    }
}

impl Error for CustomParseFloatError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
           CustomParseFloatError { error, .. } => Some(error),
       }
   }
}

////

#[derive(Debug, Clone)]
pub struct ReturnFromError {
    block: String
}

impl ReturnFromError {
    pub fn new(block: &str) -> ReturnFromError {
        ReturnFromError {
            block: block.to_string()
        }
    }
}

impl fmt::Display for ReturnFromError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.block)
    }
}

impl Error for ReturnFromError {}

////

#[derive(Debug, Clone)]
pub struct SimpleError {
    details: String
}

impl SimpleError {
    pub fn new(msg: &str) -> SimpleError {
        SimpleError {
            details: msg.to_string()
        }
    }
}

impl fmt::Display for SimpleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for SimpleError {}

////

#[derive(Debug, Clone)]
pub struct SimpleTypeError {
    source: String,
    details: String
}

impl SimpleTypeError {
    pub fn new(src: &str, msg: &str) -> SimpleTypeError {
        SimpleTypeError {
            source: src.to_string(),
            details: msg.to_string()
        }
    }
}

impl fmt::Display for SimpleTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.source, self.details)
    }
}

impl Error for SimpleTypeError {}

////

#[derive(Debug, Clone)]
pub enum RLError {
    BlockError(BlockError),
    DivisionByZero,
    ParseError(ParseError),
    TypeError(TypeError),
    ParseFloatError(CustomParseFloatError),
    ParseIntError(ParseIntError),
    ReturnFromError(ReturnFromError),
    SimpleError(SimpleError),
    SimpleProgramError,
    SimpleTypeError(SimpleTypeError),
    UnboundVariableError(UnboundVariableError),
    UndefinedFuncError(UndefinedFuncError),
}

impl fmt::Display for RLError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
        RLError::BlockError(block_error) =>
            write!(f, "The block name {} is not a symbol.",
                block_error.source),

        RLError::DivisionByZero =>
            write!(f, "DIVISION-BY-ZERO"),

        RLError::ParseError(parse_error) =>
            write!(f, "PARSE-ERROR {} - bad token: {}",
                parse_error.details,
                parse_error.token),

        RLError::TypeError(type_error) =>
            write!(f, "TYPE-ERROR expected-type: {} datum: {}",
                type_error.details, type_error.source),

        RLError::ParseFloatError(custom_parse_float_error) =>
            write!(f, "TYPE-ERROR expected-type: {} datum: {} -> {}",
                custom_parse_float_error.details,
                custom_parse_float_error.source,
                custom_parse_float_error.error),

        RLError::ParseIntError(parse_int_error) =>
            write!(f, "TYPE-ERROR {}", parse_int_error),

        RLError::ReturnFromError(return_from_error) =>
            write!(f, "Return for unknown block: {}",
                return_from_error.block.to_uppercase()),

        RLError::SimpleError(simple_error) =>
            write!(f, "SIMPLE-ERROR {}", simple_error.details),

        RLError::SimpleProgramError =>
            write!(f,
                "SIMPLE-PROGRAM-ERROR \"invalid number of arguments: ~S\""),

        RLError::SimpleTypeError(simple_type_error) =>
            write!(f,
                "SIMPLE-TYPE-ERROR expected-type: {} datum: {}",
                simple_type_error.details,
                simple_type_error.source),

        RLError::UnboundVariableError(unbound_var_error) =>
            write!(f, "UNBOUND-VARIABLE {}", unbound_var_error),

        RLError::UndefinedFuncError(undef_func_error) =>
            write!(f, "UNDEFINED-FUNCTION {}", undef_func_error)
        }
    }
}

impl Error for RLError {
   fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
       match self {
           RLError::BlockError(block_error)            =>
               Some(block_error),

           RLError::DivisionByZero                     => None,

           RLError::ParseError(parse_error)            =>
               Some(parse_error),

           RLError::TypeError(type_error)              =>
               Some(type_error),

           RLError::ParseFloatError(custom_parse_float_error) =>
               Some(&custom_parse_float_error.error),

           RLError::ParseIntError(_parse_int_error)    => None,

           RLError::ReturnFromError(return_from_error) =>
               Some(return_from_error),

           RLError::SimpleError(simple_error)          =>
               Some(simple_error),

           RLError::SimpleProgramError                 => None,

           RLError::SimpleTypeError(simple_type_error) =>
               Some(simple_type_error),

           RLError::UnboundVariableError(unbound_var_error) =>
               Some(unbound_var_error),

           RLError::UndefinedFuncError(_undefined_func_error) => None,
       }
   }
}

impl From<BlockError> for RLError {
    fn from(err: BlockError) -> Self {
        RLError::BlockError(err)
    }
}

impl From<ParseError> for RLError {
    fn from(err: ParseError) -> Self {
        RLError::ParseError(err)
    }
}

impl From<TypeError> for RLError {
    fn from(err: TypeError) -> Self {
        RLError::TypeError(err)
    }
}

impl From<CustomParseFloatError> for RLError {
    fn from(err: CustomParseFloatError) -> Self {
        RLError::ParseFloatError(err)
    }
}

impl From<ParseIntError> for RLError {
    fn from(err: ParseIntError) -> Self {
        RLError::ParseIntError(err)
    }
}

impl From<UnboundVariableError> for RLError {
    fn from(err: UnboundVariableError) -> Self {
        RLError::UnboundVariableError(err)
    }
}

impl From<UndefinedFuncError> for RLError {
    fn from(err: UndefinedFuncError) -> Self {
        RLError::UndefinedFuncError(err)
    }
}

impl From<ReturnFromError> for RLError {
    fn from(err: ReturnFromError) -> Self {
        RLError::ReturnFromError(err)
    }
}

impl From<SimpleError> for RLError {
    fn from(err: SimpleError) -> Self {
        RLError::SimpleError(err)
    }
}

impl From<SimpleTypeError> for RLError {
    fn from(err: SimpleTypeError) -> Self {
        RLError::SimpleTypeError(err)
    }
}
