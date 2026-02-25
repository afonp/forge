---
sidebar_position: 4
---

# c++ template reference

the template is automatically copied into every new exercise as `solution.cpp`. you can customize it by editing `~/.cp/templates/template.cpp`.

## compiler pragmas

```cpp
#pragma GCC optimize("O2,unroll-loops")
#pragma GCC target("avx2,bmi,bmi2,popcnt")
```

## type aliases

| alias | type |
|-------|------|
| `ll`  | `long long` |
| `ull` | `unsigned long long` |
| `ld`  | `long double` |
| `pii` | `pair<int, int>` |
| `pll` | `pair<ll, ll>` |
| `vi`  | `vector<int>` |
| `vll` | `vector<ll>` |
| `vvi` | `vector<vi>` |

## constants

| name   | value |
|--------|-------|
| `inf`  | `0x3f3f3f3f` |
| `linf` | `0x3f3f3f3f3f3f3f3f` |
| `eps`  | `1e-9` |
| `mod`  | `1e9 + 7` |
| `mod2` | `998244353` |
| `pi`   | `acos(-1.0)` |

## macros

| macro | expands to |
|-------|-----------|
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

## debug macros

only active when compiled with `-DLOCAL` (which the makefile does by default):

```cpp
dbg(x);       // prints: x = <value> to stderr
dbgv(v);      // prints: v = [1, 2, 3] to stderr
```

these are stripped when submitted to online judges.

## i/o helpers

```cpp
fast_io();          // ios::sync_with_stdio(false); cin.tie(nullptr);
read(a, b, c);     // cin >> a >> b >> c;
print(a, b, c);    // cout << a << ' ' << b << ' ' << c << '\n';
printvec(v, " ");  // prints vector elements separated by the given separator
```

## math

```cpp
ll g = gcd(a, b);
ll l = lcm(a, b);
ll p = power(base, exp, mod);  // modular exponentiation
ll inv = modinv(a, mod);       // modular inverse (mod must be prime)
```

## graph

```cpp
graph g(n);                        // create a graph with n vertices
g.add_edge(u, v, w, directed);    // add an edge (w defaults to 1, directed defaults to false)
vector<ll> dist = g.dijkstra(src); // shortest paths from src
vi dist = g.bfs(src);              // bfs distances from src (-1 if unreachable)
```

## dsu (disjoint set union)

```cpp
dsu d(n);                  // create dsu with n elements
bool merged = d.unite(a, b);  // unite sets, returns true if they were different
bool same = d.connected(a, b); // check if a and b are in the same set
int comp = d.components;       // number of connected components
```

## segment tree

0-indexed, point update, range sum query:

```cpp
segtree st(n);
st.update(pos, val);          // set position pos to val
ll sum = st.query(l, r);      // sum of range [l, r]
```

## fenwick tree (bit)

0-indexed, point update, prefix/range sum query:

```cpp
bit ft(n);
ft.update(i, delta);          // add delta to position i
ll sum = ft.prefix(i);        // sum of [0, i]
ll sum = ft.query(l, r);      // sum of [l, r]
```

## string algorithms

```cpp
vi fail = kmp_fail(pattern);   // kmp failure function
vi z = z_function(s);          // z-function
```

## main structure

```cpp
void solve() {
    // your solution here
}

int main() {
    fast_io();
    int t = 1;
    // cin >> t;  // uncomment for multiple test cases
    while (t--) solve();
    return 0;
}
```
