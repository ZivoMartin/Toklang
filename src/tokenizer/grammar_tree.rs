use std::collections::HashMap;
use super::include::{Node, TokenType, MACROS, SECTIONS, Flag};

pub fn build_grammar_tree() -> HashMap<TokenType, Node> {
    let mut group_map = HashMap::new();
    group_map.insert(
        TokenType::Line,
        Node::new(
            TokenType::Line,
            vec!(
                Node::leaf(TokenType::Section),
            ),
            vec!(
                Node::new(
                    TokenType::Ident,
                    vec!(
                        Node::leaf(TokenType::InKeyword)
                    ),
                    vec!(
                        Node::new_c(
                            TokenType::Symbol,
                            vec!(
                                Node::leaf(TokenType::Macro),
                                Node::leaf(TokenType::Expression),
                                Node::leaf(TokenType::String),
                            ),
                            vec!(),
                            vec!("=")
                        ),
                     
                    )
                ),
                Node::new_c(
                    TokenType::Keyword,
                    vec!(),
                    vec!(
                        Node::new_c(
                            TokenType::Symbol,
                            vec!(
                                Node::leaf(TokenType::SerieIdent)
                            ),
                            vec!(),
                            vec!("=")
                        )
                    ),
                    vec!("SYMB", "TPRIM", "GROUPS")
                )
            )
        )
    );
    
    group_map.insert(
        TokenType::InKeyword,
        Node::new(
            TokenType::InKeyword,
            vec!(),
            vec!(
                Node::new_c(
                    TokenType::Keyword,
                    vec!(),
                    vec!(
                        Node::new_c(
                            TokenType::Symbol,
                            vec!(
                                Node::new(
                                    TokenType::SerieString,
                                    vec!(),
                                    vec!(
                                        Node::leaf_c(TokenType::Symbol, vec!("]"))
                                    )
                                )
                            ),
                            vec!(),
                            vec!("[")
                        )
                    ),
                    vec!("in")
                )
            )
        )
    );
    
    group_map.insert(
        TokenType::Section,
        Node::new(
            TokenType::Section,
            vec!(),
            vec!(
                Node::new_c(
                    TokenType::Symbol,
                    vec!(),
                    vec!(
                        Node::leaf_c(TokenType::Ident, Vec::from(SECTIONS)).set_flag(Flag::Section).push()
                    ),
                    vec!("#")
                )
            )
        )
    );
    
    group_map.insert(
        TokenType::Macro,
        Node::new(
            TokenType::Macro,
            vec!(),
            vec!(
                Node::new_c(
                    TokenType::Symbol,
                    vec!(),
                    vec!(
                        Node::leaf_c(TokenType::Ident, Vec::from(MACROS))
                    ),
                    vec!("@")
                )
            )
        )
    );
        
     group_map.insert(
        TokenType::Expression,
        Node::new(
            TokenType::Expression,
            vec!(
                Node::new_end(
                    TokenType::Value,
                    vec!(),
                    vec!(
                        Node::new(
                            TokenType::Operator,  // Operateur
                            vec!(
                                Node::leaf(TokenType::Expression)
                            ),
                            vec!()
                        )
                    )
                )
            ),
            vec!(
                Node::new_c(
                    TokenType::Symbol,  //(
                    vec!(
                        Node::new(
                            TokenType::Expression,
                            vec!(),
                            vec!(
                                Node::new_end_c(
                                    TokenType::Symbol, // )
                                    vec!(),
                                    vec!(
                                        Node::new(
                                            TokenType::Operator,
                                            vec!(
                                                Node::leaf(TokenType::Expression)
                                            ),
                                            vec!()
                                        )
                                    ), 
                                    vec!(")") 
                                )
                            )
                        )
                    ),
                    vec!(),
                    vec!("(")
                )
            )
        )
    );

    
    group_map.insert(
        TokenType::Value,
        Node::new(
            TokenType::Value,
            vec!(),
            vec!(
                Node::new_end(
                    TokenType::Ident,
                    vec!(),
                    vec!(
                        Node::new_c(
                            TokenType::Symbol,
                            vec!(
                                Node::new(
                                    TokenType::SerieArg,
                                    vec!(),
                                    vec!(
                                        Node::leaf_c(TokenType::Symbol, vec!("}"))
                                    )
                                )
                            ),
                            vec!(
                                Node::leaf_c(TokenType::Symbol, vec!("}"))
                            ),
                            vec!("{")
                        )
                    )
                ),
            )
        )
    );

    group_map.insert(
        TokenType::Arg,
        Node::new(
            TokenType::Arg,
            vec!(
                Node::leaf(TokenType::String)
            ),
            vec!(
                Node::leaf_c(TokenType::Keyword, vec!("END"))
            )
        )
    );
        
    group_map.insert(
        TokenType::SerieArg,
        Node::new(
            TokenType::SerieArg,
            vec!(
                Node::new_end(
                    TokenType::Arg,
                    vec!(),
                    vec!(
                        Node::new_c(
                            TokenType::Symbol,
                            vec!(
                                Node::leaf(TokenType::SerieArg),
                            ),
                            vec!(),
                            vec!(",")
                        )
                    )
                )
            ),
            vec!()
        )
    );


    group_map.insert(
        TokenType::SerieIdent,
        Node::new(
            TokenType::SerieIdent,
            vec!(),
            vec!(
                Node::new_end(
                    TokenType::Ident,
                    vec!(),
                    vec!(
                        Node::new_c(
                            TokenType::Symbol,
                            vec!(
                                Node::leaf(TokenType::SerieIdent)
                            ),
                            vec!(),
                            vec!(",")
                        )
                    )
                )
            )
        )
    );
    
    
    group_map.insert(
        TokenType::SerieIdent,
        Node::new(
            TokenType::SerieIdent,
            vec!(),
            vec!(
                Node::new_end(
                    TokenType::Ident,
                    vec!(),
                    vec!(
                        Node::new_c(
                            TokenType::Symbol,
                            vec!(
                                Node::leaf(TokenType::SerieIdent)
                            ),
                            vec!(),
                            vec!(",")
                        )
                    )
                )
            )
        )
    );

    group_map.insert(
        TokenType::SerieString,
        Node::new(
            TokenType::SerieString,
            vec!(
                Node::new_end(
                    TokenType::String,
                    vec!(),
                    vec!(
                        Node::new_c(
                            TokenType::Symbol,
                            vec!(
                                Node::leaf(TokenType::SerieString)
                            ),
                            vec!(),
                            vec!(",")
                        )
                    )
                )
            ),
            vec!(
                
            )
        )
    );

    
    group_map.insert(
        TokenType::SerieIdent,
        Node::new(
            TokenType::SerieIdent,
            vec!(),
            vec!(
                Node::new_end(
                    TokenType::Ident,
                    vec!(),
                    vec!(
                        Node::new_c(
                            TokenType::Symbol,
                            vec!(
                                Node::leaf(TokenType::SerieIdent)
                            ),
                            vec!(),
                            vec!(",")
                        )
                    )
                )
            )
        )
    );

    
    group_map.insert(
        TokenType::String,
        Node::new(
            TokenType::String,
            vec!(),
            vec!(
                Node::new_c_r(
                    TokenType::Symbol,
                    vec!(
                        Node::leaf(TokenType::SerieChar)
                    ),
                    vec!(),
                    vec!("\""),
                    0
                ).consider_garbage()
            )
        )
    );

    
    group_map.insert(
        TokenType::SerieChar,
        Node::new(
            TokenType::SerieChar,
            vec!(
                Node::new(
                    TokenType::ComplexChar,
                    vec!(),
                    vec!(
                        Node::leaf_c(TokenType::Symbol, vec!("\""))
                    )
                ).consider_garbage()
            ),
            vec!()
        )
    );


    group_map.insert(
        TokenType::ComplexChar,
        Node::new(
            TokenType::ComplexChar,
            vec!(),
            vec!(
                Node::leaf_c(TokenType::Symbol, vec!("\\", "\"", "\'")).priv_const().push(), // N'importe quoi sauf la contrainte
                Node::new_c(
                    TokenType::Symbol,
                    vec!(),
                    vec!(
                        Node::leaf(TokenType::Symbol).push()
                    ),
                    vec!("\\")
                ).push()
            )
        )
    );
    
    group_map
}


