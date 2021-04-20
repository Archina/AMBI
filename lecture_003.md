Organisatorisches

\6. Mai 13:30

Neuer Foliensatz

# AMBI

## Transition function - Übergangsfunktion - Endlicher Automat

`T[i]`

state `phi(T_i)`

Tabelle für die Berechnungs der Übergangsfunktion

veritcal states zB 0-7
horizontal input alphabet

matches hervorgehoben

**prefix `T_i`** wird betrachtet

`delta(q, T[i])`:= Übergangsfunktion

accept accepting state

Running-time `Theta(n)`

## Suffix Funktion

Pattern matching system das für **alle** folgenden relevant ist.

`sigma` := suffix function - Ein exactes Pattern `P`

`Sigma^ast = {0,1,..,m}`

x Zeichenkette

`sigma(x)` => die Länge des längstes Prefixes von `P` das auch suffix von x ist.

`sigma(x) = max{k: P_k sqsupset x}`

Pattern **ab** => a | ab

Pattern = **ab** & `sigma(`cca**ab**`)`=2

=> `P` der Länge `m` UND `sigma(x) = m` => `P sqsupset x`

------------------

if `x sqsupset y` then `sigma(x) <= sigma(y)`

---------

`P[1..m]`:

state set `Q = {0,1,..,m}`, `q_0 = 0`, state `m` only accepting

transtion function `delta` for any state `q` and char `a` => `delta(q,a) = sigma(P_q a)`

--------------

Most recent read characters of `T` `T[i]`

substring of `T` ending in `T[i]` that matches prefix `P_j` => prefix `P_j` must be suffix of `T[i]`

---------------------

`T_i` Teilsequence bis zum Teil `i`.

state `q, P_q sqsubset T_i` `q = sigma(T_i)`

`phi(T_i)` and `sigma(T_i)` both equal to `q` => maintains invariant `phi(T_i) = sigma(T_i)`

`P_q b` = `P_5 b`

### Corretness prof

`T[1..n]`

`sigma(T_i)` after it reads the letter `T[i]`, because `sigma(T_i) = m, iff P sqsubset T_i` -> automated accepting in `m` iff `P` has just been scanned

#### Lemma 2 - Suffix function inequality

If automaton is in state `q` - reads next char `T[i+1] = a` => transition P that is a suffix of `T_i a` state is `sigma(T_i a)`

Cause `P_q` is longest prefix of `P` (by definition) that is suffix of `T_i`, the longest prefix of `P` that is a suffix of `T_i a` is not only `sigma(T_i a)` but also `sigma(P_q a)`

Thus automat state `q` => transition `a` ....????

------------------

`sigma(xa) <= sigma(x)+1`

`r = sigma(xa)`

if `r=0: r<= sigma(x)+1`

if `r> 0: P_r sqsubset xa => P_(r-1) sqsubset x => r-1 <= sigma(x)`

since `sigma(x)` is largest `k` such that `P_k subset x`

#### Lemma 3 = Suffix function recursion

string x and char a => if `q = sigma(x)` then `sigma(xa) = sigma(P_q a)`

-------------

if `r = sigma(xa)` then `r<=q+1`

since `P_q a sqsubset xa`, `P_r subset xa` und `|P_r| <= |P_q a|`

=> `r <= sigma(P_q a)` i.e: `sigma(xa) <= sigma(P_q a)`

It also holds `sigma(P_q a) <= sigma(xa)`, since `P_q a sqsubset xa` => `sigma(xa) = sigma(P_q a)`

### Invarianten zeigen - Characterization

`phi` is ein Endzustandsfunktion von einem String matching Automaten gegeben eines Pattern `P` and `T[1..n]`

`phi(T_i) = sigma(T_i)`

1. Induction i = 0

    `T_0 = epislon` => `phi (T_0) = sigma(T_0) = 0`

2. `phi(T_i) = sigma(T_0)` => `phi(T_(i+1)) = sigma(T_(i+1))`
    `phi(T_(I=1)) = sigma(T_i a)`
    = `delta(phi(T_i), a)`
    = `delta(q, a)`
    = `sigma(P_a a)`
    = `sigma(T_i a)`
    = `sigma(T_(i+1))`

### Transitions function

Alle Zustände q

    for each `a ele Sigma`

        do `k <- min(m+1, q+2)` // starts with the largest conceivable value of k such that `P_k sqsubset P_q a`

        k <- k-1

        until `P_k sqsubset P_q a`

        `delta(q,a) <- k`

### Laufzeit

Preprocessing

`O(m^3|Sigma|)` - `|mSigma|`

- can be improved to `O(m|Sigma|)`

Matching

`Theta(n)`

## Knuth-Morris-Pratt

Jumping by more because of failure