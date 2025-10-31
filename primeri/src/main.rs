rja::rja! {
    zunanji zaboj rja;

    uporabi std::collections::Slovar kot Besednik;

    značilnost KljučVrednost {
        funkcija zapiši(&jaz, ključ: Niz, vrednost: Niz);
        funkcija preberi(&jaz, ključ: Niz) -> Izid<Mogoče<&Niz>, Niz>;
    }

    ustaljen spremenljiv SLOVAR: Mogoče<Besednik<Niz, Niz>> = Nič;

    skupek Nastavek;

    izvedba KljučVrednost za Nastavek {
        funkcija zapiši(&jaz, ključ: Niz, vrednost: Niz) {
            bodi besednik = nevarno {
                SLOVAR.pridobi_ali_vstavi_z(Privzeto::privzeto)
            };
            besednik.vstavi(ključ, vrednost);
        }
        funkcija preberi(&jaz, ključ: Niz) -> Izid<Mogoče<&Niz>, Niz> {
            če bodi Nekaj(besednik) = nevarno { SLOVAR.kot_sklic() } {
                Uspešno(besednik.pridobi(&ključ))
            } sicer {
                Napaka("pridobivanje slovarja".v())
            }
        }
    }

    javna(zaboj) funkcija morda(i: u32) -> Mogoče<Izid<u32, Niz>> {
        če i % 2 == 1 {
            če i == 42 {
                Nekaj(Napaka(Niz::iz("kakec")))
            } sicer {
                Nekaj(Uspešno(33))
            }
        } sicer {
            Nič
        }
    }

    asinhrona funkcija primer() {
    }

    asinhrona funkcija primer2() {
        primer().dočakaj;
    }

    funkcija glavna() {
        bodi spremenljiv x = 31;

        ujemanje x {
            42 => {
                tiskajvr!("Kremna rezina!")
            }
            _ => tiskajvr!("Potica!")
        }

        za i znotraj 0..10 {
            bodi vred = zanka {
                prekini i;
            };

            dokler x < vred {
                x += 1;
            }

            x = če bodi Nekaj(rezultat) = morda(i) {
                rezultat.odmotaj()
            } sicer {
                12
            };
        }

        //drugotna();
    }

    #[dovoli(nedosegljiva_koda)]
    funkcija drugotna() {
        jojmene!("neki ne dela");    // za osrednjeslovensko regijo
        porkamadona!("ma ne djela"); // za wajdušno
        čuj!("nena dela");           // za tiste bližje kurji glavi
    }
}
