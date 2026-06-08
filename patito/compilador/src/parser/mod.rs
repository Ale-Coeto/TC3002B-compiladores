pub mod compile_error;

use crate::scanner::{Scanner, Token};
use crate::grammar::ProgramaParser;
use crate::semantics::Semantics;
use crate::semantics::dir_func::FuncJson;
use crate::memory::{MemoryManager, ConstantJson};
use crate::quads::QuadGenerator;
use crate::parser::compile_error::CompileError;
use crate::parser::compile_error::ErrorType;
use lalrpop_util::ParseError;

fn format_parse_error(e: &ParseError<usize, Token, ()>) -> String {
    fn clean_expected(expected: &[String]) -> String {
        expected.iter()
            .map(|s| s.trim_matches('"').to_string())
            .collect::<Vec<_>>()
            .join(", ")
    }

    match e {
        ParseError::UnrecognizedToken { token: (_, tok, _), expected } =>
            format!("Parse error: expected ({}), received ({:?})", clean_expected(expected), tok),
        ParseError::UnrecognizedEof { expected, .. } =>
            format!("Parse error: unexpected end of file, expected ({})", clean_expected(expected)),
        ParseError::InvalidToken { .. } =>
            "Parse error: invalid token".to_string(),
        ParseError::ExtraToken { token: (_, tok, _) } =>
            format!("Parse error: unexpected extra token ({:?})", tok),
        _ => format!("{:?}", e),
    }
}

pub struct ParseOutput {
    pub quads: Vec<String>,
    pub dir_func: Vec<FuncJson>,
    pub constants: Vec<ConstantJson>,
}

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Parser
    }

    pub fn parse(&self, input: &str) -> Result<ParseOutput, Vec<CompileError>> {
        let mut scanner: Scanner = Scanner::new(input);
        let mut semantics: Semantics = Semantics::new();
        let mut quad_generator: QuadGenerator = QuadGenerator::new();
        let mut errors = Vec::new();
        let result = ProgramaParser::new().parse(&mut semantics, &mut quad_generator, &mut scanner);

        let lex_errors = scanner.get_errors();
        for error in lex_errors {
            errors.push(CompileError { 
                error_type: ErrorType::Lexic,
                message: format!("Unrecognized token {} - {}", error.start, error.end).to_string() 
            })
        }

        let semantic_errors = semantics.get_errors();
        for error in semantic_errors {
            errors.push(CompileError {
                error_type: ErrorType::Semantic,
                message: format!("{}", error.message)
            })
        }

        let quads = quad_generator.get_results();

        let dir_func = semantics.dir_func.get_dir_func();
        let constants = MemoryManager::with_instance(|mm| mm.get_constants());

        let _ = semantics.dir_func.delete_all();
        MemoryManager::with_instance(|mm| mm.reset());

        if let Err(e) = result {
            errors.push(CompileError {
                error_type: ErrorType::Sintactic,
                message: format_parse_error(&e)
            })
        }
        
        if errors.is_empty() {
            Ok(ParseOutput { quads, dir_func, constants })
        } else {
            Err(errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn programa(vars: &str, funcs: &str, body: &str) -> String {
        format!(
            "programa main ; {} {} inicio {} fin",
            vars,
            funcs,
            body
        )
    }

    fn valid_programa(body: &str) -> String {
        programa(&valid_vars(), &valid_funcs(), &valid_cuerpo(Some(body)))
    }

    fn valid_programa_with_funcs(funcs: &str) -> String {
        programa(&valid_vars(), funcs, &valid_cuerpo(None))
    }

    fn valid_programa_with_vars(vars: &str) -> String {
        programa(vars, &valid_funcs(), &valid_cuerpo(None))
    }

    fn valid_vars() -> String {
        "vars var1 , x1 , x2 , x3 : entero ;".to_string()
    }

    fn valid_funcs() -> String {
        format!("nula f1 ( ) {{ {} }} ;", valid_cuerpo(None)).to_string()
    }

    fn valid_cuerpo(body: Option<&str>) -> String {
        format!("{{ {} }}", body.unwrap_or(""))
    }

    mod programa {
        use super::*;

        #[test]
        fn test_01_01_programa() {
            let vars = valid_vars();
            let funcs = valid_funcs();
            let body = valid_cuerpo(None);
            let input = programa(&vars, &funcs, &body);
            
            let parser: Parser = Parser::new();
            assert!(parser.parse(&input).is_ok());
        }
    }

    mod estatuto {
        use super::*;
        #[test]
        fn test_02_01_estatuto_asigna() {
            let asigna = "x1 = 5 ;";
            let input = valid_programa(&asigna);

            let parser: Parser = Parser::new();
            assert!(parser.parse(&input).is_ok());
        }

        #[test]
        fn test_02_02_estatuto_condicion_si() {
            let cond = format!("si ( 10 ) {} ;", valid_cuerpo(None));
            let input = valid_programa(&cond);

            let parser: Parser = Parser::new();
            assert!(parser.parse(&input).is_ok());
        }

        #[test]
        fn test_02_03_estatuto_condicion_sino() {
            let cond = format!("si ( 10 ) {} sino {};", valid_cuerpo(None), valid_cuerpo(None));
            let input = valid_programa(&cond);

            let parser: Parser = Parser::new();
            assert!(parser.parse(&input).is_ok());
        }

        #[test]
        fn test_02_04_estatuto_ciclo() {
            let ciclo = format!("mientras ( 1.0 ) haz {};", valid_cuerpo(None));
            let input = valid_programa(&ciclo);

            let parser: Parser = Parser::new();
            assert!(parser.parse(&input).is_ok());
        }

        #[test]
        fn test_02_05_estatuto_llamada() {
            let llamada = format!("f1() ;");
            let input = valid_programa(&llamada);

            let parser: Parser = Parser::new();
            assert!(parser.parse(&input).is_ok());
        }

        #[test]
        fn test_02_06_estatuto_imprime() {
            let imprime = format!("escribe ( \"Hola mundo\" );");
            let input = valid_programa(&imprime);

            let parser: Parser = Parser::new();
            assert!(parser.parse(&input).is_ok());
        }

        #[test]
        fn test_02_07_estatuto_lista() {
            let imprime = format!("[ escribe ( \"Hola mundo\" ); f1 (); ]");
            let input = valid_programa(&imprime);

            let parser: Parser = Parser::new();
            assert!(parser.parse(&input).is_ok());
        }
    }

    mod funcs {
        use super::*;

        #[test]
        fn test_03_01_tipo_funcion() {
            let cases = [
                "entero funcion_1 ( ) { { } } ;",
                "flotante funcion_2 ( ) { { } } ;",
                "nula funcion_3 ( ) { { } } ;",
            ];

            let parser: Parser = Parser::new();
            for funcs in cases.iter() {
                let input = valid_programa_with_funcs(funcs);
                assert!(parser.parse(&input).is_ok());
            }
        }

        #[test]
        fn test_03_02_args() {
            let cases = [
                "nula fn1 ( ) { { } } ;",
                "nula fn2 ( num : entero ) { { } } ;",
                "nula fn3 ( num : entero , puntuacion : flotante ) { { } } ;",
            ];

            let parser: Parser = Parser::new();
            for funcs in cases.iter() {
                let input = valid_programa_with_funcs(funcs);
                assert!(parser.parse(&input).is_ok());
            }
        }

        #[test]
        fn test_03_03_cuerpo_variantes() {
            let cases = [
                "nula f_sin_vars ( ) { { } } ;",
                "nula f_con_vars ( ) { vars x : entero ; { } } ;",
                "nula f_cuerpo_normal ( ) { vars x : entero ; { escribe ( \"hola\" ) ; } } ;",
            ];

            let parser: Parser = Parser::new();
            for funcs in cases.iter() {
                let input = valid_programa_with_funcs(funcs);
                assert!(parser.parse(&input).is_ok());
            }
        }
    }

    mod vars {
        use super::*;

        #[test]
        fn test_04_01_una_var() {
            let vars = "vars x : entero ;";
            let input = valid_programa_with_vars(vars);

            let parser: Parser = Parser::new();
            assert!(parser.parse(&input).is_ok());
        }

        #[test]
        fn test_04_02_multiples_ids() {
            let vars = "vars x , y , z : entero ;";
            let input = valid_programa_with_vars(vars);

            let parser: Parser = Parser::new();
            assert!(parser.parse(&input).is_ok());
        }

        #[test]
        fn test_04_03_multiples_definiciones_y_tipos() {
            let cases = [
                "vars x : entero ; y : flotante ;",
                "vars a , b : entero ; c : flotante ;",
                "vars n1 , n2 : entero ; p1 , p2 , p3 : flotante ;",
            ];

            let parser: Parser = Parser::new();
            for vars in cases.iter() {
                let input = valid_programa_with_vars(vars);
                assert!(parser.parse(&input).is_ok());
            }
        }
    }

    mod imprime {
        use super::*;

        #[test]
        fn test_05_01_imprime_letreros() {
            let imprime = "escribe ( \"hola\" , \"mundo\" , \"!\" ) ;";
            let input = valid_programa(imprime);

            let parser: Parser = Parser::new();
            assert!(parser.parse(&input).is_ok());
        }

        #[test]
        fn test_05_02_imprime_expresion() {
            let imprime = "escribe ( + 1 ) ;";
            let input = valid_programa(imprime);

            let parser: Parser = Parser::new();
            assert!(parser.parse(&input).is_ok());
        }

        #[test]
        fn test_05_03_imprime_lista_mixta() {
            let imprime = "escribe ( \"hola\" , + 1 , \"mundo\" , + 2 ) ;";
            let input = valid_programa(imprime);

            let parser: Parser = Parser::new();
            assert!(parser.parse(&input).is_ok());
        }
    }

    mod expresion {
        use super::*;

        #[test]
        fn test_06_01_expresion_exp() {
            let cases = [
                "x1 = 8 ;",
                "x1 = +8 * 8 + 9;",
                "x1 = 20 - 4 / 2;",
                "x1 = 8 + 2 * -3;",
            ];

            let parser: Parser = Parser::new();
            for expr in cases.iter() {
                let input = valid_programa(expr);
                assert!(parser.parse(&input).is_ok());
            }
        }

        #[test]
        fn test_06_02_expresion_comparadores() {
            let cases = [
                "x1 = 10 > 2 ;",
                "x1 = 1 < 9 ;",
                "x1 = +7 != 3 ;",
                "x1 = 4 == 4 ;",
            ];

            let parser: Parser = Parser::new();
            for expr in cases.iter() {
                let input = valid_programa(expr);
                assert!(parser.parse(&input).is_ok());
            }
        }
    }

    mod factor {
        use super::*;

        #[test]
        fn test_07_01_factor_parentesis_expresion() {
            let cases = [
                "x1 = ( -8 + 2 ) ;",
                "x1 = ( + 3 * - 4 ) ;",
            ];

            let parser: Parser = Parser::new();
            for expr in cases.iter() {
                let input = valid_programa(expr);
                assert!(parser.parse(&input).is_ok());
            }
        }

        #[test]
        fn test_07_02_factor_valor_con_y_sin_signo() {
            let con_signo = [
                "x1 = + 8 ;",
                "x1 = - 8 ;",
                "x1 = + 1.5 ;",
                "x1 = - 1.5 ;",
            ];

            let sin_signo = [
                "x1 = 8 ;",
                "x1 = 1.5 ;",
            ];

            let parser: Parser = Parser::new();
            for expr in con_signo.iter() {
                let input = valid_programa(expr);
                assert!(parser.parse(&input).is_ok());
            }

            for expr in sin_signo.iter() {
                let input = valid_programa(expr);
                assert!(parser.parse(&input).is_ok());
            }
        }

        #[test]
        fn test_07_03_factor_identificador_con_y_sin_signo() {
            let cases = [
                "x1 = + var1 ;",
                "x2 = - var1 ;",
                "x3 = var1 ;",
            ];

            let parser: Parser = Parser::new();
            for expr in cases.iter() {
                let input = valid_programa(expr);
                assert!(parser.parse(&input).is_ok());
            }
        }

        // #[test]
        // fn test_07_04_factor_llamada() {
        //     let cases = [
        //         "x1 = f1( ) ;",
        //         "x1 = f1 (1);",
        //         "x1 = f1 (+ 1 , -2 ) ;",
        //     ];

        //     let parser: Parser = Parser::new();
        //     for expr in cases.iter() {
        //         let input = valid_programa(expr);
        //                                     println!("INPUT{}", input);
        //     println!("{:?}", parser.parse(&input));
        //         assert!(parser.parse(&input).is_ok());
        //     }
        // }
    }
}