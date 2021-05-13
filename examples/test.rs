use std::usize;

use hashbrown::HashMap;

fn map_alpha(input: &[char]) -> HashMap<char, u8> {
    let mut alphabet = input.to_vec();
    alphabet.sort_unstable();
    alphabet.dedup(); 
    if alphabet.len() > u8::MAX as usize { panic!("Too Hecking LONG"); }
    alphabet.iter().enumerate().map(|(i, c)| (*c, i as u8)).collect()
}

fn translate_with_alpha<T>(input: &[char], alpha: &HashMap<char, T>) -> Vec<T> where T: Copy {
    input.iter().map(|c| alpha[c]).collect()
}

fn str_match<T>(start: usize, text: &[T], pattern: &[T]) -> bool where T: Eq {
    text[start..start+pattern.len()] == pattern[0..pattern.len()]
}

pub fn naive_match<T,P>(text: T, pattern: T) -> Vec<usize> where T: AsRef<[P]>, P: Eq {
    let mut matches= vec![];
    let text = text.as_ref();
    let pattern = pattern.as_ref();
    let t_count = text.len();
    let p_count = pattern.len();
    if p_count > t_count { panic!("pattern is longer than text"); }
    for t_i in 0..t_count-p_count+1 {
        if str_match(
            t_i,
            text,
            pattern
        ) {
            matches.push(t_i);
        }
    }
    matches
}

pub struct Automat{
    accept: usize,
    transition_functions: HashMap<usize, usize>,
}

impl Automat{
    fn idx_char(idx: usize, c: &u8) -> usize {
        (idx << 8) + *c as usize
    }

    pub fn new(pattern: &[u8]) -> Self{
        let mut alphabet = pattern.to_vec();
        alphabet.sort_unstable();
        alphabet.dedup();
        let mut transition_functions = HashMap::with_capacity(alphabet.len() * pattern.len()+1);
        if pattern.len() > u16::MAX as usize { panic!("Pattern too long") }
        for idx in 0..=pattern.len() {
            for a in &alphabet {
                if idx < pattern.len() && pattern[idx] == *a {
                    transition_functions.insert(
                        Automat::idx_char(idx, a),
                        idx+1
                    );
                } else {
                    let slice = [&pattern[0..idx], &[*a]].concat();
                    let pos = Automat::sigma_suffix(slice.as_slice());
                    if pos != 0 {
                        transition_functions.insert(
                            Automat::idx_char(idx, a),
                            pos
                        );
                    }
                }
            }
        }
        Self{
            accept: pattern.len(),
            transition_functions
        }
    }

    /// Return the length of the longest prefix that is also a suffix
    fn sigma_suffix(x: &[u8]) -> usize{
        for k in 1..x.len() {
            let prefix = &x[0..x.len()-k];
            let suffix = &x[k..x.len()];
            if prefix == suffix {
                return prefix.len();
            }
        }
        0
    }

    pub fn match_text(&self, text: &[u8]) -> Vec<usize> {
        let mut matches = vec![];
        let mut start = 0;
        for (idx, char) in text.iter().enumerate(){
            start = *self.transition_functions.get(&Automat::idx_char(start, char)).unwrap_or(&0);
            if start == self.accept {
                matches.push(idx-self.accept+1);
            }
        }
        matches
    }
}

fn main() {

    let poem = r#"
    >Der Zauberlehrling. Johann Wolfgang Goethe
    Hat der alte Hexenmeister
    Sich doch einmal wegbegeben!
    Und nun sollen seine Geister
    Auch nach meinem Willen leben.
    Seine Wort und Werke
    Merkt ich und den Brauch,
    Und mit Geistesstaerke
    Tu ich Wunder auch.

    Walle! walle
    Manche Strecke,
    Dass zum Zwecke,
    Wasser fliesse�
    Und mit reichem, vollem Schwalle
    Zu dem Bade sich ergiesse.
Und nun komm, du alter Besen,
Nimm die schlechten Lumpenhuellen!
Bist schon lange Knecht gewesen:
Nun erfuelle meinen Willen!
Auf zwei Beinen stehe,
Oben sei ein Kopf,
Eile nun und gehe
Mit dem Wassertopf!

    Walle! walle
    Manche Strecke,
    Dass zum Zwecke,
    Wasser fliesse�
    Und mit reichem, vollem Schwalle
    Zu dem Bade sich ergiesse.
Seht, er laeuft zum Ufer nieder!
Wahrlich! ist schon an dem Flusse,
Und mit Blitzesschnelle wieder
Ist er hier mit raschem Gusse.
Schon zum zweiten Male!
Wie das Becken schwillt!
Wie sich jede Schale
Voll mit Wasser fllt!

    Stehe! stehe!
    Denn wir haben
    Deiner Gaben
    Vollgemessen! -
    Ach, ich merk es! Wehe! wehe!
    Hab ich doch das Wort vergessen!
Ach, das Wort, worauf am Ende
Er das wird, was er gewesen!
Ach, er laeuft und bringt behende!
Waerst du doch der alte Besen!
Immer neue Guesse
Bringt er schnell herein,
Ach, und hundert Fluesse
Stuerzen auf mich ein!

    Nein, nicht laenger
    Kann ichs lassen:
    Will ihn fassen!
    Das ist Tuecke!
    Ach, nun wird mir immer baenger!
    Welche Miene! welche Blicke!
O, du Ausgeburt der Hoelle!
Soll das ganze Haus ersaufen?
Seh ich ueber jede Schwelle
Doch schon Wasserstroeme laufen.
Ein verruchter Besen,
Der nicht hoeren will!
Stock, der du gewesen,
Steh doch ch schon Wasserstroeme ueber jede Schwch ueber jede Schwwie das wird, was er gewe das wird, was er gewech ueber jede Schwdll das ganze Haus ersauer still!

    Willst am Ende
    Gar nicht lassen?
    Will dich fassen,
    Will dich halten
    Und das alte Holz behende
    Mit dem scharfen Beile spalten!
Seht, da kommt er schleppend wieder!
Wie ich mich nur auf dich werfe,
Gleich, o Kobold, liegst du nieder;
Krachend trifft die glatte Schaerfe.
Wahrlich! brav getroffen!
Seht, er ist entzwei!
Und nun kann ich hoffen,
Und ich atme frei!
Bist schon lange Knecht gewesen:Bist Wahrlich! brav getroffen!Wahrlich! brav getroffen!schon lange Knecht gewesen:Bist schon langGleich, o Kobold, liegst du nieder;e Knecht gewesen:Bist schon lange Knecht gewesen:Bist schon langeWahrlich! brav getroffen! Knecht gewesen:Bist schon lange Knecht gewesen:Und sie laufen! Nass�und naesser
    Wehe! wehe!
    Beide Teile
    Stehn in Eile
    Schon als Knechte
    Voellig fertig in die Hoehe!
    Helft mir, ach! ihr hohen Maechte!
Und sie laufen! Nass�und naesser
Wirds im Saal und auf den Stufen:
Welch entsetzliches Gewaesser!
Herr und Meister, hoer mich rufen! -
Ach, da kommt der Meister!
Herr, die Not ist gross�
Die ich rief, die Geister,
Werd ich nun nicht los.

    "In die Ecke,
    Besen! Besen!
    Seids gewesen!
    Denn als Geister
    Ruft euch nur, zu seinem Zwecke,
Erst hervor der alte Meister."
"#;

    let poem: Vec<char> = poem.chars().collect();

    let alphabet = map_alpha(poem.as_slice());

    let poem_trans = translate_with_alpha(poem.as_slice(), &alphabet);

    let patterns: Vec<Vec<char>> = vec![
        "Besen".chars().collect(),
        "Wasserstroeme".chars().collect(),
        "Eimer".chars().collect(),
        "Steh doch ch schon Wasserstroeme ueber jede Schwch ueber jede Schwwie das wird,n als Geis was er gewe das wich entsetzliches Gewaesrd, was er gewech ueber jede Schwdll das ganze Haus ersauer stioch ch schon Wasserstroeme ueber jet euch nur, zu se Geis was er gewe das winem Zweckede Schwch ueber jede Schwwie das wird, was er gewe das wird, was er gewech ueber jede Schwdll das ganze Haus ersauer still!".chars().collect()
    ];

    let patterns_trans: Vec<Vec<_>> = patterns.iter().map(|p| translate_with_alpha(p.as_slice(), &alphabet)).collect();

    for pattern in patterns_trans {
        let now = std::time::Instant::now();
        naive_match(poem_trans.as_slice(), pattern.as_slice());
        let elapsed_naive = now.elapsed().as_micros();

        let a = Automat::new(pattern.as_slice());
        let now = std::time::Instant::now();
        a.match_text(poem_trans.as_slice());
        let elapsed_aut = now.elapsed().as_micros();

        println!("Naive: {}\nAutot: {}", elapsed_naive, elapsed_aut);
    }

    // println!("{:#?}", alphabet);
}