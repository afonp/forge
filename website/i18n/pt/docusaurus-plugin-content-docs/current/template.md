---
sidebar_position: 4
---

# referencia do template c++

o template e automaticamente copiado para cada novo exercicio como `solution.cpp`. podes personaliza-lo editando `~/.cp/templates/template.cpp`.

## pragmas do compilador

```cpp
#pragma GCC optimize("O2,unroll-loops")
#pragma GCC target("avx2,bmi,bmi2,popcnt")
```

## type aliases

| alias | tipo |
|-------|------|
| `ll`  | `long long` |
| `ull` | `unsigned long long` |
| `ld`  | `long double` |
| `pii` | `pair<int, int>` |
| `pll` | `pair<ll, ll>` |
| `vi`  | `vector<int>` |
| `vll` | `vector<ll>` |
| `vvi` | `vector<vi>` |

## constantes

| nome   | valor |
|--------|-------|
| `inf`  | `0x3f3f3f3f` |
| `linf` | `0x3f3f3f3f3f3f3f3f` |
| `eps`  | `1e-9` |
| `mod`  | `1e9 + 7` |
| `mod2` | `998244353` |
| `pi`   | `acos(-1.0)` |

## macros

| macro | expande para |
|-------|-------------|
| `all(x)` | `(x).begin(), (x).end()` |
| `rall(x)` | `(x).rbegin(), (x).rend()` |
| `sz(x)` | `(int)(x).size()` |
| `pb` | `push_back` |
| `eb` | `emplace_back` |
| `mp` | `make_pair` |
| `fi` | `first` |
| `se` | `second` |
| `rep(i,a,b)` | `for (int i = a; i < b; i++)` |
| `per(i,a,b)` | `for (int i = a; i >= b; i--)` |
| `each(x,v)` | `for (auto& x : v)` |

## macros de debug

apenas ativas quando compiladas com `-DLOCAL` (que o makefile faz por defeito):

```cpp
dbg(x);       // imprime: x = <valor> para stderr
dbgv(v);      // imprime: v = [1, 2, 3] para stderr
```

## helpers de i/o

```cpp
fast_io();          // ios::sync_with_stdio(false); cin.tie(nullptr);
read(a, b, c);     // cin >> a >> b >> c;
print(a, b, c);    // cout << a << ' ' << b << ' ' << c << '\n';
printvec(v, " ");  // imprime elementos do vetor separados pelo separador dado
```

## matematica

```cpp
ll g = gcd(a, b);
ll l = lcm(a, b);
ll p = power(base, exp, mod);  // exponenciacao modular
ll inv = modinv(a, mod);       // inverso modular (mod tem de ser primo)
```

## graph

```cpp
graph g(n);                        // cria um grafo com n vertices
g.add_edge(u, v, w, directed);    // adiciona uma aresta
vector<ll> dist = g.dijkstra(src); // caminhos mais curtos a partir de src
vi dist = g.bfs(src);              // distancias bfs a partir de src
```

## dsu (disjoint set union)

```cpp
dsu d(n);
bool merged = d.unite(a, b);
bool same = d.connected(a, b);
int comp = d.components;
```

## segment tree

0-indexed, point update, range sum query:

```cpp
segtree st(n);
st.update(pos, val);
ll sum = st.query(l, r);
```

## fenwick tree (bit)

0-indexed, point update, prefix/range sum query:

```cpp
bit ft(n);
ft.update(i, delta);
ll sum = ft.prefix(i);
ll sum = ft.query(l, r);
```

## algoritmos de strings

```cpp
vi fail = kmp_fail(pattern);
vi z = z_function(s);
```

## estrutura do main

```cpp
void solve() {
    // a tua solucao aqui
}

int main() {
    fast_io();
    int t = 1;
    // cin >> t;  // descomentar para multiplos test cases
    while (t--) solve();
    return 0;
}
```
