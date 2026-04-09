use super::*;

impl Parser {
    pub(super) fn parse_top_decl(&mut self) -> Result<TopDecl, ParseError> {
        match self.peek().clone() {
            Token::Scripture => self.parse_scripture(),
            Token::Sin       => self.parse_sin_decl(),
            Token::Covenant  => self.parse_covenant_decl(),
            Token::Salm      => self.parse_salm_decl(),
            t => {
                let sp = self.sp().clone();
                Err(ParseError::at(
                    format!(
                        "'{}' is not worthy to begin a declaration — speak one of these sacred words: 'salm', 'scripture', 'sin', or 'covenant'",
                        token_name(&t)
                    ),
                    sp.line,
                    sp.col,
                ))
            }
        }
    }

    fn parse_scripture(&mut self) -> Result<TopDecl, ParseError> {
        self.expect(&Token::Scripture)?;
        let name        = self.expect_ident()?;
        let type_params = self.parse_type_params()?;

        self.expect(&Token::Indent)?;
        let mut fields = Vec::new();
        while !matches!(self.peek(), Token::Dedent | Token::Eof) {
            let fname = self.expect_ident()?;
            self.expect(&Token::Of)?;
            let ty = self.parse_type()?;
            fields.push((fname, ty));
        }
        self.expect(&Token::Dedent)?;

        if fields.is_empty() {
            let sp = self.sp().clone();
            return Err(ParseError::at(
                format!("the scripture '{}' must bear at least one field — an empty scripture is a profanation", name),
                sp.line,
                sp.col,
            ));
        }

        Ok(TopDecl::Scripture { name, type_params, fields })
    }

    fn parse_sin_decl(&mut self) -> Result<TopDecl, ParseError> {
        self.expect(&Token::Sin)?;
        let name = self.expect_ident()?;

        let fields = if self.peek() == &Token::Indent {
            self.advance();
            let mut fs = Vec::new();
            while !matches!(self.peek(), Token::Dedent | Token::Eof) {
                let fname = self.expect_ident()?;
                self.expect(&Token::Of)?;
                let ty = self.parse_type()?;
                fs.push((fname, ty));
            }
            self.expect(&Token::Dedent)?;
            fs
        } else {
            Vec::new()
        };

        Ok(TopDecl::SinDecl { name, fields })
    }

    fn parse_covenant_decl(&mut self) -> Result<TopDecl, ParseError> {
        self.expect(&Token::Covenant)?;
        let name        = self.expect_ident()?;
        let type_params = self.parse_type_params()?;
        self.expect(&Token::Indent)?;

        let mut variants = Vec::new();
        while !matches!(self.peek(), Token::Dedent | Token::Eof) {
            let variant_name = self.expect_ident()?;
            let fields = if self.peek() == &Token::Indent {
                self.advance();
                let mut fs = Vec::new();
                while !matches!(self.peek(), Token::Dedent | Token::Eof) {
                    let fname = self.expect_ident()?;
                    self.expect(&Token::Of)?;
                    let ty = self.parse_type()?;
                    fs.push((fname, ty));
                }
                self.expect(&Token::Dedent)?;
                fs
            } else {
                Vec::new()
            };
            variants.push(CovenantVariantDecl { name: variant_name, fields });
        }

        self.expect(&Token::Dedent)?;

        if variants.is_empty() {
            let sp = self.sp().clone();
            return Err(ParseError::at(
                format!("the covenant '{}' must bear at least one variant — an empty covenant is heresy", name),
                sp.line,
                sp.col,
            ));
        }

        Ok(TopDecl::Covenant { name, type_params, variants })
    }

    fn parse_salm_decl(&mut self) -> Result<TopDecl, ParseError> {
        self.expect(&Token::Salm)?;
        let name = self.expect_ident()?;

        if self.peek() == &Token::Upon {
            self.advance();
            let target_type = self.expect_ident()?;
            let type_params = self.parse_type_params()?;
            let params      = self.parse_optional_params()?;
            self.expect(&Token::Reveals)?;
            let ret_type = self.parse_type()?;
            let body     = self.parse_block()?;
            Ok(TopDecl::MethodSalm { name, type_params, target_type, params, ret_type, body })
        } else {
            let type_params = self.parse_type_params()?;
            let params      = self.parse_optional_params()?;
            self.expect(&Token::Reveals)?;
            let ret_type = self.parse_type()?;
            let body     = self.parse_block()?;
            Ok(TopDecl::Salm { name, type_params, params, ret_type, body })
        }
    }
}
