#[derive(Clone, Debug)]
pub enum Statement {
	End,
	Definition((VariableList, ExpressionList)),
	FunctionCall(FunctionCall),
	Label(Label),
	Break,
	Goto(Identifier),
	Do(Box<Block>),
	While((Expression, Block)),
	RepeatUntil(Block, Expression),
	IfTree(IfTree),
	ForExpression(
		(
			Identifier,
			(Expression, Expression, Option<Expression>),
			Block,
		),
	),
	ForList(((IdentifierList, ExpressionList), Block)),
	FunctionDefinition((FunctionIdentifier, FunctionBody)),
	LocalFunctionDefinition((Identifier, FunctionBody)),
	LocalDefinitionWithAttribute((AttributeNameList, Option<ExpressionList>)),
}
