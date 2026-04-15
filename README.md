# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## Descrição do Projeto
[cite_start]Sistema de alta performance para indexação e busca de produtos utilizando Tabelas Hash e sistema de recomendação via Grafos.

## Tecnologias Utilizadas
* Linguagem Rust
* [cite_start]Cargo (Gerenciador e Ferramenta de Testes) [cite: 90, 93]

## Arquitetura e Algoritmos
[cite_start]O sistema utiliza um **Índice Invertido com Tabelas Hash** para garantir que buscas em milhões de itens ocorram em tempo constante O(1)[cite: 22, 44]. [cite_start]As recomendações são estruturadas em um **Grafo** de adjacência[cite: 4].

## Como Executar
1. Execute o programa: `cargo run`
2. Execute os testes: `cargo test` 