# 📚 Rust (contador_web)

> Documento pessoal de aprendizado. Projeto: contador web reativo com [Leptos](https://leptos.dev/).  
> Arquivo de referência: `src/app.rs`

---

## 🗂️ Overview — Conceitos aplicados no projeto

| # | Conceito | Categoria |
|---|----------|-----------|
| 1 | **Closures** (`move \|_\|`) | Funções |
| 2 | **`move` em closures** — captura de ownership | Ownership |
| 3 | **Ownership com Signals** — por que `move` funciona aqui | Ownership / Leptos |
| 4 | **Mutabilidade controlada via callbacks** (`update`) | Mutabilidade |
| 5 | **`Vec<T>`** — criação, `push`, `pop`, `new()` | Coleções |
| 6 | **Iteração com `into_iter()`** — consumo do vetor | Iteração |
| 7 | **Encadeamento de iteradores** (`.rev()`, `.map()`, `.collect_view()`) | Iteração |
| 8 | **Tipos genéricos** (`Vec<i32>`) | Tipos |
| 9 | **Expressões condicionais** como valores (`if/else` retornando `View`) | Controle de fluxo |
| 10 | **Closures em atributos reativos** (`disabled=move \|\| ...`) | Leptos / Closures |

---

## 📖 Detalhes — Para revisar depois

### 1. Closures (`move |_| { ... }`)

```rust
let increment = move |_: leptos::ev::MouseEvent| {
    // corpo
};
```

Uma **closure** é uma função anônima que pode capturar variáveis do escopo onde foi criada.  
`|_|` significa: recebo um argumento mas não me importo com ele (o evento de clique).

---

### 2. `move` em closures — transferência de ownership

```rust
let increment = move |_: leptos::ev::MouseEvent| { ... };
```

O `move` faz a closure **tomar posse** (ownership) das variáveis capturadas, em vez de apenas pegar emprestado.  
Isso é necessário quando a closure vai ser usada em outro contexto (ex: passada como callback).

---

### 3. Por que `move` não "destrói" os signals?

```rust
// count e set_count são usados em múltiplas closures com move
let increment = move |_| { count.get(); set_count.set(...); };
let decrement = move |_| { count.get(); set_count.set(...); };
let reset     = move |_| { set_count.set(0); };
```

> **O segredo:** `count` e `set_count` são **Signals do Leptos**, que internamente são tipos `Copy`.  
> Isso significa que ao usar `move`, cada closure recebe **uma cópia** do handle do signal — não move o valor real.  
> Se fossem tipos não-`Copy` (ex: `String`, `Vec`), só a primeira closure poderia usar após o `move`.

---

### 4. Mutabilidade controlada via `update`

```rust
set_history.update(|hist| {
    hist.push(current);
});
```

`hist` aqui é uma referência mutável (`&mut Vec<i32>`) passada pelo Leptos.  
A mutação acontece **dentro do callback**, de forma segura e controlada — você não precisa declarar `mut` explicitamente.

---

### 5. `Vec<T>` — vetor dinâmico

```rust
let (history, set_history) = create_signal(Vec::<i32>::new());
```

- `Vec::new()` → cria vetor vazio
- `.push(valor)` → adiciona ao final
- `.pop()` → remove o último elemento
- `Vec::new()` como reset → substitui o vetor inteiro por um novo vazio

---

### 6. `into_iter()` — iteração com consumo

```rust
history.get().into_iter().rev().map(|v| ...).collect_view()
```

- `into_iter()` **consome** o valor e produz um iterador — o vetor original não pode mais ser usado depois disso.
- Como `history.get()` retorna um clone do valor (comportamento do Signal), isso é seguro aqui.
- Alternativa sem consumo: `.iter()` → itera por referência sem mover os dados.

---

### 7. Encadeamento de iteradores

```rust
history.get()
    .into_iter()
    .rev()
    .map(|v| view! { <li>{v}</li> })
    .collect_view()
```

- `.rev()` → inverte a ordem da iteração (mostra o mais recente primeiro)
- `.map(|v| ...)` → transforma cada elemento — aqui converte `i32` em um nó de UI
- `.collect_view()` → método do Leptos que coleta os itens num tipo renderizável

---

### 8. Tipos genéricos — `Vec<i32>`

```rust
Vec::<i32>::new()
```

`Vec<T>` é um tipo genérico — o `T` define o tipo dos elementos.  
A sintaxe `::<i32>` é o **turbofish**, usada para informar o tipo quando o compilador não consegue inferir sozinho.

---

### 9. `if/else` como expressão

```rust
if history.get().is_empty() {
    ().into_view()
} else {
    view! { ... }.into_view()
}
```

Em Rust, `if/else` é uma **expressão** — retorna um valor.  
Aqui ele retorna diferentes tipos de `View` dependendo do estado, sem precisar de `return`.

---

### 10. Closures em atributos reativos

```rust
disabled=move || count.get() == 0
```

Leptos aceita closures como valores de atributos para reatividade.  
Essa closure é executada automaticamente toda vez que `count` muda, atualizando o atributo `disabled` do botão em tempo real.

---

## 🔗 Referências

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Leptos Book](https://book.leptos.dev/)
- [Leptos Rust framework](https://leptos.dev/)
- [Rust by Example — Closures](https://doc.rust-lang.org/rust-by-example/fn/closures.html)
