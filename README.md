# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## Descrição do Projeto
Sistema de alta performance para indexação e busca de produtos utilizando Tabelas Hash e sistema de recomendação via Grafos.

## Tecnologias Utilizadas
* Linguagem Rust
* Cargo (Gerenciador e Ferramenta de Testes) 

## Arquitetura e Algoritmos
O sistema utiliza um **Índice Invertido com Tabelas Hash** para garantir que buscas em milhões de itens ocorram em tempo constante O(1). As recomendações são estruturadas em um **Grafo** de adjacência.

## Como Executar
1. Execute o programa: `cargo run`
2. Execute os testes: `cargo test` 