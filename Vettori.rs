// Questo programma spiega come manipolare i vettori

use std::ptr::eq;

fn main()
{
    // Inizializazione di un vettore di integer a 32 bit VUOTO
    let _vec1: Vec<i32> = Vec::new();



    // Inizializazione di un vettore di integer a 32 bit con dei valori
    let _vec2: Vec<i32> = vec![0, 1, 2, 3];



    // Inizializazzione della capacità di un vettore, (ATTENZIONE) la capacità iniziale non è permanente, può essere ridotto o aumentato durante la fase di execute
    let _vec3: Vec<i32> = Vec::with_capacity(10);



    // Capacità di una vettore, _c1: usize -> 10 restituisce lo spazio totale riservato nel vettore, len() invece restituisce la lunghezza corrente della vettore
    let _c1: usize = _vec3.capacity();

        // Output della capacità del vettore
        println!("{_c1}");
    

    // Coppiare gli elementi di un'array e di uno slice d'array in un vettore
    let _array: [i32; 5] = [1, 2, 3, 4, 5];
    let _array_slice: &[i32; 5] = &[10, 20, 30, 40, 50];

    let _vec4: Vec<i32> = Vec::from(_array);         // Coppia degli elementi dell'array _array nel vettore di integer _vec4
    let _vec5: Vec<i32> = Vec::from(_array_slice);   // Coppia degli elementi dello slice d'array _array_slice nel vettore di integer _vec5



    // Iteratore di un vettore
    let _vec6: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        // Usato per scorerre gli elementi di un vettore
        for i in _vec6.into_iter()
        {
            println!("{i}");
        }



    // Inizializare un vettore nella dimensione e nei valori
    let _l: Vec<i32> = vec![0; 10];    // vettore di dieci elementi inizializati tutti a zero

        // Output in modalità debug -> [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        println!("{:?}", _l);
    


    // Aggiunta di un'elemento nell'ultimo elemento di un vettore
    let mut _m: Vec<i32> = vec![1, 2, 3];
    _m.push(4);

        // Output in modalità debug prima di push() -> [1, 2, 3], Output in modalità debug dopo push() -> [1, 2, 3, 4]
        println!("{:?}", _m);

    

    // Rimozzione dell'ulitmo elemento in un vettore
    let mut _n: Vec<i32> = vec![1, 2, 3, 4];
    _n.pop();

        // Output in modalità debug prima di pop() -> [1, 2, 3, 4], Output in modalità debug dopo pop() -> [1, 2, 3]
        println!("{:?}", _n);



    // Inserimento di un'elemento in un'indice specifico
    let mut _o: Vec<i32> = vec![1, 2, 4, 5];
    _o.insert(2, 3);
    
        // Output in modalità debug prima di insert() -> [1, 2, 4, 5], Output in modalità debug dopo insert() -> [1, 2, 3, 4, 5]
        println!("{:?}", _o);



    // Rimozione di un'elemento in un'indice specifico
    let mut _p: Vec<i32> = vec![1, 2, 3, 4];
    _p.remove(3);
        
        // Output in modalità debug prima di remove() -> [1, 2, 3, 4], Output in modalità debug dopo remove() -> [1, 2, 3]
        println!("{:?}", _p);

    

    // Mantiene solo i numeri del vettore che rispettano certi criteri
    let mut _q: Vec<i32> = vec![1, 2, 3, -4, -5];
    _q.retain(|c: &i32| c.is_positive());

        // Output in modalità debug prima di retain() -> [1, 2, 3, -4, -5], Output in modalità debug dopo retain() -> [1, 2, 3]
        println!("{:?}", _q);



    // Pulizia totale del vettore
    let mut _s: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    _s.clear();
    
        // Output in modalità debug prima di clear() -> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10], Output in modalità debug dopo clear() -> []
        println!("{:?}", _s);



    // Aggiungere gli elementi di un vettore già essistente a quelli di un'altro vettore, (DA NOTARE) -> gli elemeti vengono coppiati da un vettore all'altro; non vengono spostati in modo permanente, può anche essere usato per immetere più numeri in un vettore
    let mut _t1: Vec<i32> = vec![1, 2, 3, 4, 5];
    let mut _t2: Vec<i32> = vec![6, 7, 8, 9, 10];
    let mut _t3: Vec<i32> = Vec::new();

    _t1.extend(_t2);             // Vengono immessi i valori di t2 in t1
    _t3.extend([1, 2, 3, 4]);    // Vengono immessi nel vettore _t3 precedentemente inizializato vuoto, con i valori [1, 2, 3, 4]

        // Output in modalità debug prima di extend() -> [1, 2, 3, 4, 5], Output in modalità debug dopo extend() -> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        println!("{:?}", _t1);

        // Output in modalità debug prima di extend() -> [], Output in modalità debug dopo extend() -> [1, 2, 3, 4]
        println!("{:?}", _t3);



    // Aggiungere gli elementi di un vettore già essistente a quelli di un'altro vettore, (DA NOTARE) -> gli elemeti vengono spostati da un vettore all'altro in modo permanente, NON vengono coppiati da vettore a vettore! Vengono spostati definitivamente!
    let mut _r1: Vec<i32> = vec![1, 2, 3, 4, 5];
    let mut _r2: Vec<i32> = vec![6, 7, 8, 9, 10];

    _r1.append(&mut _r2);

        // Output in modalità debug prima di append() -> [1, 2, 3, 4, 5], Output in modalità debug dopo append() -> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10], _r2 -> []
        println!("{:?}", _r1);    // -> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        println!("{:?}", _r2);    // -> []



    // Rimozione di elementi di un vettore da un'indice fino ad un'altro indice
    let mut _y: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    _y.drain(4..7);

        // Output in modalità debug prima di drain() -> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10], Output in modalità debug dopo drain() -> [1, 2, 3, 4, 8, 9, 10]
        println!("{:?}", _y);



    // Ottenere il valore di un'indice, se l'indice esiste restituisce il dato trovatosì in quell'inidice, altrimenti restituisce None (questo metodo è sicuro e gestisce bene le eventuali fuoriuscite dell'indice dal vettore)
    let mut _z: Vec<i32> = vec![1, 2, 3, 4, 5];
    let _z1 = _z.get(2);

        // Output in modalità debug _z1 -> 3
        println!("{:?}", _z1);



    // Ottenere il valore di un'indice, se l'indice esiste restituisce il dato trovatosì in quell'inidice, altrimenti se l'indice fuoriesce il programma avvia un panic!() e l'esecuzione del programma si interomperà; (DA USARE SOLO E SOLAMENTE SE SI HA LA CERTEZZA CHE L'INDICE ESISTà AL 100%, è COMUNQUE PREFERIBILLE get() )
    let _z2: i32 = _z[0];
    let _z3: i32 = _z[5];

        // Output in modalità debug di _z2
        println!("{:?}", _z2);    // _z2: i32 -> 1

        // Output in modalità debug di _z3
        println!("{:?}", _z2);    // _z3: i32 -> panic!()



    // Modificare il valore di un'indice, se l'indice esiste si va a modificare il valore nell'indice specificato, altrimenti restituisce None
    let mut _x: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    if let Some(_x1) = _x.get_mut(6)
    {
        *_x1 += 2;    // Incremento di due il valore dell'indice 6
    }
    else
    {
        println!("Errore!! Indice non trovato...");    
    }

        // Output in modalità debug prima di get_mut() -> [1, 2, 3, 4, 5, 6, 7, 8, 9], Output in modalità debug dopo get_mut() -> [1, 2, 3, 4, 5, 6, 9, 8, 9]
        println!("{:?}", _x);



    // Modificare il valore di un'indice, se l'indice esiste si andrà a modificare il valore nell'indice indicato tra le quadre, altrimenti verrà generato un panic!() che terminera l'esecuzione del programma si interomperà; (DA USARE SOLO E SOLAMENTE SE SI HA LA CERTEZZA CHE L'INDICE ESISTà AL 100%, è COMUNQUE PREFERIBILLE get_mut() )
    _x[8] = 8;

        // Output in modalità debug di _x -> [1, 2, 3, 4, 5, 6, 7, 8, 8]
        println!("{:?}", _x);

    _x[9] = 9;

        // Output in modalità debug di _x -> panic!()
        println!("{:?}", _x);



    // Ottenere il primo elemento del vettore, ritorna il dato del primo indice del vettore se c'è effetivamente un dato/valore, altrimenti ritorna None
    let mut _ab: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];
    let mut _ab1: Vec<i32> = vec![];

    let _first = _ab.first();
    let _first1 = _ab1.first();

        // Output in modalità debug dopo first() _fisrt -> 1
        println!("{:?}", _first);

        // Output in modalità debug dopo first() _fisrt1 -> None (poichè il vettore è vuoto)
        println!("{:?}", _first1);



    // Ottenere l'ultimo elemento del vettore, ritorna il dato dell'ultimo indice del vettore se c'è effetivamente un dato/valore, altrimenti ritorna None
    let mut _ac: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut _ac1: Vec<i32> = vec![];

    let _last = _ac.last();
    let _last1 = _ac1.last();

        // Output in modalità debug dopo last() _last -> 10
        println!("{:?}", _last);

        // Output in modalità debug dopo last() _last1 -> None (poichè il vettore è vuoto)
        println!("{:?}", _last1);



    // Modificare il primo elemento di un vettore, se il primo indice esiste si va a modificare il valore nel primo indice, altrimenti restituisce None se il vettore è vuoto
    let mut _ad: Vec<i32> = vec![1, 2, 3, 4];

    if let Some(_set_first) = _ad.first_mut()
    {
        *_set_first *= 2;    // Moltiplica per due il valore del primo indice
    }
    else
    {
        println!("Errore!! vettore vuoto...");
    }
    
        // Output in modalità debug prima di first_mut() -> [1, 2, 3, 4], Output in modalità debug dopo first_mut() -> [2, 2, 3, 4]
        println!("{:?}", _ad);



    // Modificare l'ultimo elemento di un vettore, se l'ultimo indice esiste si va a modificare il valore dell'ultimo indice, altrimenti restituisce None se il vettore è vuoto
    let mut _ae: Vec<i32> = vec![1, 2, 3, 4, 5, 6];

    if let Some(_set_last) = _ae.last_mut()
    {
        *_set_last -= 2;    // Deincrementa di due il valore dell'ultimo indice
    }
    else
    {
        println!("Errore!! vettore vuoto...");
    }
    
        // Output in modalità debug prima di last_mut() -> [1, 2, 3, 4, 5, 6], Output in modalità debug dopo last_mut() -> [2, 2, 3, 4, 5, 4]
        println!("{:?}", _ae);



    // Impostare la lunghezza del vettore, in caso di riduzzione della lunghezza i dati precedenti non verrano cancelati ma non vi gli sarà più acesso; in caso di allungamento della lunghezza, il vettore avrà dei nuovi valori sconosciuti "sporchi", questa funzione è sconsigliata all'uso poichè della sua instabilità nell'asegnazione dei valori, quindi va messo dentro un blocco unsafe
    let mut _af: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7 , 8, 9, 10];

    unsafe
    {
        _af.set_len(7);

        // Output in modalità debug prima di set_len(7) -> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10], Output in modalità debug dopo set_len(7) -> [1, 2, 3, 4, 5, 6, 7]
        println!("{:?}", _af);

        _af.set_len(15);

        // Output in modalità debug dopo la seconda chiamata a set_len(15) -> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, ?, ?, ?, ?, ?]
        println!("{:?}", _af);
    }



    // Lunghezza del vettore, restituisce il numero di elemetni che allocano all'interno del vettore
    let mut _ag: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let _lenght: usize = _ag.len();

        // Output della lunghezza del vettore _lenght: usize -> 10
        println!("{_lenght}");



    // Verificare se un vettore è vuoto, ritorna un valore booleano che assume il valore di TRUE (1) se il vettore è vuoto, altrimenti ritorna FALSE (0)
    let mut _ah: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];

        // _is_hollow: bool ritornerà FALSE (0) poichè precedentemente al vettore _ah sono stati precedentemente assegnati dei valori, quindi l'output sarà -> false
        let _is_hollow: bool = _ah.is_empty();
        println!("{_is_hollow}");

        _ah = vec![];   // Svuotamento del vettore

        // _is_hollow: bool ritornerà TRUE (1) poichè precedentemente il vettore _ah è stato svuotato, quindi l'output sarà -> true
        let _is_hollow: bool = _ah.is_empty();
        println!("{_is_hollow}");



    // Riservare la memoria del vettore in anticipo, avvolte per migliorare le prestazioni potrebbe allocare più posizioni del richiesto per evitare di doverle rifare in futuro
    let mut _al: Vec<i32> = Vec::new();
    _al.reserve(13);

        // Output -> 13, 13 allocazioni riservate nel vettore _al
        println!("{}", _al.capacity());



    // Riservare l'esatta memoria del vettore in anticipo
    let mut _am: Vec<i32> = Vec::new();
    _am.reserve_exact(9);
    
        // Output -> 9, 9 allocazioni esatte riservate nel vettore _al
        println!("{}", _am.capacity());



    // Adattare la capacità del vettore alla sua lunghezza
    let mut _an: Vec<i32> = Vec::with_capacity(10);
    _an.extend([1, 2, 3, 4, 5, 6, 7]);

    _an.shrink_to_fit();

        // Output prima di shrink_to_fit() -> len: 7, capacity: 10 | Output dopo shrink_to_fit() -> len: 7, capacity: 7
        println!("{} {}", _an.len(), _an.capacity());



    // Riduzione della lunghezza di un vettore
    let mut _ao: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
    _ao.truncate(8);

        // Output in modalità debug prima di truncate -> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14] | Output in modalità debug dopo truncate -> [1, 2, 3, 4, 5, 6, 7, 8]
        println!("{:?}", _ao);



    // Ordinare un vettore con l'algoritmo Tim sort, è un'ordinamento stabile ma non molto efficente con grandi data set poichè occupa un po' tanta memoria
    let mut _ap: Vec<i32> = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    _ap.sort();

        // Output in modalità debug prima di sort() -> [10, 9, 8, 7, 6, 5, 4, 3, 2, 1] | Output in modalità debug dopo sort() -> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        println!("{:?}", _ap);



    // Ordinare un vettore secondo uno specifico ordine scelto da te
    _ap = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    _ap.sort_by(|a, b| a.cmp(b));

        // Output in modalità debug prima di sort_by() -> [10, 9, 8, 7, 6, 5, 4, 3, 2, 1] | Output in modalità debug dopo sort_by() -> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        println!("{:?}", _ap);



    // Ordinare un vettore con l'algoritmo Quick sort per piccoli data set, mentre usa l'Heap sort per grandi data set
    _ap = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    _ap.sort_unstable();
    
        // Output in modalità debug prima di sort_unstable() -> [10, 9, 8, 7, 6, 5, 4, 3, 2, 1] | Output in modalità debug dopo sort_unstable() -> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        println!("{:?}", _ap);
        


    // Ordinare un vettore secondo uno specifico ordine scelto da te, ordina con l'algoritmo Quick sort per piccoli data set, mentre usa l'Heap sort per grandi data set
    _ap = vec![15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    _ap.sort_unstable_by(|a, b| a.cmp(b));    // Oridina in ordine cresecente

        // Output in modalità debug prima di sort_unstable_by() -> [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1] | Output in modalità debug dopo sort_unstable() -> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        println!("{:?}", _ap);



    // Inverte l'ordine degli elementi di un vettore
    _ap = vec![15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    _ap.reverse();

        // Output in modalità debug prima di reverse() -> [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1] | Output in modalità debug dopo reverse() -> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        println!("{:?}", _ap);



    // Rimozione degli elementi duplicati all'interno del vettore
    _ap = vec![1, 2, 3, 4, 4, 4, 5, 6, 7, 7, 8, 9, 9, 10];
    _ap.dedup();

        // Output in modalità debug prima di dedup() -> [1, 2, 3, 4, 4, 4, 5, 6, 7, 7, 8, 9, 9, 10] | Output in modalità debug dopo dedup() -> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        println!("{:?}", _ap);



    // Rimozione degli elementi duplicati all'interno del vettore che rispettino un certo criterio
    let mut _aq: Vec<&str> = vec!["CIAO", "ciao", "BELLO", "bello", "NELLO", "nello"];

    _aq.dedup_by(|a, b| a.eq_ignore_ascii_case(b));    // eq_ignore_ascii_case ignora le differenze tra MAIUSCOLO e minuscolo

        // Output in modalità debug prima di dedup_by -> ["CIAO", "ciao", "BELLO", "bello", "NELLO", "nello"] |  Output in modalità debug prima di dedup_by -> ["CIAO", "BELLO", "NELLO"]
        println!("{:?}", _aq);



    // Iteratore iter() -> restituisce uno a uno gli elementi del vettore (crea un'iteratore che accede uno a uno agli elementi del vettore in modo immutabile)
    let mut _as: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        for x in _as.iter()
        {
            print!("{x} ");    // output -> 1 2 3 4 5 6 7 8 9 10
        }



    // Iteratore iter_mut() -> itera gli elementi del vettore e li modifica diretamente (crea un'iteratore che accede uno a uno agli elementi del vettore in modo mutabile)
    let mut _at: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        for j in _at.iter_mut()
        {
            *j += 1;    // incrementa di uno tutti gli elementi del vettore, '*' viene usato per accedere all'elemento diretamente
        }

        // Output in modalita debug prima di iter_mut() -> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] | Output in modalita debug dopo iter_mut() -> [2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
        println!("{:?}", _at);



    // Trasferire gli elementi del vettore ad un'iteratore, succesivamente il vettore sarà inutilizzabile
    let mut _vec39:  Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        for l in _vec39.into_iter()
        {
            print!("{l} ");    // Stampa uno a uno gli elementi del vettore
        }



    // Convertire un vettore in un Box<[T]> ovvero un'array allocato sull'heap invece che sullo stack
    let mut _vec40:  Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];    // Vettore

    let mut _box: Box<[i32]> = _vec40.into_boxed_slice();    // Conversione del vettore in una Box<[T]>
    
        // Output in modalità debug di _box: Box<[i32]> -> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        println!("{:?}", _box);



    // Convertire un Box<[T]> in un vettore
    let mut _box1: Box<[i32]> =  Box::new([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let mut _vec41: Vec<i32> = _box1.into_vec();
        
        // Output in modalità debug del vettore _vec41: Vec<i32> -> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        println!("{:?}", _vec41);



    // map() -> Applicare una trasformazione su ogni elemento di un'iteratore, si usa in concomitanza di un'iteratore!
    let mut _vec42: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // con iter_mut() scorriamo il vettore e permettiamo eventuali modifiche, con map() effetiamo una trasformazione tramite una chiusura ( |a| *a * 2) e poi con collect() "consumiamo" gli elementi del vettore in un nuovo contenitore, Vec, array o Box 
    let _moltiplicati: Vec<_> = _vec42.iter_mut().map(|a| *a * 2).collect();

        // Output in modalità debug prima di map
        println!("{:?}", _moltiplicati);



    // Filtrare gli elementi di un vettore data una determinata condizione
    let mut _vec43: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // con iter() scorriamo il vettore, con filter() e la chiusura al suo interno diciamo che solo i numeri pari, ovvero che danno resto 0, sono validi, e con collect() immetiamo i nuovi valori dentro il nuovo vettore; DA TENERE A MENTE -> iter() NON cambia i valori originali del vettore
    let _pari: Vec<_> = _vec43.iter().filter(|a| *a % 2 == 0).collect();

        // Output in modalità debug prima di filter -> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] | Output in modalità debug dopo filter -> [2, 4, 6, 8, 10]
        println!("{:?}", _pari);



    // Combinare gli elementi del vettore tramite una determinato criterio
    let mut _vec44: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Il primo parametro è da dove inizia l'operazione, il secondo elemento passato tramite chiusura indica l'operazione che si va a fare sul vettore, va usata con un'iteratore iter()
    let _somma = _vec44.iter().fold(0, |a, b| a + *b);

        // Output di _somma: i32 -> 55
        println!("{}", _somma);



    // eq() -> Confrontare se due puntatori fanno riferimento allo stesso segmento dati in memoria, i due vettori vanno passati con per reference '&'
    let mut _vec_a: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut _vec_b: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Passiamo a _same_reference lo stesso riferimento alla memoria di _vec_a
    let _same_reference: &Vec<i32> = &_vec_a;
    
    // Passiamo a _not_same: bool il confronto dell'operazione che sarà false (0) poichè ben si i due vettori abbiano stessi valori vengono allocati in zone della memoria heap diverse
    let _not_same: bool = eq(&_vec_a, &_vec_b);

    // Passiamo a _same_vec_pointer: bool il confronto dell'operazione che sarà true (1) poichè stiamo confrontando la stessa zona di memoria
    let _same_vec_pointer: bool = eq(&_vec_a, _same_reference);

        // Output -> false
        println!("{_not_same}");

        // Output -> true
        println!("{_same_vec_pointer}");



    // en() -> Confrontare se due vettori hanno gli stessi elementi, ritorna un valore booleno FALSE (0) se ambo i vettori hanno gli stessi elementi, TRUE (1) se i vettori hanno elementi diversi
    let mut _vec45: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut _vec46: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut _vec47: Vec<i32> = vec![1, 2, 9, 4, 2, 6, 3];
    
    // en() restituirà -> FALSE (0) poichè i due vettori _vec45 e _vec46 hanno gli stessi elementi
    let _same_value: bool = Vec::ne(&_vec45, &_vec46);
    
    // en() restituirà -> TRUE (1) poichè i due vettori _vec45 e _vec47 hanno diversi elementi tra di loro
    let _not_same_value: bool = Vec::ne(&_vec45, &_vec47);

        // Output _same_value: bool -> false
        println!("{_same_value}");

        // Output _not_same_value: bool -> true
        println!("{_not_same_value}");



    // Dividere il vettore in due parti dicendo da quale indice deve tronacre il vettore in due
    let mut _vec48: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Tronchiamo il vettore all'indice 5 creando così due slice che vengono immesi in _part1 e in _part2
    let (_part1, _part2) = _vec48.split_at(5);

        // Output in modalità debug del primo pezzo del vettore -> [1, 2, 3, 4, 5]
        println!("{:?}", _part1);

        // Output in modalità debug del secondo pezzo del vettore -> [6, 7, 8, 9, 10]
        println!("{:?}", _part2);



    // Dividere il vettore in due parti dicendo da quale indice deve tronacre il vettore in due, e poi modifica i due slice
    let mut _vec49: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        // Tronchiamo il vettore all'indice 5 creando così due slice che vengono immesi in _part1 e in _part2, che potranno essere modificati
        let (_part1, _part2) = _vec49.split_at_mut(5);

        // Accediamo in modo siccuro all'elemento del primo slice
        if let Some(_h1) = _part1.get_mut(0)
        {
            *_h1 += 1;    // incrementiamo di uno il primo elemento del primo slice
        }
        else
        {
            println!("Elemento non trovato!!!");    
        }
        
        // Accediamo in modo siccuro all'elemento del secondo slice
        if let Some(_h2) = _part2.get_mut(0)
        {
            *_h2 += 1;    // incrementiamo di uno il primo elemento del secondo slice
        }
        else
        {
            println!("Elemento non trovato!!!");    
        }

        // Output in modalità debug prima di split_at_mut() -> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] | Output in modalità debug dopo split_at_mut() -> [2, 2, 3, 4, 5, 7, 7, 8, 9, 10]
        println!("{:?}", _vec49);
}