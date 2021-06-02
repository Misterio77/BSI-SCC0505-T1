use std::collections::HashMap;
use std::fmt;

use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use petgraph::{Direction, Graph};

/// Um autômato
pub struct Automaton {
    /// Transições de estados
    transitions: Graph<u16, char>,
    /// Símbolos válidos
    symbols: Vec<char>,
    /// Estado(s) iniciais
    initial_states: Vec<NodeIndex>,
    /// Estado(s) aceitos
    accepted_states: Vec<NodeIndex>,
}

impl Automaton {
    pub fn verify_chain(&self, chain: &[char]) -> bool {
        // Obter estado inicial
        let mut current_state = match self.initial_states.get(0) {
            Some(initial) => *initial,
            None => return false,
        };

        for symbol in chain {
            // Caso o símbolo seja -, não faça nenhuma mudança de estado (cadeia vazia)
            if *symbol == '-' {
                break;
            }
            // Caso o símbolo não esteja na linguagem, rejeite a cadeia
            if !self.symbols.contains(symbol) {
                return false;
            }

            // Pegar as transições que saem daquele estado
            let mut edges = self
                .transitions
                .edges_directed(current_state, Direction::Outgoing);

            // Buscar a transição cujo símbolo bate com o símbolo que estamos verificando
            // Caso não encontremos essa transição, rejeitar a cadeia
            let transition = match edges.find(|x| x.weight() == symbol) {
                Some(transition) => transition,
                None => return false,
            };
            // Armazenar o próximo estado
            current_state = transition.target();
        }
        // Verificar se o estado atual (final) é aceitável. Se sim, aceitar. Se não, rejeitar.
        self.accepted_states.contains(&current_state)
    }
}

/// Construtor de autômato
/// Existe para simplificar a construção, ao invés de chamar um construtor com vários argumentos
pub struct AutomatonBuilder {
    pub states: Vec<u16>,
    pub symbols: Vec<char>,
    pub initial_states: Vec<u16>,
    pub accepted_states: Vec<u16>,
    pub transitions: Vec<(u16, char, u16)>,
}

impl AutomatonBuilder {
    /// Cria um novo automato, baseado nos dados do construtor
    pub fn build(&self) -> Result<Automaton, AutomatonError> {
        // Criar um grafo, com tamanho do número de estados, e quantidade de vértices igual ao
        // número de estados multiplicados pelo número de símbolos
        let mut transitions_graph =
            Graph::with_capacity(self.states.len(), self.states.len() * self.symbols.len());

        // Vetor que irá armazenar os índices (no grafo) dos estados iniciais
        let mut initial_states_indexes = Vec::new();
        // Vetor que irá armazenar os índices (no grafo) dos estados aceitos
        let mut accepted_states_indexes = Vec::new();

        // Vamos guardar os índices dos estados no  grafo num mapa hash temporário,
        // pra facilitar na hora de marcar as transições
        let mut index = HashMap::new();

        // Adicionar todos os estados em nós do grafo
        for state in &self.states {
            // Adicionar o estado ao grafo, e guardar seu índice
            let state_index = transitions_graph.add_node(*state);

            // Adicionar o estado e seu índice no nosso hashmap
            index.insert(state, state_index);

            // Caso o estado seja um dos iniciais
            if self.initial_states.contains(&state) {
                initial_states_indexes.push(state_index);
            }
            // Caso o estado seja um dos aceitos
            if self.accepted_states.contains(&state) {
                accepted_states_indexes.push(state_index);
            }
        }

        // Adicionar todos as transições em vértices do grafo
        for transition in self.transitions.iter() {
            // Estado pré
            let q0 = index.get(&transition.0);
            // Estado pós
            let q1 = index.get(&transition.2);
            // Símbolo
            let x = transition.1;

            // Verificar que q0 e q1 existem nos estados, e que x existe nos símbolos
            // Retornar um erro caso contrário
            let (q0, q1) = match (q0, q1, self.symbols.contains(&x)) {
                (Some(q0), Some(q1), true) => Ok((q0, q1)),
                _ => Err(AutomatonError::InvalidTransition(*transition)),
            }?;

            // Adicionar vértice ao grafo
            transitions_graph.add_edge(*q0, *q1, x);
        }

        // Retornar autômato criado
        Ok(Automaton {
            transitions: transitions_graph,
            symbols: self.symbols.clone(),
            initial_states: initial_states_indexes,
            accepted_states: accepted_states_indexes,
        })
    }
}

/// Possíveis erros que podem ocorrer
#[derive(Debug, Clone, Copy)]
pub enum AutomatonError {
    /// Ocorre quando uma transição não é válida
    InvalidTransition((u16, char, u16)),
}

/// Marcar como erro
impl std::error::Error for AutomatonError {}

/// Mensagens de erro
impl fmt::Display for AutomatonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AutomatonError::InvalidTransition((q0, x, q1)) => {
                write!(f, "A transição {:?} {:?} {:?} é inválida", q0, x, q1)
            }
        }
    }
}
