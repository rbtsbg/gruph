# Gruph
grep + rust + graph =  gruph; a lightweight regex matcher for graphs

## Vision 

Gruph shall become a lightweight regex matcher for graphs. 

More specifically, it probably will sit on top of [petgraph](https://github.com/petgraph/petgraph) and allow to query graphs with regular expressions. 

A candidate query language is described by [Wang et al. (2019)](https://arxiv.org/pdf/1904.11653.pdf) but the project could also follow [tgrep2](https://web.stanford.edu/dept/linguistics/corpora/cas-tut-tgrep.html) or [Tregex](https://nlp.stanford.edu/software/tregex.html) and focus on tree-like graphs only. 

Ultimately, the library shall be used for a binary CLI that allows to quickly match patterns in trees, like the latter mentioned libraries. 

## Scope 

This is a pet project to learn the Rust programming language, which is also the main reason why Rust is chosen as the programming language. If I abandon this project at some point, feel free to adopt it. If you know how to improve my code, don't hesitate to pull request. 
