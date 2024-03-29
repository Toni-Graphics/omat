use crate::ast::*;

pub struct Ir {
    c_code: String,
    exprs: Vec<expr::Expr>
}

impl Ir {
    pub fn new(_exprs: Vec<expr::Expr>) -> Self {
        Ir {
            c_code: String::new(),
            exprs: _exprs,
        }
    }

    pub fn gen(&mut self) {
        self.c_code = String::from(r#"
            #include <stdio.h>
            int main() {
                printf("Hello World");
                return 0;
            }
            "#);
    }

    pub fn get_c_code(&self) -> String {
        self.c_code.clone()
    }
}