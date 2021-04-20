# AMBI - Laufzeit von Algorithmen

Größten Größenordnung der Eingabedaten

`T(n) = an^2 + bn + c`

`n^2 > n`

Konstanten machinenabhängig und eher uninteressant.

=> Konzentration auf den **Worst-Case**

## O-Notation (Big O)

Funktion `g(n)` mit `O(g(n))` obere schranke der Laufzeit von n => Merne von Funktionen mit Eigenschaft:

 * `O(g(n))` = gegeben eine `f(n)` => es gibt ein `c` und `n_0` so das `c*g(n')` > `f(n')` für die gilt das `n' > n_0`

**Obere Schranke**

-----------

Insert-Sort `O(n^2)`

## Omega-Notiation

**untere Schranke** - Mindestlaufzeit

## Teta-Notation (O-)

**untere + obere Schranke**

`c_1` & `c_2` - eine konstante obere eine untere Schranke

# Rabing Karp -> Fortsetzung

* Alphabet numerisch verschlüsseln `a,b,c,d` => `0,1,2,3`

## **Horner Regel**

`m` länge des Patterns

letzte + d(m-1) + d(m-2)

`t_(s+1)` = `d(t_s - d^(m-1) T[s+1]) + T[s+m+1]`

Diskretly

31452 => `t_s = 3145`, `m = 5`, `T[s+m+1] = 2`

==> `10(31415 - 10^5 * 3) + 2`

O(m + n)

### Vergleich von Hash values

modulo operator - should be a big prime number

**Valid match** vs **spurious hit**

### Performance

Preprocessing `Teta(2m) = Teta(m)`

Is modulo q big in comparison to `d^m` => `q >= m` then `O(m+n)` becomes `O(n)` for m << n

## Shortsighted man

`Single characters`

Looks at each character only once

Endlicher Automat - Anzahl der matches

`M` endlicher Automat = `(Q, q_0, A, Sigma, delta)` with

Q as finite set of states
q_0 start state
A subset of Q as a set of accepting states
Sigma as finite input alphabet
delta as function of QxSigma -> Q übergangsfunktionen

### The final-state function

accpetiers string iff letzte zeichen im akzeptieren Zustand