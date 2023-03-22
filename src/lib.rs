use std::process;

pub mod lexer;
pub mod parser;

#[derive(PartialEq, Clone, Debug)]
pub enum TokenTypes {
    KEYWORD,      // fun, match, include
    IDENTIFIER,   // x, y, z, i, j ,k
    TYPE,         // types
    SYMBOL,       // used for ( ) { } right now
    FUNCTIONCALL, // used to figure out if a function is being called
    EOL,          // End of Line
    EOF,          // End of File
}

#[derive(Debug, Clone, Copy)]
pub enum Types {
    U4 = 1,
    U4List = 33,
    U8 = 2,
    U8List = 34,
    U16 = 3,
    U16List = 35,
    U32 = 4,
    U32List = 36,
    U64 = 5,
    U64List = 37,
    U128 = 6,
    U128List = 38,
    I4 = 7,
    I4List = 39,
    I8 = 8,
    I8List = 40,
    I16 = 9,
    I16List = 41,
    I32 = 10,
    I32List = 42,
    I64 = 11,
    I64List = 43,
    I128 = 12,
    I128List = 44,
    F4 = 13,
    F4List = 45,
    F8 = 14,
    F8List = 46,
    F16 = 15,
    F16List = 47,
    F32 = 16,
    F32List = 48,
    F64 = 17,
    F64List = 49,
    F128 = 18,
    F128List = 50,
    Str = 19,
    StrList = 51,
    Bool = 21,
    BoolList = 52,
    None = 22,
}

pub fn exit(msg: &str, code: Option<i32>) -> ! {
    println!("{}", msg);
    process::exit(code.unwrap_or(1));
}

pub fn type_from_str(string: &str) -> Types {
    if string == "u4" {
        return Types::U4;
    }
    if string == "u8" {
        return Types::U8;
    }
    if string == "u16" {
        return Types::U16;
    }
    if string == "u32" {
        return Types::U32;
    }
    if string == "u64" {
        return Types::U64;
    }
    if string == "u128" {
        return Types::U128;
    }
    if string == "i4" {
        return Types::I4;
    }
    if string == "i8" {
        return Types::I8;
    }
    if string == "i16" {
        return Types::I16;
    }
    if string == "i32" {
        return Types::I32;
    }
    if string == "i64" {
        return Types::I64;
    }
    if string == "i128" {
        return Types::I128;
    }
    if string == "f4" {
        return Types::F4;
    }
    if string == "f8" {
        return Types::F8;
    }
    if string == "f16" {
        return Types::F16;
    }
    if string == "f32" {
        return Types::F32;
    }
    if string == "f64" {
        return Types::F64;
    }
    if string == "f128" {
        return Types::F128;
    }
    if string == "str" {
        return Types::Str;
    }
    if string == "bool" {
        return Types::Bool;
    }

    if string == "u4[]" {
        return Types::U4List;
    };
    if string == "u8[]" {
        return Types::U8List;
    };
    if string == "u16[]" {
        return Types::U16List;
    };
    if string == "u32[]" {
        return Types::U32List;
    };
    if string == "u64[]" {
        return Types::U64List;
    };
    if string == "u128[]" {
        return Types::U128List;
    };
    if string == "i4[]" {
        return Types::I4List;
    };
    if string == "i8[]" {
        return Types::I8List;
    };
    if string == "i16[]" {
        return Types::I16List;
    };
    if string == "i32[]" {
        return Types::I32List;
    };
    if string == "i64[]" {
        return Types::I64List;
    };
    if string == "i128[]" {
        return Types::I128List;
    };
    if string == "f4[]" {
        return Types::F4List;
    };
    if string == "f8[]" {
        return Types::F8List;
    };
    if string == "f16[]" {
        return Types::F16List;
    };
    if string == "f32[]" {
        return Types::F32List;
    };
    if string == "f64[]" {
        return Types::F64List;
    };
    if string == "f128[]" {
        return Types::F128List;
    };
    if string == "str[]" {
        return Types::StrList;
    };
    if string == "bool[]" {
        return Types::BoolList;
    };

    return Types::None;
}

// #[derive(Clone, Debug)]
// pub struct Token {
//   t_type: TokenTypes,
//   token_string: String,
//   line: i32,
//   column: i32
// }

// impl PartialEq for Token {
//   // basically, don't compare line and column
//   fn eq(&self, other: &Self) -> bool {
//     (self.t_type == other.t_type) && (self.token_string == other.token_string)
//   }
// }
