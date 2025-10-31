# ŕja

![](https://github.com/sstanovnik/rja/raw/vodilna/logo.png)

Vas pisanje programov jezika Rust v angleščini utruja?
Čutite notranje zadovoljstvo, ko izrečete "jojmene"?
Želite izkusiti pisanje visoko zmogljivega in dokazljivo pravilnega programja v maternem jeziku?
Bi vas veselilo prisostvovanje pri napredku slovenščine v tehnični stroki?

**ŕja** vas bo razveselila!
Z njo boste lahko uživali vse prednosti pisanja programske kode znotraj ekosistema Rust, a uporabili polnovredne
ustreznice v slovenščini, ki vas bodo navdale z do sedaj nepredstavljivim občutkov izraznosti pri programiranju.

Ta projekt je bil zasnovan za uporabo kot uradni programski jezik za razvoj bodočega Operacijskega sistema
slovenske neodvisnosti (OSSN).

Če ste, dragi bralec, član slovenske vlade ali ste na odločevalskem položaju, ki vam narekuje ohranjanje slovenščine,
bomo veseli vaše finančne podpore na [liberapay](https://liberapay.com/bnjbvr/).

Lahko ste brez skrbi, saj slovenska inačica programskega jezika Rust podpira več narečij, prav tako pa lahko še vedno
uporabljate angleške izraze in jih vmešate v programsko kodo po želji, če vam to narekuje situacija.

Nadaljujmo s primerom uporabe ŕje:

### Uporaba

```rust
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
}
```

### Podpora za slovenska narečja

```rust
#[dovoli(nedosegljiva_koda)]
funkcija drugotna() {
    jojmene!("neki ne dela");    // za osrednjeslovensko regijo
    porkamadona!("ma ne djela"); // za wajdušno
    čuj!("nena dela");           // za tiste bližje kurji glavi
}
```

### Other examples

Preglej [primere](primeri/src/main.rs) za obširnejši pregled nad zmožnostmi jezika.

## Ostali jeziki

- Arabic: [sada](https://github.com/LAYGATOR/sada)
- Bulgarian: [ryzhda](https://github.com/gavadinov/ryzhda)
- Catalan: [rovell](https://github.com/gborobio73/rovell)
- Chinese: [xiu (锈)](https://github.com/lucifer1004/xiu)
- Corsican: [rughjina](https://github.com/aldebaranzbradaradjan/rughjina)
- Croatian: [hrđa](https://github.com/njelich/hrdja)
- Czech: [rez](https://github.com/radekvit/rez)
- Danish: [rust?](https://github.com/LunaTheFoxgirl/rust-dk)
- Dutch: [roest](https://github.com/jeroenhd/roest)
- Esperanto: [rustteksto](https://github.com/dscottboggs/rustteksto)
- Estonian: [rooste](https://github.com/hanshs/rooste)
- Finnish: [ruoste](https://github.com/vkoskiv/ruoste)
- French: [rouille](https://github.com/bnjbvr/rouille/)
- German: [rost](https://github.com/michidk/rost)
- Greek: [skouriasmeno](https://github.com/devlocalhost/skouriasmeno)
- Hindi: [zung](https://github.com/rishit-khandelwal/zung)
- Hungarian: [rozsda](https://github.com/jozsefsallai/rozsda)
- Indonesian: [karat](https://github.com/annurdien/karat)
- Italian: [ruggine](https://github.com/DamianX/ruggine)
- Japanese: [sabi (錆)](https://github.com/yuk1ty/sabi)
- Kannada: [tukku (ತುಕ್ಕು)](https://github.com/sanathNU/tukku.git)
- Korean: [Nok (녹)](https://github.com/Alfex4936/nok)
- Latin: [ferrugo](https://github.com/pianoman911/ferrugo)
- Malagasy: [arafesina](https://github.com/luckasRanarison/arafesina)
- Marathi: [gan̄ja](https://github.com/pranavgade20/ganja)
- Nepali: [khiya (खिया)](https://github.com/sudanchapagain/khiya.git)
- Norwegian: [korrosjon](https://github.com/datagutt/korrosjon)
- Persian: [zangar (زنگار)](https://github.com/ui-ce/zangar)
- Polish: [rdza](https://github.com/phaux/rdza)
- Romanian: [rugină](https://github.com/aionescu/rugina)
- Russian: [Ржавый](https://github.com/Sanceilaks/rzhavchina)
- Sanskrit: [jangam](https://github.com/ishantanu/jangam.git)
- Scottish Gaelic: [meirg](https://github.com/KSPAtlas/meirg)
- Slovak: [hrdza](https://github.com/TheMessik/hrdza)
- Spanish: [rustico](https://github.com/UltiRequiem/rustico)
- Swedish: [rost](https://github.com/vojd/rost/)
- Swiss: [roeschti](https://github.com/Georg-code/roeschti)
- Thai: [sanim (สนิม)](https://github.com/korewaChino/sanim)
- Toki Pona: [jaki kiwen](https://github.com/jgcodes2020/jaki-kiwen)
- Turkish: [pas](https://github.com/ekimb/pas)
- Ukrainian: [irzha](https://github.com/brokeyourbike/irzha)
- Vietnamese: [gỉ](https://github.com/Huy-Ngo/gir)
- All of the above: [unirust](https://github.com/charyan/unirust)

## Licenca

Delo je licencirano pod slovenskim prevodom [WTFPL](http://www.wtfpl.net/),
licence _delaj kar hočeš_.
