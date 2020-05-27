pub mod constants {
    // prefix UQ indicates UNQUALIFIED
    const UQ_OPERATOR_MUL: &str = "unqualified_operator_multiply"; // *
    const UQ_OPERATOR_DIV: &str = "unqualified_operator_divide"; // /
    const UQ_OPERATOR_SUB: &str = "unqualified_operator_subtract"; // -
    const UQ_OPERATOR_ADD: &str = "unqualified_operator_add"; // +
    const UQ_OPERATOR_EXP: &str = "unqualified_operator_exponentiate"; // ^

    //const UQ_COERCE_
}

use crate::helper::lex_wrap::*;

/*pub mod literal {
    pub enum */

#[derive(Debug)]
pub struct Namespace<'a> {
    // maybe add a <symbols> member for better perf checking
    // redeclaration?
    pub public: bool,
    pub name: Option<&'a str>,
    pub contents: Option<ParseUnit<'a>>,

    //
    pub failed: bool,
}

impl<'a> Namespace<'a> {
    pub fn display(&self, f: &mut std::fmt::Formatter<'_>, depth: usize) {
        let _ = writeln!(
            f,
            "{}Namespace that parsed {} with name {} and public {} has children:",
            indent(depth),
            self.failed,
            self.name.unwrap_or("<unnamed>"),
            self.public);

        self.contents.iter().for_each(|contents| contents.display(f, depth+1));
    }

}

/*impl<'a> std::fmt::Debug for Namespace<'a> {
    fn fmt(&self, &mut f: std::fmt::Formatter) -> std::fmt::Result {
        write! /**/
    }
}*/

fn indent(ind: usize) -> String {
    let mut s: String = "|".to_string();

    for _ in 0..ind {
        //s.push('\t');
        s.push_str("  ");
    }

    s
}

fn findent(f: &mut std::fmt::Formatter<'_>, depth: usize) {
    write!(f, "{}", indent(depth)).unwrap();
}

#[derive(Debug)]
pub struct ParseUnit<'a> {
    //namespaces: Vec<Namespace<'a>>,
    pub failed: bool,
    pub declarations: Vec<Result<SymbolDeclaration<'a>, ParseResultError<'a>>>,
    // modrefs (using exprs)?
}

impl<'a> std::fmt::Display for ParseUnit<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = write!(f, "\n");

        self.display(f, 1);

        //self.fmt(f, 0);

        r
        //Ok(())
    }
}

impl<'a> ParseUnit<'a> {
    pub fn display(&self, f: &mut std::fmt::Formatter<'_>, depth: usize) {
        //
        //
        //let _ = write!(f, "{}", indent(depth));

        findent(f, depth);
        let _ = writeln!(f, "ParseUnit that parsed {} with children:", self.failed);

        self.declarations
            .iter()
            .for_each(|elem| {
                      elem
                          .iter()
                          .for_each(
                                |elem| elem.display(f, depth+1)
                          )
            });

        /*for dec in self.declarations {
            dec.display(f, depth + 1);
        }*/
    }
}

#[derive(Debug)]
pub enum SymbolDeclaration<'a> {
    FunctionDeclaration(FunctionDeclaration<'a>),
    StaticDeclaration(VariableDeclaration<'a>),
    NamespaceDeclaration(Namespace<'a>),
}

impl<'a> SymbolDeclaration<'a> {
    pub fn display(&self, f: &mut std::fmt::Formatter<'_>, depth: usize) {
        match self {
            //Self::FunctionDeclaration(fd) => fd.display(f, depth),
            //Self::StaticDeclaration(sd) => sd.display(f, depth),
            Self::NamespaceDeclaration(ns) => ns.display(f, depth),
            _ => {},
        }
    }
}



#[derive(Debug)]
pub struct FunctionDeclaration<'a> {
    pub failed: bool,
    pub expressions: Vec<Box<dyn Expression<'a>>>,
    pub return_type: TypeReference<'a>,
    pub params: Vec<VariableDeclaration<'a>>,
}

#[derive(Debug)]
pub struct VariableDeclaration<'a> {
    pub failed: bool,
    pub name: &'a str,
    pub var_expr: Option<Box<dyn Expression<'a>>>,
    pub var_type: Option<TypeReference<'a>>, // None indicates request for type inference
}

/*pub struct ParseUnit {
    namespace: Namespace,
}*/

#[derive(Debug)]
pub struct TypeReference<'a> {
    pub failed: bool,
    typename: &'a str,
    refers_to: Option<Box<dyn Type>>,
}

#[derive(Debug)]
pub struct AssignmentExpression<'a> {
    lhs: Box<dyn Expression<'a>>,
    rhs: Box<dyn Expression<'a>>,
}

#[derive(Debug)]
pub struct BinaryOperationExpression<'a> {
    operation: BinaryOperation,
    lhs: Box<dyn Expression<'a>>,
    rhs: Box<dyn Expression<'a>>,
}

#[derive(Debug)]
pub struct ComparisonOperationExpression<'a> {
    operation: ComparisonOperation,
    lhs: Box<dyn Expression<'a>>,
    rhs: Box<dyn Expression<'a>>,
}

#[derive(Debug)]
pub enum ComparisonOperation {
    Equal,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    NotEqual,
}

#[derive(Debug)]
pub enum BinaryOperation {
    Multiply,
    Divide,
    Add,
    Subtract,
}

#[derive(Debug)]
pub enum UnaryOperation {
    Negate,
    Invert,
    Dereference,
}

#[derive(Debug)]
pub struct UnaryOperationExpression<'a> {
    operation: UnaryOperation,
    subexpr: Box<dyn Expression<'a>>,
}

#[derive(Debug)]
pub struct TernarySelectorOperationExpression<'a> {
    condition: Box<dyn Expression<'a>>,
    first: Box<dyn Expression<'a>>,
    second: Box<dyn Expression<'a>>,
}

#[derive(Debug)]
pub struct Closure<'a> {
    pub failed: bool,
    pub expressions: Vec<Box<dyn Expression<'a>>>,
    pub return_type: TypeReference<'a>,
    pub params: Vec<VariableDeclaration<'a>>,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub struct IdentifierExpression<'a> {
    pub name: &'a str,
}

pub trait Type: std::fmt::Debug {
}

pub trait Expression<'a>: std::fmt::Debug {
}

impl<'a> Expression<'a> for ComparisonOperationExpression<'a> {
}

impl<'a> Expression<'a> for BinaryOperationExpression<'a> {
}

impl<'a> Expression<'a> for IdentifierExpression<'a> {
}
