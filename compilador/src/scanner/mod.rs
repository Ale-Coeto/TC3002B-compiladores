pub mod tokens;
pub mod lex_error;

pub use tokens::Token;
pub use lex_error::LexError;
use logos::Logos;

pub struct Scanner<'a> {
    lexer: logos::Lexer<'a, Token>,
    errors: Vec<LexError>,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Token::lexer(input),
            errors: Vec::<LexError>::new(),
        }
    }

    pub fn get_errors(&self) -> &Vec<LexError> {
        &self.errors
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = (usize, Token, usize);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let tok = self.lexer.next()?;
            let span = self.lexer.span();

            return match tok {
                Ok(t) => Some((span.start, t, span.end)),
                Err(_) => {
                    self.errors.push(LexError {
                        start: span.start,
                        end: span.end,
                    });
                    continue;
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_token_success(word: &str, token: Token) {
        let input = format!("{} {}", word, word);
        let mut scanner: Scanner = Scanner::new(&input);
        let mut offset = 0;

        while let Some((start, t, end)) = scanner.next() {
            assert_eq!(t, token);
            assert_eq!(start, offset);
            assert_eq!(end, offset + word.len());
            offset += word.len() + 1;
        }

        assert!(scanner.next().is_none());
        assert!(scanner.get_errors().is_empty());
    }

    fn test_token_not_present(input: &str, token: Token) {
        let mut scanner: Scanner = Scanner::new(input);

        while let Some((_, t, _)) = scanner.next() {
            assert_ne!(t, token);
        }

        assert!(scanner.next().is_none());
        assert!(scanner.get_errors().is_empty());
    }

    mod reserved_words {
        use super::*;

        #[test]
        fn test_01_01_programa_success() {
            test_token_success("programa", Token::Programa);
        }

        #[test]
        fn test_01_02_programa_invalid() {
            test_token_not_present("program programas", Token::Programa);
        }

        #[test]
        fn test_02_01_inicio_success() {
            test_token_success("inicio", Token::Inicio);
        }

        #[test]
        fn test_02_02_inicio_invalid() {
            test_token_not_present("inicia inicios", Token::Inicio);
        }

        #[test]
        fn test_03_01_fin_success() {
            test_token_success("fin", Token::Fin);
        }

        #[test]
        fn test_03_02_fin_invalid() {
            test_token_not_present("fi final", Token::Fin);
        }

        #[test]
        fn test_04_01_entero_success() {
            test_token_success("entero", Token::Entero);
        }

        #[test]
        fn test_04_02_entero_invalid() {
            test_token_not_present("enter enteras", Token::Entero);
        }

        #[test]
        fn test_05_01_flotante_success() {
            test_token_success("flotante", Token::Flotante);
        }

        #[test]
        fn test_05_02_flotante_invalid() {
            test_token_not_present("flotant flotantes", Token::Flotante);
        }

        #[test]
        fn test_06_01_vars_success() {
            test_token_success("vars", Token::Variable);
        }

        #[test]
        fn test_06_02_vars_invalid() {
            test_token_not_present("var varss", Token::Variable);
        }

        #[test]
        fn test_07_01_escribe_success() {
            test_token_success("escribe", Token::Escribe);
        }

        #[test]
        fn test_07_02_escribe_invalid() {
            test_token_not_present("escrib escribes", Token::Escribe);
        }

        #[test]
        fn test_08_01_nula_success() {
            test_token_success("nula", Token::Nula);
        }

        #[test]
        fn test_08_02_nula_invalid() {
            test_token_not_present("nulo nulos nulas", Token::Nula);
        }

        #[test]
        fn test_09_01_mientras_success() {
            test_token_success("mientras", Token::Mientras);
        }

        #[test]
        fn test_09_02_mientras_invalid() {
            test_token_not_present("mientas mientrasmientras", Token::Mientras);
        }

        #[test]
        fn test_10_01_haz_success() {
            test_token_success("haz", Token::Haz);
        }

        #[test]
        fn test_10_02_haz_invalid() {
            test_token_not_present("ha hazas", Token::Haz);
        }

        #[test]
        fn test_11_01_si_success() {
            test_token_success("si", Token::Si);
        }

        #[test]
        fn test_11_02_si_invalid() {
            test_token_not_present("s sios", Token::Si);
        }

        #[test]
        fn test_12_01_sino_success() {
            test_token_success("sino", Token::Sino);
        }

        #[test]
        fn test_12_02_sino_invalid() {
            test_token_not_present("sin sinos", Token::Sino);
        }
    }

    mod symbols_and_operators {
        use super::*;

        #[test]
        fn test_01_01_puntoycoma_success() {
            test_token_success(";", Token::PuntoYComa);
        }

        #[test]
        fn test_01_02_puntoycoma_invalid() {
            test_token_not_present("punto :", Token::PuntoYComa);
        }

        #[test]
        fn test_02_01_dospuntos_success() {
            test_token_success(":", Token::DosPuntos);
        }

        #[test]
        fn test_02_02_dospuntos_invalid() {
            test_token_not_present("; ,,", Token::DosPuntos);
        }

        #[test]
        fn test_03_01_coma_success() {
            test_token_success(",", Token::Coma);
        }

        #[test]
        fn test_03_02_coma_invalid() {
            test_token_not_present("coma ;", Token::Coma);
        }

        #[test]
        fn test_04_01_llaveabrir_success() {
            test_token_success("{", Token::LlaveAbrir);
        }

        #[test]
        fn test_04_02_llaveabrir_invalid() {
            test_token_not_present("llave } ; [", Token::LlaveAbrir);
        }

        #[test]
        fn test_05_01_llavecerrar_success() {
            test_token_success("}", Token::LlaveCerrar);
        }

        #[test]
        fn test_05_02_llavecerrar_invalid() {
            test_token_not_present("] ) {", Token::LlaveCerrar);
        }

        #[test]
        fn test_06_01_corcheteabrir_success() {
            test_token_success("[", Token::CorcheteAbrir);
        }

        #[test]
        fn test_06_02_corcheteabrir_invalid() {
            test_token_not_present("corchete { ( ]", Token::CorcheteAbrir);
        }

        #[test]
        fn test_07_01_corchetecerrar_success() {
            test_token_success("]", Token::CorcheteCerrar);
        }

        #[test]
        fn test_07_02_corchetecerrar_invalid() {
            test_token_not_present("} ) [", Token::CorcheteCerrar);
        }

        #[test]
        fn test_08_01_parentesisabrir_success() {
            test_token_success("(", Token::ParentesisAbrir);
        }

        #[test]
        fn test_08_02_parentesisabrir_invalid() {
            test_token_not_present("{ [ )", Token::ParentesisAbrir);
        }

        #[test]
        fn test_09_01_parentesiscerrar_success() {
            test_token_success(")", Token::ParentesisCerrar);
        }

        #[test]
        fn test_09_02_parentesiscerrar_invalid() {
            test_token_not_present("} ] (", Token::ParentesisCerrar);
        }

        #[test]
        fn test_10_01_comparadorigual_success() {
            test_token_success("==", Token::ComparadorIgual);
        }

        #[test]
        fn test_10_02_comparadorigual_invalid() {
            test_token_not_present("= =", Token::ComparadorIgual);
        }

        #[test]
        fn test_11_01_noigual_success() {
            test_token_success("!=", Token::NoIgual);
        }

        #[test]
        fn test_11_02_noigual_invalid() {
            test_token_not_present("= ==", Token::NoIgual);
        }

        #[test]
        fn test_12_01_mayor_success() {
            test_token_success(">", Token::Mayor);
        }

        #[test]
        fn test_12_02_mayor_invalid() {
            test_token_not_present("<", Token::Mayor);
        }

        #[test]
        fn test_13_01_menor_success() {
            test_token_success("<", Token::Menor);
        }

        #[test]
        fn test_13_02_menor_invalid() {
            test_token_not_present(">", Token::Menor);
        }

        #[test]
        fn test_14_01_igual_success() {
            test_token_success("=", Token::Igual);
        }

        #[test]
        fn test_14_02_igual_invalid() {
            test_token_not_present("== !=", Token::Igual);
        }

        #[test]
        fn test_15_01_mas_success() {
            test_token_success("+", Token::Mas);
        }

        #[test]
        fn test_15_02_mas_invalid() {
            test_token_not_present("plus -", Token::Mas);
        }

        #[test]
        fn test_16_01_menos_success() {
            test_token_success("-", Token::Menos);
        }

        #[test]
        fn test_16_02_menos_invalid() {
            test_token_not_present("minus =", Token::Menos);
        }

        #[test]
        fn test_17_01_multiplicar_success() {
            test_token_success("*", Token::Multiplicar);
        }

        #[test]
        fn test_17_02_multiplicar_invalid() {
            test_token_not_present("star x", Token::Multiplicar);
        }

        #[test]
        fn test_18_01_dividir_success() {
            test_token_success("/", Token::Dividir);
        }

        #[test]
        fn test_18_02_dividir_invalid() {
            test_token_not_present("}]", Token::Dividir);
        }
    }

    mod regex {
        use super::*;

        #[test]
        fn test_01_01_cte_entero_success() {
            let cases = [1, 9, 12220909, -12, -987654321, 0, 01, 20];
            let input = cases.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" ");
            let mut scanner: Scanner = Scanner::new(&input);
            for v in cases.iter() {
                let (_, t, _) = scanner.next().expect("Expected a token");
                assert_eq!(t, Token::ConstanteEntero(*v));
            }

            assert!(scanner.next().is_none());
            assert!(scanner.get_errors().is_empty());
        }

        #[test]
        fn test_02_01_cte_flotante_success() {
            let cases = ["1.0", "0.9", "0.10", "-0.1", "-1.0"];
            let input = cases.join(" ");
            let mut scanner: Scanner = Scanner::new(&input);
            for s in cases.iter() {
                let (_, t, _) = scanner.next().expect("Expected a token");
                let expected = s.parse::<f64>().expect("Expected parseable f64");
                assert_eq!(t, Token::ConstanteFlotante(expected));
            }

            assert!(scanner.next().is_none());
            assert!(scanner.get_errors().is_empty());
        }

        #[test]
        fn test_03_01_cte_flotante_cientifico_success() {
            let cases = ["1.0e1","1.0E1","-0.1e+1","2.0e-3","3.5E-2","0.0e0","5.25e+3"];

            let input = cases.join(" ");
            let mut scanner: Scanner = Scanner::new(&input);

            for s in cases.iter() {
                let (_, token, _) = scanner.next().expect("Expected a float token");
                let expected = s.parse::<f64>().expect("Expected parseable f64");
                assert_eq!(token, Token::ConstanteFlotante(expected));
            }

            assert!(scanner.next().is_none());
            assert!(scanner.get_errors().is_empty());
        }

        #[test]
        fn test_04_01_identificador_success() {
            let cases = ["hola", "_start", "a1_b2"];
            let input = cases.join(" ");
            let mut scanner: Scanner = Scanner::new(&input);

            for s in cases.iter() {
                let (_, token, _) = scanner.next().expect("Expected identifier token");
                assert_eq!(token, Token::Identificador(s.to_string()));
            }

            assert!(scanner.next().is_none());
            assert!(scanner.get_errors().is_empty());
        }

        #[test]
        fn test_04_02_identificador_invalid() {
            test_token_not_present("1abc", Token::Identificador("1abc".to_string()));
            test_token_not_present("9_start", Token::Identificador("9_start".to_string()));
            test_token_not_present("-foo", Token::Identificador("-foo".to_string()));
        }

        #[test]
        fn test_05_01_letrero_success() {
            let cases = ["\"hola\"", "\"he\\\"llo\"", "\"\\\\\""]; 
            let input = cases.join(" ");
            let mut scanner: Scanner = Scanner::new(&input);

            for s in cases.iter() {
                let (_, token, _) = scanner.next().expect("Expected string token");
                assert_eq!(token, Token::Letrero(s.to_string()));
            }

            assert!(scanner.next().is_none());
            assert!(scanner.get_errors().is_empty());
        }
    }

    mod errors {
        use super::*;

        #[test]
        fn test_01_01_puntuacion_error() {
            let mut scanner: Scanner = Scanner::new("& @ $ # . | \\");
            while scanner.next().is_some() {}

            assert_eq!(scanner.get_errors().len(), 7);
        }

        #[test]
        fn test_01_02_strings_error() {
            let mut scanner = Scanner::new(".holas 'no \"eee");
            while scanner.next().is_some() {}

            assert_eq!(scanner.get_errors().len(), 3);
        }

    }

}