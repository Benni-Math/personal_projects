use crate::{agent::*, agent_list::*};

mod agent;
mod agent_list;

struct ModelFrame;

// Small outline of an ECS -- will incorporate with ModelFrame and Agent

// Will want to remove these in the future
// either remove mutable borrowing or find some other alternative
use std::{cell::{RefCell, RefMut}, collections::HashMap};


struct Model {
    // Need to think about parameter list implementation
    parameters: HashMap<String, f64>,
    agents: AgentList,
}

struct AgentList {
    agents_count: usize,
    // Need to think about how to implement the parameter list
    parameters: HashMap<String, f64>,
    variables: Vec<Box<dyn VariableVec>>,
}

// TODO: look at other ECS models
trait VariableVec {
    // Do I want this to be something else?
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn push_none(&mut self);
}

// An Agent is essentially a 'cross-section' of a 'Vec<AgentVariable>'
// i.e. if I want to implement a Agent struct, it should be via some indexing
struct AgentVariable<T>(RefCell<Vec<Option<T>>>);
struct Agent(usize);

impl<T> AgentVariable<T> {
    fn new() -> Self {
        Self(RefCell::new(Vec::new()))
    }

    fn with_capacity(n: usize) -> Self {
        Self(RefCell::new(Vec::with_capacity(n)))
    }

    fn push(&self, var: T) {
        self.0.get_mut().push(Some(var));
    }
}

impl<T> std::ops::Index<Agent> for AgentVariable<T> {
    type Output = Option<T>;

    fn index(&self, index: Agent) -> &Self::Output {
        &self.0.borrow()[index.0]
    }
}

impl<T> std::ops::IndexMut<Agent> for AgentVariable<T> {
    fn index_mut(&mut self, index: Agent) -> &mut Self::Output {
        &mut self.0.borrow_mut()[index.0]
    }
}

// Want to change these implementations
impl<T: 'static> VariableVec for AgentVariable<T> {
    // Unsure if these actually work...
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn push_none(&mut self) {
        self.0.get_mut().push(None)
    }
}

impl AgentList {
    fn new() -> Self {
        Self {
            agents_count: 0,
            parameters: HashMap::new(),
            variables: Vec::new(),
        }
    }

    fn new_agent(&mut self) -> usize {
        let agent_id = self.agents_count;
        self.variables
            .iter_mut()
            .for_each(|variable_vec| {
                variable_vec.push_none();
            });
        
        self.agents_count += 1;
        agent_id
    }

    fn add_variable_to_agent<VariableType: 'static>(
        &mut self,
        agent: usize,
        variable: VariableType,
    ) {
        // Currently, I need a struct/type for each individual variable...
        // Not the most efficient, so might want to change this later
        for variable_vec in self.variables.iter_mut() {
            if let Some(var_vec) = variable_vec
                .as_any_mut()
                .downcast_mut::<AgentVariable<VariableType>>()
            {
                var_vec.0.borrow_mut()[agent] = Some(variable);
                return;
            }
        }

        // No matching variable exists yet
        let mut new_var: AgentVariable<VariableType> 
            = AgentVariable::with_capacity(self.agents_count);
        
        for _ in 0..self.agents_count {
            new_var.push_none();
        }

        new_var[Agent(agent)] = Some(variable);
    }
}

// Should Agents reference their parent model through some `container_of` equiv?
