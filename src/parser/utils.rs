/// Utilities shared across all parser sub-modules:
/// block parsing, type parsing, param/arg list parsing.
use super::*;

impl Parser {
    // ── Block ─────────────────────────────────────────────────────────────────

    pub(super) fn parse_block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let sp = self.sp().clone();
        self.expect(&Token::Indent)?;

        let mut stmts = Vec::new();
        while !matches!(self.peek(), Token::Dedent | Token::Eof) {
            stmts.push(self.parse_stmt()?);
        }

        self.expect(&Token::Dedent)?;

        if stmts.is_empty() {
            return Err(ParseError::at(
                "thou hast left a sacred block empty — the spirit cannot dwell in the void",
                sp.line,
                sp.col,
            ));
        }
        Ok(stmts)
    }

    // ── Types ─────────────────────────────────────────────────────────────────

    pub(super) fn parse_type(&mut self) -> Result<HolyType, ParseError> {
        let sp = self.sp().clone();
        match self.advance().token {
            Token::Atom       => Ok(HolyType::Atom),
            Token::Fractional => Ok(HolyType::Fractional),
            Token::Word       => Ok(HolyType::Word),
            Token::Dogma      => Ok(HolyType::Dogma),
            Token::Void       => Ok(HolyType::Void),
            Token::Ident(n)   => self.parse_custom_type(n),
            t => Err(ParseError::at(
                format!(
                    "'{}' is a profane type — only these are consecrated: atom, fractional, word, dogma, void, grace, verdict, or a declared type name",
                    token_name(&t)
                ),
                sp.line,
                sp.col,
            )),
        }
    }

    fn parse_custom_type(&mut self, name: String) -> Result<HolyType, ParseError> {
        if self.peek() == &Token::Of && self.is_type_start_ahead(1) {
            self.advance();
            let first = self.parse_type()?;
            let mut type_args = vec![first];

            if self.peek() == &Token::Thus {
                self.advance();
            } else {
                loop {
                    match self.peek() {
                        Token::Comma if self.is_type_start_ahead(1) => {
                            self.advance();
                            type_args.push(self.parse_type()?);
                            if self.peek() == &Token::Thus {
                                self.advance();
                                break;
                            }
                        }
                        Token::And if self.is_type_start_ahead(1) => {
                            self.advance();
                            type_args.push(self.parse_type()?);
                            break;
                        }
                        _ => break,
                    }
                }
            }
            Ok(HolyType::Generic(name, type_args))
        } else {
            Ok(HolyType::Custom(name))
        }
    }

    /// Peeks ahead by `offset` positions and checks if that token can start a type.
    pub(super) fn is_type_start_ahead(&self, offset: usize) -> bool {
        let tok = self.tokens.get(self.pos + offset).map(|s| &s.token);
        matches!(
            tok,
            Some(Token::Atom)
                | Some(Token::Fractional)
                | Some(Token::Word)
                | Some(Token::Dogma)
                | Some(Token::Void)
                | Some(Token::Ident(_))
        )
    }

    // ── Type argument lists ───────────────────────────────────────────────────

    /// Parses the `of T` / `of T, U` type parameters in a declaration head.
    pub(super) fn parse_type_params(&mut self) -> Result<Vec<String>, ParseError> {
        if self.peek() != &Token::Of || !self.is_type_start_ahead(1) {
            return Ok(Vec::new());
        }
        self.advance();
        self.parse_ident_list()
    }

    /// Parses `of T` call-site type arguments (e.g. `hail foo of atom`).
    pub(super) fn parse_call_type_args(&mut self) -> Result<Vec<HolyType>, ParseError> {
        if self.peek() != &Token::Of || !self.is_type_start_ahead(1) {
            return Ok(Vec::new());
        }
        self.advance();
        self.parse_type_list()
    }

    /// Parses optional `of T, U` type args on a variant (e.g. `granted of verdict`).
    pub(super) fn parse_variant_type_args(&mut self) -> Result<Vec<HolyType>, ParseError> {
        if self.peek() == &Token::Of && self.is_type_start_ahead(1) {
            self.advance();
            self.parse_type_list()
        } else {
            Ok(Vec::new())
        }
    }

    fn parse_type_list(&mut self) -> Result<Vec<HolyType>, ParseError> {
        let mut items = vec![self.parse_type()?];
        loop {
            match self.peek() {
                Token::Comma if self.is_type_start_ahead(1) => {
                    self.advance();
                    items.push(self.parse_type()?);
                }
                Token::And if self.is_type_start_ahead(1) => {
                    self.advance();
                    items.push(self.parse_type()?);
                    break;
                }
                _ => break,
            }
        }
        Ok(items)
    }

    // ── Parameter lists ───────────────────────────────────────────────────────

    pub(super) fn parse_optional_params(&mut self) -> Result<Vec<(String, HolyType)>, ParseError> {
        if self.peek() == &Token::Receiving {
            self.advance();
            self.parse_param_list()
        } else {
            Ok(Vec::new())
        }
    }

    fn parse_param_list(&mut self) -> Result<Vec<(String, HolyType)>, ParseError> {
        let mut params = vec![self.parse_param()?];
        loop {
            match self.peek() {
                Token::Comma => {
                    self.advance();
                    params.push(self.parse_param()?);
                }
                Token::And => {
                    self.advance();
                    params.push(self.parse_param()?);
                    break;
                }
                _ => break,
            }
        }
        Ok(params)
    }

    fn parse_param(&mut self) -> Result<(String, HolyType), ParseError> {
        let name = self.expect_ident()?;
        self.expect(&Token::Of)?;
        let ty = self.parse_type()?;
        Ok((name, ty))
    }

    // ── Identifier & argument lists ───────────────────────────────────────────

    pub(super) fn parse_ident_list(&mut self) -> Result<Vec<String>, ParseError> {
        let mut items = vec![self.expect_ident()?];
        loop {
            match self.peek() {
                Token::Comma => {
                    self.advance();
                    items.push(self.expect_ident()?);
                }
                Token::And => {
                    self.advance();
                    items.push(self.expect_ident()?);
                    break;
                }
                _ => break,
            }
        }
        Ok(items)
    }

    pub(super) fn parse_arg_list(&mut self) -> Result<Vec<Expr>, ParseError> {
        let mut args = vec![self.parse_expr()?];
        loop {
            match self.peek() {
                Token::Comma => {
                    self.advance();
                    args.push(self.parse_expr()?);
                }
                Token::And => {
                    self.advance();
                    args.push(self.parse_expr()?);
                    break;
                }
                _ => break,
            }
        }
        Ok(args)
    }
}
