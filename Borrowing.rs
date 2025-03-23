// questo programma spiega il borrowing in breve
fn main()
{
    // BORROWING IMMUTABILE
    let x = 10;
    let y = &x;     // y avrà il contenuto di x in prestito! Senza però modificare effetivamente il contenuto di x
    let t = &x;     // t avrà il contenuto di x in prestito! Senza però modificare effetivamente il contenuto di x

    // più prestiti immutabili possono coesistere

    println!("x: {x} y: {y} t: {t}");

    // BORROWING MUTABILE
    let mut z = 22;
    let i = &mut z;     // i avrà il contenuto di z in prestito!, però qui il contenuto sarà modificato e tornera modificato in z

    *i+=1;

    println!("i: {i}");

    // CLONAZIONE
    let s1 = String::from("ciao bello mio?!");
    let s2 = s1.clone();    // s2 avrà il contenuto di s1 coppiato, non prestato ma coppiato!

    println!("s2: {s2}");
}

/*
    ERRORI COMUNI NEL BORROWING:

    1) è permesso solo un borrowing mutabile alla volta:
    let mut z = 22;
    let i = &mut z;   <- okay
    let d = &mut z;   <- errore: non si possono avere due prestiti mutabili

    2) fare un riferimento mutabile e uno immutabile allo stesso tempo:
    let mut z1 = 20;
    let z2 = &z1;      <- riferimento immutabile
    let z3 = &mut z1;  <- errore [non si possono fare riferimenti muttabili con altri riferimenti immutabili]

    Questi errori vengono catturati dal compilatore
*/
