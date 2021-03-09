pub fn init<TokenId>() -> Vec<(TokenId, TokenId)>
where
	TokenId: Default,
{
	vec![]
}

pub fn assign<TokenId>(relations: &mut Vec<(TokenId, TokenId)>, parent: TokenId, child: TokenId) -> bool
where
	TokenId: PartialEq,
{
	// check assigned status
	if relations.iter_mut().position(|i| i.1 == child) == None {
		relations.push((child, parent));
		return true;
	}

	false
}

pub fn unassign<TokenId>(relations: &mut Vec<(TokenId, TokenId)>, node: TokenId) -> bool
where
	TokenId: Default + PartialEq,
{
	let index = match relations.iter_mut().position(|i| i.0 == node) {
		Some(i) => i,
		None => return false,
	};

	relations.remove(index);
	relations.push((node, TokenId::default()));
	true
}

pub fn remove<TokenId>(relations: &mut Vec<(TokenId, TokenId)>, node: TokenId) -> bool
where
	TokenId: Default + PartialEq,
{
	let index = match relations.iter_mut().position(|i| i.0 == node) {
		Some(i) => i,
		None => return false,
	};

	relations.remove(index);
	true
}

pub fn get_flatten_childs_subtree<TokenId>(relations: &mut Vec<(TokenId, TokenId)>, node: TokenId) -> Vec<TokenId>
where
	TokenId: Default + PartialEq + Clone + Copy,
{
	let mut childs = get_flatten_childs(relations, vec![node]);
	childs.remove(0);
	childs
}

pub fn get_flatten_childs<TokenId>(relations: &mut Vec<(TokenId, TokenId)>, items: Vec<TokenId>) -> Vec<TokenId>
where
	TokenId: Default + PartialEq + Clone + Copy,
{
	let mut result: Vec<TokenId> = Vec::new();

	for token in items {
		result.push(token);
		let chld = relations
			.iter_mut()
			.filter(|i| i.1 == token)
			.map(|v| v.0)
			.collect::<Vec<_>>();
		if chld.is_empty() {
			result.extend(get_flatten_childs(relations, chld))
		}
	}
	result
}

pub fn move_node<TokenId>(relations: &mut Vec<(TokenId, TokenId)>, node: TokenId, new_parent: TokenId) -> bool
where
	TokenId: Default + PartialEq + Clone + Copy,
{
	let index = match relations.iter_mut().position(|i| i.0 == node) {
		Some(i) => i,
		None => return false,
	};

	if move_node(relations, node, new_parent) {
		relations.remove(index);
		relations.push((node, new_parent));
		return true;
	}
	false
}

pub fn can_assign<TokenId>(relations: &[(TokenId, TokenId)], node: TokenId) -> bool
where
	TokenId: Default + PartialEq + Clone + Copy,
{
	// check assigned status
	relations.iter().position(|i| i.0 == node) == None
}

pub fn can_move_node<TokenId>(relations: &[(TokenId, TokenId)], node: TokenId, new_parent: TokenId) -> bool
where
	TokenId: Default + PartialEq + Clone + Copy,
{
	// check relations
	let mut p: TokenId = new_parent;
	while p != TokenId::default() {
		if p == node {
			return false;
		}
		p = relations.iter().find(|i| i.0 == p).unwrap().1;
	}
	true
}
