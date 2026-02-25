#pragma GCC optimize("O2,unroll-loops")
#pragma GCC target("avx2,bmi,bmi2,popcnt")

#include <bits/stdc++.h>
using namespace std;

// type aliases
using ll  = long long;
using ull = unsigned long long;
using ld  = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using vi  = vector<int>;
using vll = vector<ll>;
using vvi = vector<vi>;

// constants
const int inf    = 0x3f3f3f3f;
const ll  linf   = 0x3f3f3f3f3f3f3f3fLL;
const ld  eps    = 1e-9;
const int mod    = 1e9 + 7;
const int mod2   = 998244353;
const ld  pi     = acos((ld)-1.0);

// macros
#define all(x)      (x).begin(), (x).end()
#define rall(x)     (x).rbegin(), (x).rend()
#define sz(x)       (int)(x).size()
#define pb          push_back
#define eb          emplace_back
#define mp          make_pair
#define fi          first
#define se          second
#define rep(i,a,b)  for (int i = (a); i < (b); i++)
#define per(i,a,b)  for (int i = (a); i >= (b); i--)
#define each(x,v)   for (auto& x : v)

// debug macros (stripped on judges)
#ifdef LOCAL
#define dbg(x) cerr << #x << " = " << (x) << endl
#define dbgv(v) { cerr << #v << " = ["; for (int i = 0; i < sz(v); i++) cerr << (i ? ", " : "") << v[i]; cerr << "]" << endl; }
#else
#define dbg(x)
#define dbgv(v)
#endif

// i/o helpers
void fast_io() { ios::sync_with_stdio(false); cin.tie(nullptr); }

template<typename... T>
void read(T&... args) { ((cin >> args), ...); }

template<typename... T>
void print(T&&... args) { ((cout << args << ' '), ...); cout << '\n'; }

template<typename T>
void printvec(const vector<T>& v, const string& sep = " ") {
    for (int i = 0; i < sz(v); i++) cout << (i ? sep : "") << v[i];
    cout << '\n';
}

// math
ll gcd(ll a, ll b) { return b ? gcd(b, a % b) : a; }
ll lcm(ll a, ll b) { return a / gcd(a, b) * b; }

ll power(ll base, ll exp, ll mod) {
    ll res = 1; base %= mod;
    while (exp > 0) {
        if (exp & 1) res = res * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return res;
}

ll modinv(ll a, ll mod) { return power(a, mod - 2, mod); }

// graph
struct graph {
    int n;
    vector<vector<pair<int, ll>>> adj;

    graph(int n) : n(n), adj(n) {}

    void add_edge(int u, int v, ll w = 1, bool directed = false) {
        adj[u].pb({v, w});
        if (!directed) adj[v].pb({u, w});
    }

    vector<ll> dijkstra(int src) {
        vector<ll> dist(n, linf);
        priority_queue<pll, vector<pll>, greater<pll>> pq;
        dist[src] = 0;
        pq.push({0, src});
        while (!pq.empty()) {
            auto [d, u] = pq.top(); pq.pop();
            if (d > dist[u]) continue;
            for (auto [v, w] : adj[u]) {
                if (dist[u] + w < dist[v]) {
                    dist[v] = dist[u] + w;
                    pq.push({dist[v], v});
                }
            }
        }
        return dist;
    }

    vi bfs(int src) {
        vi dist(n, -1);
        queue<int> q;
        dist[src] = 0;
        q.push(src);
        while (!q.empty()) {
            int u = q.front(); q.pop();
            for (auto [v, w] : adj[u]) {
                if (dist[v] == -1) {
                    dist[v] = dist[u] + 1;
                    q.push(v);
                }
            }
        }
        return dist;
    }
};

// disjoint set union
struct dsu {
    vi parent, rank_;
    int components;

    dsu(int n) : parent(n), rank_(n, 0), components(n) {
        iota(all(parent), 0);
    }

    int find(int x) {
        return parent[x] == x ? x : parent[x] = find(parent[x]);
    }

    bool unite(int a, int b) {
        a = find(a); b = find(b);
        if (a == b) return false;
        if (rank_[a] < rank_[b]) swap(a, b);
        parent[b] = a;
        if (rank_[a] == rank_[b]) rank_[a]++;
        components--;
        return true;
    }

    bool connected(int a, int b) { return find(a) == find(b); }
};

// segment tree (0-indexed, range sum, point update)
struct segtree {
    int n;
    vll tree;

    segtree(int n) : n(n), tree(4 * n, 0) {}

    void update(int pos, ll val, int node = 1, int lo = 0, int hi = -1) {
        if (hi == -1) hi = n - 1;
        if (lo == hi) { tree[node] = val; return; }
        int mid = (lo + hi) / 2;
        if (pos <= mid) update(pos, val, 2 * node, lo, mid);
        else update(pos, val, 2 * node + 1, mid + 1, hi);
        tree[node] = tree[2 * node] + tree[2 * node + 1];
    }

    ll query(int l, int r, int node = 1, int lo = 0, int hi = -1) {
        if (hi == -1) hi = n - 1;
        if (r < lo || hi < l) return 0;
        if (l <= lo && hi <= r) return tree[node];
        int mid = (lo + hi) / 2;
        return query(l, r, 2 * node, lo, mid) + query(l, r, 2 * node + 1, mid + 1, hi);
    }
};

// fenwick tree / binary indexed tree (0-indexed)
struct bit {
    int n;
    vll tree;

    bit(int n) : n(n), tree(n + 1, 0) {}

    void update(int i, ll delta) {
        for (i++; i <= n; i += i & (-i)) tree[i] += delta;
    }

    ll prefix(int i) {
        ll s = 0;
        for (i++; i > 0; i -= i & (-i)) s += tree[i];
        return s;
    }

    ll query(int l, int r) {
        return prefix(r) - (l ? prefix(l - 1) : 0);
    }
};

// string algorithms
vi kmp_fail(const string& p) {
    int m = sz(p);
    vi fail(m, 0);
    for (int i = 1; i < m; i++) {
        int j = fail[i - 1];
        while (j > 0 && p[i] != p[j]) j = fail[j - 1];
        if (p[i] == p[j]) j++;
        fail[i] = j;
    }
    return fail;
}

vi z_function(const string& s) {
    int n = sz(s);
    vi z(n, 0);
    int l = 0, r = 0;
    for (int i = 1; i < n; i++) {
        if (i < r) z[i] = min(r - i, z[i - l]);
        while (i + z[i] < n && s[z[i]] == s[i + z[i]]) z[i]++;
        if (i + z[i] > r) { l = i; r = i + z[i]; }
    }
    return z;
}

void solve() {

}

int main() {
    fast_io();
    int t = 1;
    // cin >> t;
    while (t--) solve();
    return 0;
}
