#[derive(Clone, Debug)]
pub struct IfTree {
	/// The initial condition (if .. then ..)
	pub initial: (Expression, Block),
	/// The tree of statements that follow (executed in order)
	pub elseifs: Vec<(Expression, Block)>,
	/// The last statement to execute of all other conditions are false
	pub otherwise: Option<Block>,
}
