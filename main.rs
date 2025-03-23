// Questo programma mostra le istruzioni per manipolare le stringhe in Rust

use std::fmt::Write;

fn main()
{
    // Inizializazzione di una stringa vuota _x: String -> ""
    let _x: String = String::new();



    // Inizializazzione di una stringa con testo _y: String -> "Hi Dude"
    let _y: String = String::from("Hi Dude");



    // Conversione di un valore in una stringa _k: i32 -> 12, _s: String -> "12"
    let _k: i32 = 12;
    let _s: String = _k.to_string();



    // Lunghezza di una stringa _w: usize -> 4
    let _m: String = String::from("Dude");
    let _w: usize = _m.len();



    // Inizializazione della capacita di una stringa, _lz è una stringa con una capacita (MASSIMA) di 10 caratteri, NON FISSA, ma che può raggiungere durante l'esecuzione
    let mut _lz: String = String::with_capacity(10);



    // Capacita di una stringa, _pi: usize -> 10, restituisce lo spazio totale riservato nella stringa, len() invece restituisce la lunghezza corrente della stringa
    let _pi: usize = _lz.capacity();



    // Controllo delle stringhe vuote
    let _q: String = String::new();
    let _n: String = String::from("YOLOOOO");

        // Ritorna FALSE (0) se la stringa contiene del testo, altrimenti ritorna TRUE (1) se la stringa è vuota
        println!("{}", _q.is_empty());
        println!("{}", _n.is_empty());



    // Iteratore chars() -> restituisce uno a uno gli elemeti di una stringa
    let _o: String = String::from("XDDDDD");
    let _j = _o.chars();

        // Viene usato principalmente nei cicli, stampa uno ad uno gli elementi della stringa
        for c in _o.chars()
        {
            println!("{c}");
        }
            


    // Iteratore bytes() -> restituisce uno a uno gli elementi (ASCII) di una stringa
    let _w: String = String::from("Nikko Nikko");
    let _r = _w.bytes();

        // Viene usato principalmente nei cicli
        for x in _w.bytes()
        {
            println!("{x}");    // Stampa uno ad uno i valori ascii di una stringa
        }



    // Converte String in &str
    let _u: String = String::from("AMERICAAA");
    let _d: &str = _u.as_str();


    // Aggiunta di un singolo caraterre alla fine di una stringa, (PRIMA di .push() ) _g: String -> "You're so :", (DOPO .push() ) _g: String -> "You're so :)"
    let mut _g: String = String::from("You're so :");
    _g.push(')');



    // Concatenazione di una stringa ad uno slice (&str), (PRIMA di .push_str() ) _e: String -> "HI ", (DOPO .push_str() ) _e: String -> "Hi Dude :)"
    let mut _e: String = String::from("Hi ");
    _e.push_str("Dude :)");



    // Inserimento di un caraterre in un apposito indice, , (PRIMA di .push() ) _g: String -> "You're so :", (DOPO di .push() ) _g: String -> "You're so :)"
    let mut _t: String = String::from(";)");
    _t.insert(0, ':');



    // Inserire un pezzo di stringa in una stringa da uncerto inidce -> (PRIMA di insert_str() ) "hi  dude", (DOPO insert_str() ) "hi nahh dude"
    let mut _r: String = String::from("hi  dude");
    _r.insert_str(3, "nahh");



    // Rimozione di un carattere in una posizione specifica _b: String -> (PRIMA di remove() ) "nice soup!", (DOPO remove () ) "nice soup"
    let mut _b: String = String::from("nice soup!");
    _b.remove(9);



    // Sostiuisce un certo pezzo di stringa con un nuovo pezzo di stringa -> (PRIMA di replace() ) i love pizza, pizza is very nice!, (DOPO replace() ) i love life, life is very nice!
    let mut _k: String = String::from("i love pizza, pizza is very nice!");
    let _x = _k.replace("pizza", "life");



    // Sostituire un certo pezzo di stringa da un indice fino ad un'altro indice -> Cino is like pompidou
    let mut _a: String = String::from("Cino is like Cina");
    _a.replace_range(7..13, "pompidou");



    // Riduzione di una stringa (passiamo la nuova lunghezza della stringa) -> (PRIMA di truncate() ) "Hi to my code!", (DOPO truncate() ) "Hi to my" 
    let mut _string: String = String::from("Hi to my code!");
    _string.truncate(7);



    // Svuotamento di una stringa (cancella il contenuto della stringa) -> (PRIMA di clear() ) "nice, very nice work!", (DOPO clear() ) ""
    let mut _new: String = String::from("nice, very nice work!");
    _new.clear();



    // Mantiene i caraterri di una stringa che rispettano certi criteri -> janeisnotmydude (se il carattere è alfabetico (is_alphabetic) lo immete nella stringa)
    let mut _kq: String = String::from("jane is not my dude!");
    _kq.retain(|c: char| c.is_alphabetic());



    // Rimozione di un singolo caraterre -> (PRIMA di pop() ) "serial", (DOPO pop() ) "seria"
    let mut _sw: String = String::from("serial");
    _sw.pop();



    // Formatazione di due stringhe tramite funzione _hw: String -> Hi, my name is jen, i'm 25!
    let mut _lr: i32 = 25;
    let mut _df: &str = "jen";
    let _hw: String = format!("Hi, my name is {}, i'm {}!", _lr, _df);



    // Formatazione di due stringhe tramite procedura buffer: String -> Hi, my name is Jack, and i'm 31
    let mut _qw: i32 = 31;
    let mut _hj: &str = "Jack";
    let mut buffer: String = String::new();

    write!(&mut buffer, "Hi, my name is {}, and i'm {}", _hj, _qw).unwrap();



    // Conversione di una stringa da minuscolo a MAIUSOLO _lp: String -> AAA
    let mut _df: String = String::from("aaa");
    let _lp: String = _df.to_uppercase(); 



    // Conversione di una stringa da MAIUSCOLO a minuscolo _jq: String -> aaa
    let mut _lk: String = String::from("AAA");
    let _jq: String = _lk.to_lowercase();



    // Rimozione degli spazi all'inizio della stringa e alla fine della stringa _qn -> "     hey dude     ", _bj -> "hey dude"
    let mut _qn: String = String::from("     hey dude     ");
    let _bj: &str  = _qn.trim();



    // Rimozione degli spazi SOLO all'inizio di una stringa _qa: &str -> nice shoes!
    let mut _eh: String = String::from("           nice shoes!");
    let _qa: &str = _eh.trim_start();



    // Rimozione degli spazi SOLO alla fine di una stringa _xs: &str -> jane
    let mut _ll: String = String::from("jane     ");
    let _xs: &str = _ll.trim_end();



    // Cercare una sotto stringa in una stringa, restituisce due possibili valori -> TRUE se la sotto stringa è contenuta nella stringa _qq, FALSE se non è contenuta nella stringa _qq
    let mut _qq: String = String::from("A day, a fox jump into my garden");
    let _hz: bool = _qq.contains("fox");



    // Ricerca del primo indice di una sottostringa dall'inizio in una stringa, _q1 -> 12 (INIZIA A CERCARE LA SOTTO STRINGA DALL'INIZIO DELLA STRINGA)
    let mut _i: String = String::from("Into my new house, i have an other house!!");
    let _q1 = _i.find("house");



    // Ricerca dell'ultimo elemento di una sottostringa dalla fine di una stringa, _q2 -> 39 (INIZIA A CERCARE LA SOTTO STRINGA DALLA FINE DELLA STRINGA)
    let mut _i: String = String::from("Into my new house, i have an other house!!");
    let _q2 = _i.rfind("house");



    // Controllare se una stringa inizia con un determinato slice, restituisce un valore booleano TRUE se la stringa inizia con quel determinato slice, altrimenti restituisce FALSE
    let mut _jp: String = String::from("Hi from my String in Rust!");
    _jp.starts_with("Hi");



    // Controlla se una stringa finisce con un determinato slice, restituisce un valore booleano TRUE se la stringa finisce con quel determinato slice, altrimenti restituisce FALSE
    let mut _jp: String = String::from("Hi from my String in Rust!");
    _jp.ends_with("Rust!");



    // Contatore di quante volte uno slice viene usato in una stringa, matches() trova le slice nella stringa principale, count() tiene conto di quante volte viene invocata lo slice, _lp: usize -> 4
    let mut _sd: String = String::from("There's a day in a day with a day that wasn't a day, what a strange day!");
    let _lp = _sd.matches("day").count();



    // Restituisce tutti i primi indici di uno slice di una determinata stringa _pq -> 10, 19, 30, 52, 72
    let mut _sd: String = String::from("There's a day in a day with a day that wasn't a day, what a strange day!");
    let _pq = _sd.match_indices("day");
    

    // Ottenere uno slice da una stringa specificando da quale indice iniziamo a prelevare il nostro slice e da quale indice l'istruzione smette di prelevare lo slice _po Option<&str> -> "Hi from" altrimenti se gli indici forniti escono dalle dimmensioni della stringa restituira None
    let mut _op: String = String::from("Hi from my rusty computer!");
    let _po: Option<&str> = _op.get(0..7);



    // Divisore di una stringa in piccoli slice con un delimitatore _df: Vec<&str> -> ["Hi", "my name is jack", "i'm 19", "i live in Toronto", "and i have a dog"]
    let mut _kp: String = String::from("Hi, my name is jack, i'm 19, i live in Toronto, and i have a dog");
    let _df: Vec<&str> = _kp.split(',').collect();



    // Divisore di una stringa in un determinato numero di piccoli slice con un delimitaore _lw -> ["Hi", "my name is jack", "i'm 19", "i live in toronto and i have a dog"]
    _kp = String::from("Hi, my name is jack, i'm 19, i live in Toronto, and i have a dog");
    let _lw = _kp.splitn(4, ',');



    // Divisore di una stringa in piccoli slice tramite lo spazio vuoto " ", _qm -> ["Hi", "my", "name", "is", "jack", "i'm", "19", "i", "live", "in", "Toronto", "and", "i", "have", "a", "dog"]
    _kp = String::from("Hi my name is jack i'm 19 i live in Toronto and i have a dog");
    let _qm = _kp.split_whitespace();



    // Divisore di una stringa in piccoli slice tramite il newline (\n), _ox -> ["Hi from my", "program write in Rust", "i like Rust"]
    let mut _lz: String = String::from("Hi from my\n program write in Rust\n i like Rust");
    let _ox = _lz.lines();



    // Covertitore di una sequenza di byte (ascii) in careterri char, _mystr -> "ciao" (LA SEQUENZA DEVE RIENTRARE NELLA CODIFICA ASCII, SENNO' RESTITUISCE ERRORE), _mystr: String -> "ciao"
    let mut _aa: Vec<u8> = vec![99, 105, 97, 111];
    let _mystr: String = String::from_utf8(_aa).unwrap();



    // Convertitore di una sequenza di byte (ascii) in caraterri, con la differnza sostanziale da from_utf8() che se trova valori non utf-8 con dei caraterri di sostituzione, _invalid_string: String -> "ci�o"
    let mut _lx: &[u8; 4] = &[99, 105, 0x48, 111];
    let _invalid_string = String::from_utf8_lossy(_lx);
}