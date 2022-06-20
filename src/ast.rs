use std::cell::RefCell;
use std::fmt::{Debug};
use std::rc::Rc;

// -----------------------------------------------------------------------------

pub struct ProgramInformation {
  pub generic_functions: RefCell<Vec<Rc<FunctionDeclaration>>>,
  pub generic_function_calls: RefCell<Vec<Rc<Expression>>>
}

impl ProgramInformation {
  pub fn new() -> Self {
    Self {
      generic_functions: RefCell::new(Vec::new()),
      generic_function_calls: RefCell::new(Vec::new())
    }
  }
}

// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct Program {
  pub statements: Vec<Statement>
}

// -----------------------------------------------------------------------------

#[derive(Debug)]
pub enum Statement {
  Expression(Rc<Expression>),
  FunctionDeclaration(Rc<FunctionDeclaration>),
  ClassDeclaration(ClassDeclaration),
  StructDeclaration(StructDeclaration)
}

// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct ClassDeclaration {
  pub class_type: ClassType,
  pub name: String,
  pub extended_class_name: Option<String>,
  pub body_statements: Vec<ClassBodyStatement>
}

#[derive(Debug)]
pub enum ClassType {
  Class,
  StatemachineClass
}

#[derive(Debug)]
pub enum ClassBodyStatement {
  Method {
    encapsulation: Option<EncapsulationType>,
    function_declaration: Rc<FunctionDeclaration>
  },
  Property {
    encapsulation: Option<EncapsulationType>,
    property_declaration: VariableDeclaration
  },
  DefaultValue(VariableAssignment)
}

#[derive(Debug)]
pub enum EncapsulationType {
  Private,
  Public,
  Protected
}

// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct StructDeclaration {
  pub name: String,
  pub body_statements: Vec<StructBodyStatement>
}

#[derive(Debug)]
pub enum StructBodyStatement {
  Property(VariableDeclaration),
  DefaultValue(VariableAssignment)
}

// -----------------------------------------------------------------------------

#[derive(Debug)]
/// property.
pub struct FunctionDeclaration {
  pub function_type: FunctionType,
  pub name: String,
  pub generic_types: Option<Vec<String>>,
  pub parameters: Vec<TypedIdentifier>,
  pub type_declaration: Option<TypeDeclaration>,
  pub body_statements: Vec<FunctionBodyStatement>,
  pub is_latent: bool
}

#[derive(Debug)]
pub enum FunctionType {
  Function,
  Timer,
  Event
}

#[derive(Debug)]
pub enum FunctionBodyStatement {
  VariableDeclaration(VariableDeclaration),
  Expression(Rc<Expression>),
  Return(Rc<Expression>),
  Assignement(VariableAssignment),
  IfStatement(IfStatement),
  ForStatement(ForStatement),
  WhileStatement(WhileStatement),
  DoWhileStatement(DoWhileStatement)
}

// -----------------------------------------------------------------------------

#[derive(Debug)]
pub enum IfStatement {
  If {
    condition: Rc<Expression>,
    body_statements: Vec<FunctionBodyStatement>,
    else_statements: Vec<Box<IfStatement>>
  },
  Else {
    condition: Option<Rc<Expression>>,
    body_statements: Vec<FunctionBodyStatement>
  }
}

// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct VariableAssignment {
  pub variable_name: Box<IdentifierTerm>,
  pub assignment_type: AssignmentType,
  pub following_expression: Rc<Expression>
}

// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct ForStatement {
  pub initialization: Option<VariableDeclarationOrAssignment>,
  pub condition: Rc<Expression>,
  pub iteration: VariableAssignment,
  pub body_statements: Vec<FunctionBodyStatement>
}

#[derive(Debug)]
pub enum VariableDeclarationOrAssignment {
  Declaration(VariableDeclaration),
  Assignement(VariableAssignment)
}

// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct WhileStatement {
  pub condition: Rc<Expression>,
  pub body_statements: Vec<FunctionBodyStatement>
}

#[derive(Debug)]
pub struct DoWhileStatement {
  pub condition: Rc<Expression>,
  pub body_statements: Vec<FunctionBodyStatement>
}

// -----------------------------------------------------------------------------


#[derive(Debug)]
pub struct VariableDeclaration {
  pub declaration: TypedIdentifier,
  pub following_expression: Option<Rc<Expression>>
}

#[derive(Debug)]
pub struct FunctionCallParameters(pub Vec<Rc<Expression>>);

#[derive(Debug)]
pub struct IdentifierTerm {
  pub text: String,
  pub indexing: Option<Rc<Expression>>,
  pub nesting: Option<Box<IdentifierTerm>>
}

#[derive(Debug)]
pub struct TypedIdentifier {
  pub name: String,
  pub type_declaration: TypeDeclaration
}

/// Represents a type declaration that could be after anything, for example
/// ```
/// a: int
/// ```
/// 
/// `: int` is the typeDeclaration
#[derive(Debug)]
pub struct TypeDeclaration {
  pub type_name: String,
  pub generic_type_assignment: Option<Vec<TypeDeclaration>>
}

#[derive(Debug)]
pub enum Expression {
  Number(i32),

  String(String),

  Identifier(Box<IdentifierTerm>),

  FunctionCall {
    accessor: Box<IdentifierTerm>,
    generic_types: Option<Vec<String>>,
    parameters: FunctionCallParameters
  },

  /// An operation between two expressions
  Operation(Rc<Expression>, OperationCode, Rc<Expression>),
  Error,
}

#[derive(Copy, Clone, Debug)]
pub enum OperationCode {
  Mul,
  Div,
  Add,
  Sub,
  Comparison(ComparisonType)
}

#[derive(Debug)]
pub enum AssignmentType {
  Equal,
  PlusEqual,
  MinusEqual,
  AsteriskEqual,
  SlashEqual
}

#[derive(Copy, Clone, Debug)]
pub enum ComparisonType {
  Greater,
  GreaterEqual,
  Lower,
  LowerEqual,
  Equal,
  Different
}