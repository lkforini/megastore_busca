use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Produto {
    pub id: u32,
    pub nome: String,
    pub categoria: String,
    pub preco: f64,
}

pub struct MegaStore {
    produtos: HashMap<u32, Produto>,
    indice_palavras: HashMap<String, HashSet<u32>>,
    grafo_recomendacao: HashMap<u32, Vec<u32>>,
}

impl MegaStore {
    pub fn new() -> Self {
        MegaStore {
            produtos: HashMap::new(),
            indice_palavras: HashMap::new(),
            grafo_recomendacao: HashMap::new(),
        }
    }

    pub fn adicionar_produto(&mut self, produto: Produto) {
        let id = produto.id;
        for palavra in produto.nome.to_lowercase().split_whitespace() {
            self.indice_palavras.entry(palavra.to_string()).or_insert_with(HashSet::new).insert(id);
        }
        self.produtos.insert(id, produto);
    }

    pub fn adicionar_recomendacao(&mut self, id_origem: u32, id_destino: u32) {
        self.grafo_recomendacao.entry(id_origem).or_insert_with(Vec::new).push(id_destino);
    }

    pub fn buscar(&self, termo: &str) -> Vec<&Produto> {
        if let Some(ids) = self.indice_palavras.get(&termo.to_lowercase()) {
            ids.iter().filter_map(|id| self.produtos.get(id)).collect()
        } else {
            Vec::new()
        }
    }
}

fn main() {
    let mut loja = MegaStore::new();
    loja.adicionar_produto(Produto { id: 1, nome: "Smartphone".into(), categoria: "Eletrônicos".into(), preco: 2000.0 });
    println!("Sistema MegaStore rodando com sucesso!");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn teste_busca() {
        let mut loja = MegaStore::new();
        loja.adicionar_produto(Produto { id: 1, nome: "Teclado".into(), categoria: "Periféricos".into(), preco: 100.0 });
        assert_eq!(loja.buscar("teclado").len(), 1);
    }
}