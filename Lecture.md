# AMBI - Organisatorisches

Einmalige Verschiebung 27. 28. April -> Mittwoch?

Mi 14-16 Uhr ab nächster Woche -> Blätter alle 2 Wochen

Beantragung auf Verlängerung der Auflagen - Prüfungsamt Informatik

---------------

Mündliche Prüfungen - 14 Tage vorher anmelden

Vorlesungsunterlagen
User: AMBI
PW: AMBIzwanzigzwanzig

---------------

# Terminology & Definitionen

alphabet `Sigma` consist of letters

* find and replace

for us `Sigma_(DNA) = {G,C,T,A}`

`d = |Sigma| = 4` Länge des Alphabets

Reihenfolge 5' -> 3'

FASTA format

Aminosäurenfolge => Primärstruktur

----

## Definitionen

`Sigma^**`: Menga aller endlicher Zeichenketten 

`epsilon`: empty string

`xy`: concat of two string

-- < is a blocky square bracket

`w[x` if x = wy and y e of `Sigma^**`: prefix of a string x  `|w| <= |x|` "x is prefix of w"

`w]x` "x suffix of w"

+ umkehrung

-------------

### Lemma 1

a) if |x| <= |y| => x]y (symmetrisch) => if |x| >= |y| => y]x

b) if |x| = |y| => x = y

# String-Matching

Algorithmen eine Einführung/Introduction to Algorithms

Wir suchen in einem [T]]ext von Länge *n* ein [P]attern *m*. |P| < |T|

Verschiebung *shift* s ist der Index an dem unser Match beginnt.

=> Find all valid shifts in which P occurs in T.

Unterschied zu *alignment* -> Gaps

-----

O Notation => Obere Grenze/Schranke der Laufzeit

-----

Worst case - match everywhere

# Rabin-Karp algorithmus

Hashing function for [P]attern and [T]ext.

Text enthält Teilsequenzen der Länge m => Alle *n-m+1* Teilsequenzen müssen geprüft werden.

`T[s+1..s+m]` => `t_s`
`P[1..m]` => `p`

Berechnen aller Hash werde im in [T]. // Übersetzung des Text in von einem Alphabet `Sigma` nach `Sigma'` als Zahlenwerte.

Selber Hashwert => immernoch verschieden.

Horner's rule

`p = Hash(P) = P[m] + d(P[m-1]) + d(P[m-2]) + ...`

Nur einmal den Hashwert in O(m) berechnen => jeder weiterer Hashwert in Konstanter Zeit

# Finite automaton

# Knuth-Morris-Pratt algorithm

# Boyer-Moore algorithm