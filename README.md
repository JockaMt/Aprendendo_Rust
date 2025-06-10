<h1 style="display: flex; justify-content: center; align-items: center; height: 100px;">
  Aprendendo Rust
</h1>

## Iniciando 🚀

### Passo 1 - Instalar Rust (inclui cargo)

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Esse comando instala o Rust, o Cargo e o rust-analyzer (ferramenta para autocomplete e análise).

### Passo 2 - Criar um novo projeto

```sh
cargo new meu_projeto
cd meu_projeto

cargo build    # compila o projeto
cargo run      # compila e executa o programa
cargo test     # roda os testes (se existirem)
```
Esse comando cria uma estrutura básica com Cargo.toml (onde ficam as dependências) e a pasta src.

### Pronto, já pode começar a _codar_.
---
## Mão no código 🤖

Rust é uma linguagem fortemente tipada, se estiver vindo do python ou javascript isso pode assustar. (me assustou).

Isso significa que toda variável irá ser de um só tipo do começo ao fim do código. ~~A não ser que você sobrescreva a variável...~~

### Definindo variáveis:

Para definir uma variável em rust, usa-se a palavra resevada `let`, sim, como no javascript, porem, todas as variáveis, por padrão são fixas, ou melhor, imutáveis.

Diferente de `const`, que não pode ser alterado de forma alguma, `let` pode ser mudado, porém, não se definido assim:

```rust
let nome = "João";
nome = "Maria"

// ❌ Isso não funciona
```

_Existe também o `static`, que representa uma variável global com tempo de vida `'static` (vive enquanto o programa estiver rodando)._
</br>
_Ao contrário de `const`, `static` pode ser mutável, mas só pode ser definido fora de funções — sim, isso pode parecer estranho no começo, então não vou falar sobre, agora._

#### Numéricos
- Inteiros com sinal (signed):

     - i8, i16, i32, i64, i128, isize

- Inteiros sem sinal (unsigned):

    - u8, u16, u32, u64, u128, usize

- Ponto flutuante:

    - f32, f64

#### Textuais
-   char: Unicode (4 bytes)
    ```rust
    let letra: char = 'A';
    ```
-   &str: (fatia imutável de string)
    ```rust
    fn saudacao(nome: &str) {
        println!("Olá, {}", nome);
    }
    ```
-   String: string alocada dinamicamente e mutável
    ```rust
    let mut nome = String::from("Alice");
    nome.push_str(" Silva");
    ```

#### Compostos
- Tuplas
    ```rust
    let tupla: (i32, f64, char) = (42, 3.14, 'x');
    ```
- Arrays
    ```rust
    let a: [i32; 3] = [1, 2, 3];
    ```
- Vetores
    ```rust
    let v: Vec<i32> = vec![1, 2, 3];
    ```

#### Tipos personalizados
- Structs
    ```rust
    struct Pessoa {
        nome: String,
        idade: u8,
    }
    ```
- Enums
    ```rust
    enum Resultado {
        Sucesso,
        Erro(String),
    }
    ```

### Armazenamento e utilização das variáveis

Primeiramente queria dizer que as variáveis em rust são muito egoístas — elas gostam de deixar tudo claro, principalmente em relação à posse (ownership) e tempo de vida.

Em Rust, o armazenamento das variáveis na memória ocorre principalmente em três regiões: stack (pilha), heap, e área estática.

**Stack** (pilha): é usada para valores cujo tamanho é conhecido em tempo de compilação e que não precisam de alocação dinâmica. Exemplos:

- Números (i32, f64 etc.)

- Tuplas e arrays com tamanho fixo

- Structs compostas apenas por tipos com tamanho fixo

**Heap**: usada para dados com tamanho variável ou que precisam viver além do escopo onde foram criados. A alocação é feita dinamicamente, e o gerenciamento de memória ocorre via ownership e borrowing (sem garbage collector). Exemplos:

- String (alocada dinamicamente)

- Vec<T> (vetores de tamanho dinâmico)

- Tipos como Box<T> e Rc<T>

**Área estática** (static): usada para variáveis globais de tempo de vida 'static, ou seja, que vivem por toda a duração do programa. Variáveis marcadas como static são armazenadas nessa área, assim como literais de string.

```rust
static PI: f64 = 3.1415; // armazenado na área estática

fn main() {
    let x = 42; // empilhado (stack)
    let s = String::from("Olá"); // parte dos dados está no heap
}
```

Rust te obriga a lidar com esses detalhes para evitar vazamentos de memória e erros de concorrência. Por isso, cada variável tem um dono, e você precisa deixá-lo claro no código.

#### Cause briga entre variáveis:

Se você definir uma variável de tamanho variável, e atribuir o valor dela a outra variável, a variável original vai ficar muito zangada.

```rust
let item_de_joao = "bicicleta"
let item_de_maria = item_de_joao

println!("{}", item_de_joao); // ❌ Erro: valor movido
```

É basicamente isso, se uma pessoa tem uma bicicleta e dá essa bicicleta para outra pessoa, a pessoa inicial não tem mais uma bicicleta.

~~rust é bizarro~~
