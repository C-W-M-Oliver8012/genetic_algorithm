use rand;

#[derive(Copy, Clone, PartialEq)]
enum NodeType {
	Bias,
	Input,
	Hidden,
	Output,
}

#[derive(Copy, Clone)]
pub struct Node {
	node_type: NodeType,
	value: f64,
	layer: usize,
}

#[derive(Copy, Clone)]
pub struct Connection {
	input: usize,
	output: usize,
	weight: f64,
}

#[derive(Clone)]
pub struct NN {
	num_bias: usize,
	num_inputs: usize,
	num_outputs: usize,
	num_hidden: usize,
	num_layers: usize,
	nodes: Vec<Node>,
	connections: Vec<Connection>,
}

impl NN {
	pub fn new(nn_info: Vec<usize>) -> NN {
		let num_inputs = nn_info[0];
		let num_outputs = nn_info[nn_info.len() - 1];
		let mut num_hidden = 0;
		for i in 0..nn_info.len() {
			num_hidden += nn_info[i];
		}
		num_hidden -= num_inputs - num_outputs;

		let mut nn = NN {
			num_bias: 1,
			num_inputs: num_inputs,
			num_outputs: num_outputs,
			num_hidden: num_hidden,
			num_layers: nn_info.len(),
			nodes: Vec::new(),
			connections: Vec::new(),
		};

		nn.init_nodes(nn_info);
		nn.init_connections();

		nn
	}

	fn init_nodes(&mut self, nn_info: Vec<usize>) {
		// add bias node
		self.nodes.push(Node {
			node_type: NodeType::Bias,
			value: 1.0,
			layer: 0,
		});
		// create nodes
		for i in 0..nn_info.len() {
			for _j in 0..nn_info[i] {
				if i == 0 {
					self.nodes.push(Node {
						node_type: NodeType::Input,
						value: 0.0,
						layer: i,
					});
				} else if i == nn_info.len() - 1 {
					self.nodes.push(Node {
						node_type: NodeType::Output,
						value: 0.0,
						layer: i,
					});
				} else {
					self.nodes.push(Node {
						node_type: NodeType::Hidden,
						value: 0.0,
						layer: i,
					});
				}
			}
		}
	}

	fn init_connections(&mut self) {
		// create connections
		for i in 0..self.nodes.len() {
			for j in 0..self.nodes.len() {
				match self.nodes[i].node_type {
					NodeType::Bias => {
						if self.nodes[j].node_type != NodeType::Bias && self.nodes[j].node_type != NodeType::Input {
							self.connections.push(Connection {
								input: i,
								output: j,
								weight: rand::random::<f64>() * 2.0 - 1.0,
							});
						}
					},
					NodeType::Input | NodeType::Hidden => {
						if self.nodes[i].layer + 1 == self.nodes[j].layer {
							self.connections.push(Connection {
								input: i,
								output: j,
								weight: rand::random::<f64>() * 2.0 - 1.0,
							});
						}
					},
					_ => {},
				}
			}
		}
	}

	pub fn print_nodes(&self) {
		for i in 0..self.nodes.len() {
			println!("Node {}", i);
			match self.nodes[i].node_type {
				NodeType::Bias => println!("node_type: Bias"),
				NodeType::Input => println!("node_type: Input"),
				NodeType::Hidden => println!("node_type: Hidden"),
				NodeType::Output => println!("node_type: Output"),
			}
			println!("value: {}", self.nodes[i].value);
			println!("layer: {}", self.nodes[i].layer);
			println!("");
		}
	}

	pub fn print_connections(&self) {
		for i in 0..self.connections.len() {
			println!("Connection {}", i);
			println!("input: {}", self.connections[i].input);
			println!("output: {}", self.connections[i].output);
			println!("weight: {}", self.connections[i].weight);
			println!("");
		}
	}

	// this should only be used with NNs that have
	// the exact same structures
	pub fn crossover(&self, parent2: NN) -> NN {
		let mut baby_nn = NN {
			num_bias: self.num_bias,
			num_inputs: self.num_inputs,
			num_outputs: self.num_outputs,
			num_hidden: self.num_hidden,
			num_layers: self.num_layers,
			nodes: self.nodes.clone(),
			connections: self.connections.clone(),
		};

		let split = rand::random::<usize>() % self.connections.len();

		for i in 0..self.connections.len() {
			let mutation_chance = rand::random::<f64>();
			if mutation_chance <= 0.01 {
				baby_nn.connections[i].weight = rand::random::<f64>() * 2.0 - 1.0;
			} else {
				if i <= split {
					baby_nn.connections[i].weight = self.connections[i].weight;
				} else {
					baby_nn.connections[i].weight = parent2.connections[i].weight;
				}
			}
		}
		baby_nn
	}

	pub fn feed_forward(&mut self, inputs: Vec<f64>) -> Result<Vec<f64>, &str> {
		// set inputs or return error
		if inputs.len() != self.num_inputs {
			return Err("Inputs do not match.");
		} else {
			for i in 0..inputs.len() {
				self.nodes[i+1].value = inputs[i];
			}
		}

		// resets hidden and output node values to 0.0
		for i in 0..self.nodes.len() {
			match self.nodes[i].node_type {
				NodeType::Hidden | NodeType::Output => self.nodes[i].value = 0.0,
				_ => {},
			}
		}

		// feed forward the neural network
		for i in 0..self.connections.len() {
			let input = self.connections[i].input;
			let output = self.connections[i].output;
			let weight = self.connections[i].weight;
			match self.nodes[input].node_type {
				NodeType::Bias | NodeType::Input =>
					self.nodes[output].value += self.nodes[input].value * weight,
				_ => self.nodes[output].value += leaky_relu(self.nodes[input].value) * weight,
			}
		}

		// set and return outputs
		let mut outputs = Vec::new();
		for i in 0..self.nodes.len() {
			match self.nodes[i].node_type {
				NodeType::Output => {
					self.nodes[i].value = leaky_relu(self.nodes[i].value);
					outputs.push(self.nodes[i].value);
				}
				_ => {},
			}
		}
		Ok(outputs)
	}
}

pub fn sigmoid(mut x: f64) -> f64 {
	x = x * -1.0;
	1.0 / (1.0 + x.exp())
}

pub fn leaky_relu(x: f64) -> f64 {
	if x >= 0.0 {
		return x;
	} else {
		return x * 0.1;
	}
}