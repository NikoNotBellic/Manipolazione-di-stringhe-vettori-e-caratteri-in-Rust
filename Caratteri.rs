// Questo programma elenca le istruzioni per la manipolazione dei singoli caratteri in Rust

fn main()
{
    // Inizializazione di un singolo carattere (_a è una variabile di tipo char contenente il carattere 'a')
    let _a: char = 'a';



    // Carattere unicode
    let _b: char = '🥹';



    // Controllare se il carattere è alfabetico, ritorna un valore booleano in base al s'è alfabetico, ritorna (true) altrimenti (false), _q: bool -> true
    let _kk: char = 'a';
    let _q: bool = _kk.is_alphabetic();



    // Controllare se il carattere è numerico, ritorna un valore booleano in base al s'è numerico, ritorna (true) altrimenti (false), _p: bool -> true
    let _d: char = '1';
    let _p: bool = _d.is_numeric();



    // Controllare se il carattere è minuscolo, ritorna un valore booleano in base al s'è minuscolo, ritorna (true) altrimenti (false), _z: bool -> true
    let _la: char = 'a';
    let _z: bool = _la.is_lowercase();



    // Controllare se il carattere è MAIUSCOLO, ritorna un valore booleano in base al s'è MAIUSCOLO, ritorna (true) altrimenti (false), _ac: bool -> true
    let _ui: char = 'A';
    let _ac: bool = _ui.is_uppercase();



    // Controllare se il carattere è uno spazio bianco, ritorna un valore booleano in base al s'è uno spazio bianco, ritorna (true) altrimenti (false), _nn: bool -> true
    let _ad: char = ' ';
    let _nn: bool = _ad.is_whitespace();



    // Convertire un carattere in un valore appartenente alla codifica ascii, _hs -> 97 (nella codifica ascii equivale ad 'a')
    let _pd: char = 'a';
    let _hs: u8 = _pd as u8;


    // Controllare se un carattere appartiene alla codifica ascii, ritorna un valore booleano (true) in base al s'appartiene all'insieme dei caratteri ascii, altrimenti (false), _op: bool -> true
    let _lo: char = 'a';
    let _op: bool = _lo.is_ascii();



    // Controllare se un carattere appartiene alla codifica ascii e controlla anche s'è alfabetico, ritorna un valore booleano (true) in base al s'appartiene all'insieme dei caratteri ascii ed è anche alfabetico, altrimenti (false), _zz: bool -> true
    let _zz: char = 'x';
    let _px: bool = _zz.is_ascii_alphabetic();



    // Controlla se un carattere è numericamente incluso tra 0 e 9 (appartenente alla codifica ascii), ritorna un valore booleano (true) s'appartiene all'insieme dei numeri compresi tra 0 e 9 E che appartengono alla codifica ascii, altrimenti restituisce (false), _aq: bool -> true
    let _aq: char = '2';
    let _sz: bool = _aq.is_ascii_digit();



    // Controlla se un carattere è un segno di puntegiatura appartenente alla codifica ascii, se lo è ritorna un valore booleano (true), altrimenti (false), _lq: bool -> true
    let _ww: char = ',';
    let _lq: bool = _ww.is_ascii_punctuation();



    // Controlla se un carattere è uno spazio appartente alla codifica ascii, ritorna due possibili valori boolean, true s'è uno spazio appartente alla codifica ascii altrimenti restituice false
    let _oo: char = ' ';
    let _hm: char = '\n';

    let _s: bool = _oo.is_ascii_whitespace();   // _s: bool -> true
    let _d: bool = _hm.is_ascii_whitespace();   // _d: bool -> true



    // Controlla se un carattere appartiene all'insieme dei caratteri ascii e MAIUSCOLO, ritorna due possibili valori booleani, true se il caraterre appartiene all'appena citato insieme, altrimenti false
    let _hc: char = 'A';
    let _ja: char = 'a';

    let _dl: bool = _hc.is_ascii_uppercase();   // _dl: bool -> true
    let _zl: bool = _ja.is_ascii_uppercase();   // _zl: bool -> false



    // Controlla se un carattere appartiene all'insieme dei caratteri ascii e minuscoli, ritorna due possibili valori booleani, true se il caraterre appartiene all'appena citato insieme, altrimenti false
    let _dk: char = 'G';
    let _am: char = 'g';

    let _nn: bool = _dk.is_ascii_lowercase();   // _nn: bool -> false
    let _tt: bool = _am.is_ascii_lowercase();   // _tt: bool -> true



    // Ottenere gli indici di ogni singolo carattere, restituisce un'iteratore sui singoli caratteri di uno slice &str
    let _sk = "ciao";

        // Usato per iterare nei cicli, x: usize -> inidce del carattere, y: char -> carattere
        for (_x, _y) in _sk.char_indices()
        {
            println!("Indice: {_x}, carattere: {_y}");
        }



    // Convertire un carattere minuscolo in MAIUSCOLO, (PRIMA di to_uppercase() ) _qw -> 'a', (DOPO to_uppercase() ) _qw -> 'A'
    let _qw: char = 'a';
    let _qw = _qw.to_uppercase();



    // Convertire un carattere MAIUSCOLO in minuscolo, (PRIMA di to_uppercase() ) _sw -> 'A', (DOPO to_uppercase() ) _sw -> 'a'
    let _sw: char = 'a';
    let _sw = _sw.to_lowercase();



    // Cercare un carattere in una stringa, se trova il carattere restituisce true, altrimenti false
    let _lu: String = String::from("ciao");
    let _ko: bool = _lu.contains('c');
}