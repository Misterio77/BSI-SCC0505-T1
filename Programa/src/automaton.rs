use std::collections::HashMap;

use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use petgraph::{Direction, Graph};

use crate::{AutomatonError, Result};

/// Um autômato
pub struct Automaton {
    /// Transições de estados
    transitions: Graph<u16, char>,
    /// Símbolos válidos
    symbols: Vec<char>,
    /// Estado(s) iniciais (são armazenados índices p/ nós do grafo)
    initial_states: Vec<NodeIndex>,
    /// Estado(s) aceitos (são armazenados índices p/ nós do grafo)
    accepted_states: Vec<NodeIndex>,
}

impl Automaton {
    /// Cria um novo autômato, dado vetor de estados (u16), símbolos (char),
    /// estados iniciais (u16), estados aceitos (u16) e transições (tripla u16, char, u16)
    pub fn new(
        states: &[u16],
        symbols: &[char],
        initial_states: &[u16],
        accepted_states: &[u16],
        transitions: &[(u16, char, u16)],
    ) -> Result<Automaton> {
        // Criar um grafo, com tamanho do número de estados, e quantidade de vértices igual ao
        // número de estados multiplicados pelo número de símbolos
        let mut transitions_graph =
            Graph::with_capacity(states.len(), states.len() * symbols.len());

        // Vetor que irá armazenar os índices (no grafo) dos estados iniciais
        let mut initial_states_indexes = Vec::new();
        // Vetor que irá armazenar os índices (no grafo) dos estados aceitos
        let mut accepted_states_indexes = Vec::new();

        // Vamos guardar os índices dos estados no  grafo num mapa hash temporário,
        // pra facilitar na hora de marcar as transições
        let mut index = HashMap::new();

        // Adicionar todos os estados em nós do grafo
        for state in states {
            // Adicionar o estado ao grafo, e guardar seu índice
            let state_index = transitions_graph.add_node(*state);

            // Adicionar o estado e seu índice no nosso hashmap
            index.insert(state, state_index);

            // Caso o estado seja um dos iniciais
            if initial_states.contains(&state) {
                initial_states_indexes.push(state_index);
            }
            // Caso o estado seja um dos aceitos
            if accepted_states.contains(&state) {
                accepted_states_indexes.push(state_index);
            }
        }

        // Adicionar todos as transições em vértices do grafo
        for transition in transitions.iter() {
            // Estado pré
            let q0 = index.get(&transition.0);
            // Estado pós
            let q1 = index.get(&transition.2);
            // Símbolo
            let x = transition.1;

            // Verificar que q0 e q1 existem nos estados, e que x existe nos símbolos (ou é -)
            // Retornar um erro caso contrário
            let (q0, q1) = match (q0, q1, (symbols.contains(&x) || x == '-')) {
                (Some(q0), Some(q1), true) => Ok((q0, q1)),
                _ => Err(AutomatonError::InvalidTransition(*transition)),
            }?;

            // Adicionar vértice ao grafo
            transitions_graph.add_edge(*q0, *q1, x);
        }

        // Retornar autômato criado
        Ok(Automaton {
            transitions: transitions_graph,
            symbols: symbols.into(),
            initial_states: initial_states_indexes,
            accepted_states: accepted_states_indexes,
        })
    }
    /// Dado uma cadeia, retorna se ela é válida ou não dentro da linguagem
    pub fn verify_chain(&self, chain: &[char]) -> bool {
        // Caso a cadeia contenha '-' (indica lambda), transformar num slice vazio
        let chain = if chain.contains(&'-') {
            &[]
        } else {
            chain
        };

        for symbol in chain.iter() {
            // Caso algum dos símbolos da cadeia não exista na linguagem
            if !self.symbols.contains(symbol) {
                return false;
            }
        }

        // Para cada possibilidade de estado inicial
        for initial_state in &self.initial_states {
            // Caso a função recursiva retorne true, retornar true também
            // Se não, tentar com o próximo
            if self.verify_chain_recursive(initial_state, chain) {
                return true;
            }
        }
        // Caso nenhum tenha retornado true, retornar false
        false
    }
    /// Função recursiva auxiliar (não é pública)
    fn verify_chain_recursive(&self, current_state: &NodeIndex, chain: &[char]) -> bool {
        // Pegar todas as transições saindo do estado atual
        let edges = self
            .transitions
            .edges_directed(*current_state, Direction::Outgoing);

        // Caso a chain não esteja vazia
        if !chain.is_empty() {
            // Pegar as arestas que contém o elemento atual da cadeia
            let matching_edges = edges.clone().filter(|edge| *edge.weight() == chain[0]);

            // Para cada aresta
            for edge in matching_edges {
                // Seguir no vértice, tirando o elemento atual da cadeia
                if self.verify_chain_recursive(&edge.target(), &chain[1..]) {
                    // Caso em algum desses esteja verificado, propagar isso
                    return true;
                }
            }
        // Caso esteja vazia, e o estado atual bate com um dos aceitos
        } else if self.accepted_states.contains(current_state) {
                return true
        }

        // Filtrar apenas as transições que são "-" (lambdas)
        let lambda_edges = edges.clone().filter(|edge| *edge.weight() == '-');

        // Tentar cada aresta lambda
        for edge in lambda_edges {
            // Chamar ele, com a cadeia intacta (o elemento não é consumido ao entrar num lambda)
            if self.verify_chain_recursive(&edge.target(), chain) {
                // Caso em algum desses esteja verificado, propagar isso
                return true;
            }
        }

        // Se chegou aqui, é pq não tem nenhum aresta (nem mesmo lambda) para seguir
        // E nenhum dos que tentamos seguir caíram num nó aceito
        // Nesse caso, retornar false
        false
    }
}
