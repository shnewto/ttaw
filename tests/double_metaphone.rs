extern crate ttaw;

use ttaw::pronounciation::double_metaphone;

#[test]
fn ptah() {
    assert_eq!(
        double_metaphone("ptah"),
        Ok(vec!["PT".to_string(), "PT".to_string()])
    )
}

#[test]
fn ceasar() {
    assert_eq!(
        double_metaphone("ceasar"),
        Ok(vec!["SSR".to_string(), "SSR".to_string()])
    )
}

#[test]
fn ach() {
    assert_eq!(
        double_metaphone("ach"),
        Ok(vec!["AK".to_string(), "AK".to_string()])
    )
}

#[test]
fn chemical() {
    assert_eq!(
        double_metaphone("chemical"),
        Ok(vec!["KMKL".to_string(), "KMKL".to_string()])
    )
}

#[test]
fn choral() {
    assert_eq!(
        double_metaphone("choral"),
        Ok(vec!["KRL".to_string(), "KRL".to_string()])
    )
}

#[test]
fn aleksander() {
    assert_eq!(
        double_metaphone("alexander"),
        double_metaphone("aleksander")
    )
}

#[test]
fn hiccups() {
    assert_eq!(double_metaphone("HICCUPS"), double_metaphone("HiCcUpS"));
    assert_eq!(double_metaphone("HiCcUpS"), double_metaphone("hiccups"));
}

#[test]
fn gnarl() {
    assert_eq!(
        double_metaphone("gnarl").unwrap().first().unwrap().get(..1),
        Some("N")
    );
}

#[test]
fn knack() {
    assert_eq!(
        double_metaphone("knack").unwrap().first().unwrap().get(..1),
        Some("N")
    );
}

#[test]
fn pneumatic() {
    assert_eq!(
        double_metaphone("pneumatic")
            .unwrap()
            .first()
            .unwrap()
            .get(..1),
        Some("N")
    );
}

#[test]
fn wrack() {
    assert_eq!(
        double_metaphone("wrack").unwrap().first().unwrap().get(..1),
        Some("R")
    );
}

#[test]
fn psycho() {
    assert_eq!(
        double_metaphone("psycho")
            .unwrap()
            .first()
            .unwrap()
            .get(..1),
        Some("S")
    );
}

#[test]
fn xavier() {
    assert_eq!(
        double_metaphone("Xavier")
            .unwrap()
            .first()
            .unwrap()
            .get(..1),
        Some("S")
    );
}

#[test]
fn vowels() {
    assert_eq!(double_metaphone("a").unwrap().first().unwrap(), "A");
    assert_eq!(double_metaphone("e").unwrap().first().unwrap(), "A");
    assert_eq!(double_metaphone("i").unwrap().first().unwrap(), "A");
    assert_eq!(double_metaphone("o").unwrap().first().unwrap(), "A");
    assert_eq!(double_metaphone("u").unwrap().first().unwrap(), "A");
    assert_eq!(double_metaphone("y").unwrap().first().unwrap(), "A");
}

#[test]
fn drop_initial() {
    assert_eq!(double_metaphone("ba").unwrap().first().unwrap().len(), 1);
    assert_eq!(double_metaphone("be").unwrap().first().unwrap().len(), 1);
    assert_eq!(double_metaphone("bi").unwrap().first().unwrap().len(), 1);
    assert_eq!(double_metaphone("bo").unwrap().first().unwrap().len(), 1);
    assert_eq!(double_metaphone("bu").unwrap().first().unwrap().len(), 1);
    assert_eq!(double_metaphone("by").unwrap().first().unwrap().len(), 1);
}

#[test]
fn b_to_p() {
    assert_eq!(
        double_metaphone("b").unwrap().first().unwrap().get(..1),
        Some("P")
    );
    assert_eq!(
        double_metaphone("bb").unwrap().first().unwrap().get(..1),
        Some("P")
    );
}

#[test]
fn c_cedilla_to_s() {
    assert_eq!(
        double_metaphone("Ç").unwrap().first().unwrap().get(..1),
        Some("S")
    );
}

#[test]
fn when_c_to_k() {
    assert_eq!(
        double_metaphone("ACH").unwrap().get(0).unwrap().get(1..2),
        Some("K")
    );
    assert_ne!(
        double_metaphone("AACH").unwrap().get(0).unwrap().get(2..3),
        Some("K")
    );
    assert_ne!(
        double_metaphone("ACHI").unwrap().get(0).unwrap().get(1..2),
        Some("K")
    );
    assert_eq!(
        double_metaphone("ACHB").unwrap().get(0).unwrap().get(1..2),
        Some("K")
    );
    assert_eq!(
        double_metaphone("MACHER")
            .unwrap()
            .get(1)
            .unwrap()
            .get(1..2),
        Some("K")
    );
    assert_eq!(
        double_metaphone("BACHER")
            .unwrap()
            .get(1)
            .unwrap()
            .get(1..2),
        Some("K")
    );
}

#[test]
fn caesar() {
    assert_eq!(
        double_metaphone("CAESAR").unwrap().get(0).unwrap().get(..1),
        Some("S")
    );
}

#[test]
fn chianti() {
    assert_eq!(
        double_metaphone("chianti")
            .unwrap()
            .get(0)
            .unwrap()
            .get(..1),
        Some("K")
    );
}

#[test]
fn michael() {
    assert_eq!(
        double_metaphone("michael")
            .unwrap()
            .get(0)
            .unwrap()
            .get(1..2),
        Some("K")
    );

    assert_eq!(
        double_metaphone("michael")
            .unwrap()
            .get(1)
            .unwrap()
            .get(1..2),
        Some("X")
    );
}

#[test]
fn chiastic() {
    assert_eq!(
        double_metaphone("chiastic")
            .unwrap()
            .get(0)
            .unwrap()
            .get(..1),
        Some("K")
    );
}

#[test]
fn chemical_c_to_k() {
    assert_eq!(
        double_metaphone("chemical")
            .unwrap()
            .get(0)
            .unwrap()
            .get(..1),
        Some("K")
    );
}

#[test]
fn choral_c_to_k() {
    assert_eq!(
        double_metaphone("choral").unwrap().get(0).unwrap().get(..1),
        Some("K")
    );
}

#[test]
fn chyme_c_to_k() {
    assert_eq!(
        double_metaphone("chyme").unwrap().get(0).unwrap().get(..1),
        Some("K")
    );
}

#[test]
fn character_c_to_k() {
    assert_eq!(
        double_metaphone("character")
            .unwrap()
            .get(0)
            .unwrap()
            .get(..1),
        Some("K")
    );
}

#[test]
fn charisma_c_to_k() {
    assert_eq!(
        double_metaphone("charisma")
            .unwrap()
            .get(0)
            .unwrap()
            .get(..1),
        Some("K")
    );
}

#[test]
fn von_ch_c_to_k() {
    assert_eq!(
        double_metaphone("von ch")
            .unwrap()
            .get(0)
            .unwrap()
            .get(2..3),
        Some("K")
    );
}

#[test]
fn schooner_c_to_k() {
    assert_eq!(
        double_metaphone("schooner")
            .unwrap()
            .get(0)
            .unwrap()
            .get(1..2),
        Some("K")
    );
}

#[test]
fn orchestra_c_to_k() {
    assert_eq!(
        double_metaphone("orchestra")
            .unwrap()
            .get(0)
            .unwrap()
            .get(2..3),
        Some("K")
    );
}

#[test]
fn architect_c_to_k() {
    assert_eq!(
        double_metaphone("architect")
            .unwrap()
            .get(0)
            .unwrap()
            .get(2..3),
        Some("K")
    );
}

#[test]
fn arch_not_c_to_k() {
    assert_ne!(
        double_metaphone("arch").unwrap().get(0).unwrap().get(2..3),
        Some("K")
    );
}

#[test]
fn orchid_c_to_k() {
    assert_eq!(
        double_metaphone("orchid")
            .unwrap()
            .get(0)
            .unwrap()
            .get(2..3),
        Some("K")
    );
}

#[test]
fn chthonic_c_to_k() {
    assert_eq!(
        double_metaphone("chthonic")
            .unwrap()
            .get(0)
            .unwrap()
            .get(..1),
        Some("K")
    );
}

#[test]
fn fuchsia_c_to_k() {
    assert_eq!(
        double_metaphone("fuchsia")
            .unwrap()
            .get(0)
            .unwrap()
            .get(1..2),
        Some("K")
    );
}

#[test]
fn chloride_c_to_k() {
    assert_eq!(
        double_metaphone("chloride")
            .unwrap()
            .get(0)
            .unwrap()
            .get(..1),
        Some("K")
    );
}

#[test]
fn chroma_c_to_k() {
    assert_eq!(
        double_metaphone("chroma").unwrap().get(0).unwrap().get(..1),
        Some("K")
    );
}

#[test]
fn tichner_c_to_k() {
    assert_eq!(
        double_metaphone("tichner")
            .unwrap()
            .get(1)
            .unwrap()
            .get(1..2),
        Some("K")
    );
}

#[test]
fn mchugh_c_to_k() {
    assert_eq!(
        double_metaphone("McHugh")
            .unwrap()
            .get(0)
            .unwrap()
            .get(1..2),
        Some("K")
    );
}

#[test]
fn chore() {
    assert_eq!(
        double_metaphone("chore").unwrap().get(0).unwrap().get(..1),
        Some("X")
    );
}

#[test]
fn h_after_c() {
    assert_eq!(
        double_metaphone("achievement")
            .unwrap()
            .get(0)
            .unwrap()
            .get(1..2),
        Some("X")
    );

    assert_eq!(
        double_metaphone("achievement")
            .unwrap()
            .get(1)
            .unwrap()
            .get(1..2),
        Some("K")
    );
}

#[test]
fn czerny() {
    assert_eq!(
        double_metaphone("czerny").unwrap().get(0).unwrap().get(..1),
        Some("S")
    );

    assert_eq!(
        double_metaphone("czerny").unwrap().get(1).unwrap().get(..1),
        Some("X")
    );
}

#[test]
fn focaccia() {
    assert_eq!(
        double_metaphone("focaccia")
            .unwrap()
            .get(0)
            .unwrap()
            .get(2..3),
        Some("X")
    );
}

#[test]
fn accident() {
    assert_eq!(
        double_metaphone("accident")
            .unwrap()
            .get(0)
            .unwrap()
            .get(1..2),
        Some("K")
    );
    assert_eq!(
        double_metaphone("accident")
            .unwrap()
            .get(0)
            .unwrap()
            .get(2..3),
        Some("S")
    );
}

#[test]
fn accede() {
    assert_eq!(
        double_metaphone("accede")
            .unwrap()
            .get(0)
            .unwrap()
            .get(1..2),
        Some("K")
    );
    assert_eq!(
        double_metaphone("accede")
            .unwrap()
            .get(0)
            .unwrap()
            .get(2..3),
        Some("S")
    );
}

#[test]
fn succeed() {
    assert_eq!(
        double_metaphone("succeed")
            .unwrap()
            .get(0)
            .unwrap()
            .get(1..2),
        Some("K")
    );
    assert_eq!(
        double_metaphone("succeed")
            .unwrap()
            .get(0)
            .unwrap()
            .get(2..3),
        Some("S")
    );
}

#[test]
fn bertucci() {
    assert_eq!(
        double_metaphone("bertucci")
            .unwrap()
            .get(0)
            .unwrap()
            .get(3..4),
        Some("X")
    );
}

#[test]
fn hiccups_c_to_k() {
    assert_eq!(
        double_metaphone("hiccups")
            .unwrap()
            .get(0)
            .unwrap()
            .get(1..2),
        Some("K")
    );
}

#[test]
fn knack_c_to_k() {
    assert_eq!(
        double_metaphone("knack").unwrap().get(0).unwrap().get(1..2),
        Some("K")
    );
}

#[test]
fn ancient() {
    assert_eq!(
        double_metaphone("ancient")
            .unwrap()
            .get(0)
            .unwrap()
            .get(2..3),
        Some("S")
    );

    assert_eq!(
        double_metaphone("ancient")
            .unwrap()
            .get(1)
            .unwrap()
            .get(2..3),
        Some("X")
    );
}

#[test]
fn delicious() {
    assert_eq!(
        double_metaphone("delicious")
            .unwrap()
            .get(0)
            .unwrap()
            .get(2..3),
        Some("S")
    );

    assert_eq!(
        double_metaphone("delicious")
            .unwrap()
            .get(1)
            .unwrap()
            .get(2..3),
        Some("X")
    );
}

#[test]
fn acicula() {
    assert_eq!(
        double_metaphone("acicula")
            .unwrap()
            .get(0)
            .unwrap()
            .get(1..2),
        Some("S")
    );
}

#[test]
fn abduce() {
    assert_eq!(
        double_metaphone("abduce")
            .unwrap()
            .get(0)
            .unwrap()
            .get(3..4),
        Some("S")
    );
}

#[test]
fn acyl() {
    assert_eq!(
        double_metaphone("acyl").unwrap().get(0).unwrap().get(1..2),
        Some("S")
    );
}

#[test]
fn maccaffery() {
    assert_eq!(
        double_metaphone("Mac Caffrey")
            .unwrap()
            .get(0)
            .unwrap()
            .get(1..2),
        Some("K")
    )
}

#[test]
fn macgregor() {
    assert_eq!(
        double_metaphone("Mac Gregor")
            .unwrap()
            .get(0)
            .unwrap()
            .get(1..2),
        Some("K")
    )
}

#[test]
fn macquillan() {
    assert_eq!(
        double_metaphone("Mac Quillan")
            .unwrap()
            .get(0)
            .unwrap()
            .get(1..2),
        Some("K")
    )
}
#[test]
fn aback() {
    assert_eq!(
        double_metaphone("aback").unwrap().get(0).unwrap().get(2..3),
        Some("K")
    )
}
#[test]
fn acquit() {
    assert_eq!(
        double_metaphone("acquit")
            .unwrap()
            .get(0)
            .unwrap()
            .get(1..2),
        Some("K")
    )
}
#[test]
fn acclimate() {
    assert_eq!(
        double_metaphone("acclimate")
            .unwrap()
            .get(0)
            .unwrap()
            .get(1..2),
        Some("K")
    )
}
#[test]
fn edge() {
    assert_eq!(
        double_metaphone("edge").unwrap().get(0).unwrap().get(1..2),
        Some("J")
    )
}
#[test]
fn pidgin() {
    assert_eq!(
        double_metaphone("pidgin")
            .unwrap()
            .get(0)
            .unwrap()
            .get(1..2),
        Some("J")
    )
}
#[test]
fn edgy() {
    assert_eq!(
        double_metaphone("edgy").unwrap().get(0).unwrap().get(1..2),
        Some("J")
    )
}
#[test]
fn edgar() {
    assert_eq!(
        double_metaphone("Edgar").unwrap().get(0).unwrap().get(1..3),
        Some("TK")
    )
}
#[test]
fn width() {
    assert_eq!(
        double_metaphone("width").unwrap().get(0).unwrap().get(1..2),
        Some("T")
    )
}
#[test]
fn add() {
    assert_eq!(
        double_metaphone("add").unwrap().get(0).unwrap().get(1..2),
        Some("T")
    )
}
#[test]
fn abduce_slice() {
    assert_eq!(
        double_metaphone("Abduce")
            .unwrap()
            .get(0)
            .unwrap()
            .get(2..3),
        Some("T")
    )
}
#[test]
fn affect() {
    assert_eq!(
        double_metaphone("affect")
            .unwrap()
            .get(0)
            .unwrap()
            .get(1..2),
        Some("F")
    )
}
#[test]
fn abaft() {
    assert_eq!(
        double_metaphone("abaft").unwrap().get(0).unwrap().get(2..3),
        Some("F")
    )
}
#[test]
fn aargh() {
    assert_eq!(
        double_metaphone("aargh").unwrap().get(0).unwrap().get(2..3),
        Some("K")
    )
}
#[test]
fn ghislane() {
    assert_eq!(
        double_metaphone("ghislane")
            .unwrap()
            .get(0)
            .unwrap()
            .get(..1),
        Some("J")
    )
}
#[test]
fn ghoul() {
    assert_eq!(
        double_metaphone("ghoul").unwrap().get(0).unwrap().get(..1),
        Some("K")
    )
}
#[test]
fn hugh() {
    assert_eq!(
        double_metaphone("hugh").unwrap().get(0),
        Some(&"H".to_string())
    )
}
#[test]
fn bough() {
    assert_eq!(
        double_metaphone("bough").unwrap().get(0),
        Some(&"P".to_string())
    )
}
#[test]
fn broughton() {
    assert_eq!(
        double_metaphone("broughton").unwrap().get(0),
        Some(&"PRTN".to_string())
    )
}
#[test]
fn laugh() {
    assert_eq!(
        double_metaphone("laugh").unwrap().get(0),
        Some(&"LF".to_string())
    )
}
#[test]
fn curagh() {
    assert_eq!(
        double_metaphone("curagh").unwrap().get(0),
        Some(&"KRK".to_string())
    )
}
#[test]
fn weight() {
    assert_eq!(
        double_metaphone("weight").unwrap().get(0),
        Some(&"AT".to_string())
    )
}

//   t.test(
//     'should transform GN to KN and N, when preceded by a vowel and ^, and not Slavo-Germanic',
//     function(st) {
//       var phonetics = double_metaphone("agnize")

//       assert_eq!(phonetics[0].get(..3), 'AKN")
//       assert_eq!(phonetics[1].get(..2), 'AN")

//       st.end()
//     }
//   )

//   t.deepEqual(
//     double_metaphone("tagliaro"),
//     ['TKLR', 'TLR'],
//     'should transform GLI to KL and L'
//   )

//   t.test(
//     'should transform GN to N and KN, when not followed by EY and Y, and not Slavo-Germanic',
//     function(st) {
//       var phonetics = double_metaphone("acceptingness")

//       assert_eq!(phonetics[0].slice(-3), 'NNS")
//       assert_eq!(phonetics[1].slice(-4), 'NKNS")

//       st.end()
//     }
//   )

#[test]
fn cagney() {
    assert_eq!(
        double_metaphone("cagney").unwrap().get(0),
        Some(&"KKN".to_string())
    )
}

//   t.test(
//     'should transform an initial GY., GES, GEP, GEB, GEL, GEY, GIB, GIL, GIN, GIE, GEI, and GER to K and J',
//     function(st) {
//       var phonetics = double_metaphone("Gerben")

//       assert_eq!(phonetics[0].unwrap().get(..1), 'K")
//       assert_eq!(phonetics[1].unwrap().get(..1), 'J")

//       st.end()
//     }
//   )

//   t.test(
//     'should transform GER to K and J, when not in DANGER, RANGER, and MANGER, and not preceded by E and I',
//     function(st) {
//       var phonetics = double_metaphone("auger")

//       assert_eq!(phonetics[0].unwrap().get(1..2), 'K")
//       assert_eq!(phonetics[1].unwrap().get(1..2), 'J")

//       st.end()
//     }
//   )

//   t.test(
//     'should transform GY to K and J, when not preceded by E, I, R, and O',
//     function(st) {
//       var phonetics = double_metaphone("bulgy")

//       assert_eq!(phonetics[0].unwrap().get(2..3), 'K")
//       assert_eq!(phonetics[1].unwrap().get(2..3), 'J")

//       st.end()
//     }
//   )

//   assert_eq!(
#[test]
fn altogether() {
    assert_eq!(
        double_metaphone("altogether")
            .unwrap()
            .get(0)
            .unwrap()
            .get(3..4),
        Some("K")
    )
}
//     'should transform the G in GET to K'
//   )
//   assert_eq!(
#[test]
fn vanagema() {
    assert_eq!(
        double_metaphone("Van Agema")
            .unwrap()
            .get(0)
            .unwrap()
            .get(2..3),
        Some("K")
    )
}
//     'should transform G to K, when Germanic and followed by E, I, or Y'
//   )
//   assert_eq!(
#[test]
fn vongoggin() {
    assert_eq!(
        double_metaphone("Von Goggin")
            .unwrap()
            .get(0)
            .unwrap()
            .get(3..4),
        Some("K")
    )
}
//     'should transform G to K, when Germanic, preceded by A or O, and followed by GI'
//   )
//   assert_eq!(
#[test]
fn tangier() {
    assert_eq!(
        double_metaphone("tangier")
            .unwrap()
            .get(0)
            .unwrap()
            .get(2..3),
        Some("J")
    )
}
//     'should transform G to J, when followed by "IER "'
//   )

//   t.test(
//     'should transform G to J and K, when followed by E, I, or Y, or preceded by A or O and followed by GI',
//     function(st) {
//       var phonetics = double_metaphone("biaggi")

//       assert_eq!(phonetics[0].unwrap().get(1..2), 'J")
//       assert_eq!(phonetics[1].unwrap().get(1..2), 'K")

//       st.end()
//     }
//   )

#[test]
fn two_gs() {
    assert_eq!(
        double_metaphone("GG").unwrap().get(0),
        Some(&"K".to_string())
    )
}
#[test]
fn one_g() {
    assert_eq!(
        double_metaphone("G").unwrap().get(0),
        Some(&"K".to_string())
    )
}
#[test]
fn ha() {
    assert_eq!(
        double_metaphone("ha").unwrap().get(0),
        Some(&"H".to_string())
    )
}
//   assert_eq!(
#[test]
fn aha() {
    assert_eq!(
        double_metaphone("aha").unwrap().get(0),
        Some(&"AH".to_string())
    )
}
//     'should keep H when both followed and preceded by a vowel'
//   )
#[test]
fn one_h() {
    assert_eq!(double_metaphone("h").unwrap().get(0), Some(&"".to_string()))
}
//   assert_eq!(
#[test]
fn sanjacinto() {
    assert_eq!(
        double_metaphone("San Jacinto")
            .unwrap()
            .get(0)
            .unwrap()
            .get(2..3),
        Some("H")
    )
}
//     'should transform J to H when obviously spanish (an initial "SAN ")'
//   )
//   assert_eq!(
#[test]
fn jose() {
    assert_eq!(
        double_metaphone("Jose").unwrap().get(0),
        Some(&"H".to_string())
    )
}
//     'should transform J to H in an initial "J... "'
//   )

//   t.test('should transform the J to J and H, when in JOSE', function(st) {
//     var phonetics = double_metaphone("Joseph")

//     assert_eq!(phonetics[0].unwrap().get(..1), 'J")
//     assert_eq!(phonetics[1].unwrap().get(..1), 'H")

//     st.end()
//   })

//   t.test('should transform the J to J and H, when in JOSE', function(st) {
//     var phonetics = double_metaphone("Jankelowicz")

//     assert_eq!(phonetics[0].unwrap().get(..1), 'J")
//     assert_eq!(phonetics[1].unwrap().get(..1), 'A")

//     st.end()
//   })

//   t.test(
//     'should transform J to J and H, when preceded by a vowel, followed by A or O, and not Slavo-Germanic',
//     function(st) {
//       var phonetics = double_metaphone("bajador")

//       assert_eq!(phonetics[0].unwrap().get(1..2), 'J")
//       assert_eq!(phonetics[1].unwrap().get(1..2), 'H")

//       st.end()
//     }
//   )

//   t.test('should both keep and drop a final J', function(st) {
//     var phonetics = double_metaphone("svaraj")

//     assert_eq!(phonetics[0], 'SFRJ")
//     assert_eq!(phonetics[1], 'SFR")

//     st.end()
//   })

//   assert_eq!(
#[test]
fn abject() {
    assert_eq!(
        double_metaphone("abject")
            .unwrap()
            .get(0)
            .unwrap()
            .get(2..3),
        Some("J")
    )
}
//     'should keep J when not preceded by S, K, and L, and not followed by L, T, K, S, N, M, B, and Z'
//   )
#[test]
fn sjji() {
    assert_eq!(
        double_metaphone("sjji").unwrap().get(0).unwrap().get(..1),
        Some("S")
    )
}
#[test]
fn disject() {
    assert_eq!(
        double_metaphone("disject").unwrap().get(0),
        Some(&"TSKT".to_string())
    )
}
#[test]
fn trekker() {
    assert_eq!(
        double_metaphone("trekker").unwrap().get(0),
        Some(&"TRKR".to_string())
    )
}
#[test]
fn like() {
    assert_eq!(
        double_metaphone("like").unwrap().get(0),
        Some(&"LK".to_string())
    )
}

//   t.test(
//     'should both transform LL to L, and drop it, when in a final ILLO, ILLA and ALLE',
//     function(st) {
//       st.deepEqual(double_metaphone("cabrillo"), ['KPRL', 'KPR'])
//       st.deepEqual(double_metaphone("villa"), ['FL', 'F'])
//       st.deepEqual(double_metaphone("crevalle"), ['KRFL', 'KRF'])

//       st.end()
//     }
//   )

//   t.test(
//     'should both transform the LL to L, and drop it, in ALLE, when the given value ends in A, O, AS, or OS',
//     function(st) {
//       st.deepEqual(double_metaphone("allegretto"), ['ALKRT', 'AKRT'])
//       st.deepEqual(double_metaphone("allegros"), ['ALKRS', 'AKRS'])
//       st.end()
//     }
//   )

#[test]
fn two_lls() {
    assert_eq!(
        double_metaphone("ll").unwrap().get(0),
        Some(&"L".to_string())
    )
}
#[test]
fn one_l() {
    assert_eq!(
        double_metaphone("l").unwrap().get(0),
        Some(&"L".to_string())
    )
}
#[test]
fn thumb() {
    assert_eq!(
        double_metaphone("thumb").unwrap().get(0),
        Some(&"0M".to_string())
    )
}
//   assert_eq!(
#[test]
fn dumber() {
    assert_eq!(
        double_metaphone("dumber").unwrap().get(0),
        Some(&"TMR".to_string())
    )
}
//     'should transform UMB to M when followed by ER'
//   )
#[test]
fn tow_mms() {
    assert_eq!(
        double_metaphone("mm").unwrap().get(0),
        Some(&"M".to_string())
    )
}
#[test]
fn one_m() {
    assert_eq!(
        double_metaphone("m").unwrap().get(0),
        Some(&"M".to_string())
    )
}
#[test]
fn two_nns() {
    assert_eq!(
        double_metaphone("nn").unwrap().get(0),
        Some(&"N".to_string())
    )
}
#[test]
fn one_n() {
    assert_eq!(
        double_metaphone("n").unwrap().get(0),
        Some(&"N".to_string())
    )
}
#[test]
fn top_tilda_n() {
    assert_eq!(
        double_metaphone("Ñ").unwrap().get(0),
        Some(&"N".to_string())
    )
}
#[test]
fn ph() {
    assert_eq!(
        double_metaphone("ph").unwrap().get(0),
        Some(&"F".to_string())
    )
}
#[test]
fn pb() {
    assert_eq!(
        double_metaphone("pb").unwrap().get(0),
        Some(&"P".to_string())
    )
}
#[test]
fn twp_pps() {
    assert_eq!(
        double_metaphone("pp").unwrap().get(0),
        Some(&"P".to_string())
    )
}
#[test]
fn one_p() {
    assert_eq!(
        double_metaphone("p").unwrap().get(0),
        Some(&"P".to_string())
    )
}
#[test]
fn two_qqs() {
    assert_eq!(
        double_metaphone("qq").unwrap().get(0),
        Some(&"K".to_string())
    )
}
#[test]
fn one_q() {
    assert_eq!(
        double_metaphone("q").unwrap().get(0),
        Some(&"K".to_string())
    )
}

//   t.deepEqual(
//     double_metaphone("Xavier"),
//     ['SF', 'SFR'],
//     'should both drop and keep a final R when preceded by IE, in turn not preceded by ME and MA'
//   )

#[test]
fn two_rrs() {
    assert_eq!(
        double_metaphone("rr").unwrap().get(0),
        Some(&"R".to_string())
    )
}
#[test]
fn one_r() {
    assert_eq!(
        double_metaphone("r").unwrap().get(0),
        Some(&"R".to_string())
    )
}
//   assert_eq!(
#[test]
fn island() {
    assert_eq!(
        double_metaphone("island").unwrap().get(0),
        Some(&"ALNT".to_string())
    )
}
//     'should drop S when preceded by I or Y and followed by L'
//   )

//   t.test('should transform the S to X and S in an initial SUGAR', function(st) {
//     var phonetics = double_metaphone("sugar")

//     assert_eq!(phonetics[0].unwrap().get(..1), 'X")
//     assert_eq!(phonetics[1].unwrap().get(..1), 'S")

//     st.end()
//   })

//   assert_eq!(
#[test]
fn sholz() {
    assert_eq!(
        double_metaphone("Sholz").unwrap().get(0).unwrap().get(..1),
        Some("S")
    )
}
//     'should transform the SH to S in SHEIM, SHOEK, SHOLM, SHOLZ'
//   )
#[test]
fn sh() {
    assert_eq!(
        double_metaphone("sh").unwrap().get(0).unwrap().get(..1),
        Some("X")
    )
}

//   t.deepEqual(
//     double_metaphone("sio"),
//     ['S', 'X'],
//     'should transform SIO and SIA to S and X, when not Slavo-Germanic'
//   )

//   t.deepEqual(
//     double_metaphone("sioricz"),
//     ['SRS', 'SRX'],
//     'should transform SIO and SIA to S, when Slavo-Germanic'
//   )

//   t.deepEqual(double_metaphone("sz"), ['S', 'X'], 'should transform SZ to X and S")

//   t.deepEqual(
//     double_metaphone("sl"),
//     ['SL', 'XL'],
//     'should transform S to X and S when followed by L, M, N, or W'
//   )

//   t.deepEqual(
//     double_metaphone("schenker"),
//     ['XNKR', 'SKNKR'],
//     'should transform SCH to X and SK when followed by ER or EN'
//   )

//   t.deepEqual(
//     double_metaphone("schooner"),
//     ['SKNR', 'SKNR'],
//     'should transform SCH to SK when followed by OO, UY, ED, or EM'
//   )

//   t.deepEqual(
//     double_metaphone("schlepp"),
//     ['XLP', 'SLP'],
//     'should transform SCH to X and S, when initial, and not followed by a non-vowel and W'
//   )

#[test]
fn borscht() {
    assert_eq!(
        double_metaphone("borscht").unwrap().get(0),
        Some(&"PRXT".to_string())
    )
}
#[test]
fn sci() {
    assert_eq!(
        double_metaphone("sci").unwrap().get(0),
        Some(&"S".to_string())
    )
}
#[test]
fn scu() {
    assert_eq!(
        double_metaphone("scu").unwrap().get(0),
        Some(&"SK".to_string())
    )
}

//   t.deepEqual(
//     double_metaphone("ois"),
//     ['A', 'AS'],
//     'should drop and keep S, when final and preceded by AI or OI'
//   )

#[test]
fn two_sss() {
    assert_eq!(
        double_metaphone("ss").unwrap().get(0),
        Some(&"S".to_string())
    )
}
#[test]
fn one_s() {
    assert_eq!(
        double_metaphone("s").unwrap().get(0),
        Some(&"S".to_string())
    )
}
#[test]
fn tion() {
    assert_eq!(
        double_metaphone("tion").unwrap().get(0),
        Some(&"XN".to_string())
    )
}
#[test]
fn tia() {
    assert_eq!(
        double_metaphone("tia").unwrap().get(0),
        Some(&"X".to_string())
    )
}
#[test]
fn tch() {
    assert_eq!(
        double_metaphone("tch").unwrap().get(0),
        Some(&"X".to_string())
    )
}
//   assert_eq!(
#[test]
fn thom() {
    assert_eq!(
        double_metaphone("thom").unwrap().get(0),
        Some(&"TM".to_string())
    )
}
//     'should transform TH to T, when followed by OM or AM (1)'
//   )
//   assert_eq!(
#[test]
fn tham() {
    assert_eq!(
        double_metaphone("tham").unwrap().get(0),
        Some(&"TM".to_string())
    )
}
//     'should transform TH to T, when followed by OM or AM (2)'
//   )
//   assert_eq!(
#[test]
fn vongoethals() {
    assert_eq!(
        double_metaphone("Von Goethals")
            .unwrap()
            .get(0)
            .unwrap()
            .get(3..4),
        Some("T")
    )
}
//     'should transform TH to T, when Germanic'
//   )
//   assert_eq!(
#[test]
fn vonmatthes() {
    assert_eq!(
        double_metaphone("Von Matthes")
            .unwrap()
            .get(0)
            .unwrap()
            .get(3..4),
        Some("T")
    )
}
//     'should transform TT to T, when Germanic and followed by H'
//   )

//   t.deepEqual(double_metaphone("th"), ['0', 'T'], 'should transform TH to 0 and T")

#[test]
fn two_tts() {
    assert_eq!(
        double_metaphone("tt").unwrap().get(0),
        Some(&"T".to_string())
    )
}
#[test]
fn td() {
    assert_eq!(
        double_metaphone("td").unwrap().get(0),
        Some(&"T".to_string())
    )
}
#[test]
fn one_t() {
    assert_eq!(
        double_metaphone("t").unwrap().get(0),
        Some(&"T".to_string())
    )
}
#[test]
fn two_vvs() {
    assert_eq!(
        double_metaphone("vv").unwrap().get(0),
        Some(&"F".to_string())
    )
}
#[test]
fn one_v() {
    assert_eq!(
        double_metaphone("v").unwrap().get(0),
        Some(&"F".to_string())
    )
}
#[test]
fn awr() {
    assert_eq!(
        double_metaphone("awr").unwrap().get(0),
        Some(&"AR".to_string())
    )
}

//   t.deepEqual(
//     double_metaphone("wa"),
//     ['A', 'F'],
//     'should transform W to A and F, when initial and followed by a vowel'
//   )

//   assert_eq!(
#[test]
fn wh() {
    assert_eq!(
        double_metaphone("wh").unwrap().get(0),
        Some(&"A".to_string())
    )
}
//     'should transform W to A, when initial and followed by H'
//   )

//   t.test(
//     'should both drop and transform W to F, when in EWSKI, EWSKY, OWSKI, or OWSKY',
//     function(st) {
//       st.deepEqual(double_metaphone("Tsjaikowski"), ['TSKSK', 'TSKFSK'])
//       st.deepEqual(double_metaphone("Tsjaikowsky"), ['TSKSK', 'TSKFSK'])

//       st.end()
//     }
//   )

//   t.deepEqual(
//     double_metaphone("schwa"),
//     ['X', 'XF'],
//     'should both drop and transform W to F, when the value starts with SCH'
//   )

//   t.deepEqual(
//     double_metaphone("Arnow"),
//     ['ARN', 'ARNF'],
//     'should both drop and transform W to F, when final and preceded by a vowel'
//   )

//   t.test(
//     'should transform W to TS and FX, when followed by ICZ or ITZ',
//     function(st) {
//       st.deepEqual(double_metaphone("Filipowicz"), ['FLPTS', 'FLPFX'])
//       st.deepEqual(double_metaphone("Filipowitz"), ['FLPTS', 'FLPFX'])

//       st.end()
//     }
//   )

#[test]
fn w() {
    assert_eq!(double_metaphone("w").unwrap().get(0), Some(&"".to_string()))
}
#[test]
fn matrix() {
    assert_eq!(
        double_metaphone("matrix").unwrap().get(0),
        Some(&"MTRKS".to_string())
    )
}

//   t.test(
//     'should transform X to KS, when *NOT* preceded by IAU, EAU, AU, or OU',
//     function(st) {
#[test]
fn iauxa() {
    assert_eq!(
        double_metaphone("iauxa").unwrap().get(0),
        Some(&"AKS".to_string())
    )
}
#[test]
fn eauxa() {
    assert_eq!(
        double_metaphone("eauxa").unwrap().get(0),
        Some(&"AKS".to_string())
    )
}
#[test]
fn auxa() {
    assert_eq!(
        double_metaphone("auxa").unwrap().get(0),
        Some(&"AKS".to_string())
    )
}
#[test]
fn ouxa() {
    assert_eq!(
        double_metaphone("ouxa").unwrap().get(0),
        Some(&"AKS".to_string())
    )
}

//       st.end()
//     }
//   )

#[test]
fn aux() {
    assert_eq!(
        double_metaphone("AUX").unwrap().get(0),
        Some(&"A".to_string())
    )
}
#[test]
fn oux() {
    assert_eq!(
        double_metaphone("OUX").unwrap().get(0),
        Some(&"A".to_string())
    )
}
#[test]
fn breaux() {
    assert_eq!(
        double_metaphone("breaux").unwrap().get(0),
        Some(&"PR".to_string())
    )
}

#[test]
fn axc() {
    assert_eq!(
        double_metaphone("AXC").unwrap().get(0),
        Some(&"AKS".to_string())
    )
}
#[test]
fn axx() {
    assert_eq!(
        double_metaphone("axx").unwrap().get(0),
        Some(&"AKS".to_string())
    )
}
#[test]
fn axe() {
    assert_eq!(
        double_metaphone("axe").unwrap().get(0),
        Some(&"AKS".to_string())
    )
}
#[test]
fn zhao() {
    assert_eq!(
        double_metaphone("zhao").unwrap().get(0),
        Some(&"J".to_string())
    )
}

//   t.test(
//     'should transform Z to S and TS, when followed by ZA, ZI, or ZO',
//     function(st) {
//       st.deepEqual(double_metaphone("zza"), ['S', 'TS'])
//       st.deepEqual(double_metaphone("zzi"), ['S', 'TS'])
//       st.deepEqual(double_metaphone("zzo"), ['S', 'TS'])

//       st.end()
//     }
//   )

//   t.deepEqual(
//     double_metaphone("Mazurkiewicz"),
//     ['MSRKTS', 'MTSRKFX'],
//     'should transform Z to S and TS, when not initial, not Slavo-Germanic, and not preceded by T'
//   )

#[test]
fn two_zzs() {
    assert_eq!(
        double_metaphone("zz").unwrap().get(0),
        Some(&"S".to_string())
    )
}
#[test]
fn one_z() {
    assert_eq!(
        double_metaphone("z").unwrap().get(0),
        Some(&"S".to_string())
    )
}

//   t.end()
// })

// test('cli', function(t) {
//   var input = new PassThrough()
//   var helps = ['-h', '--help']
//   var versions = ['-v', '--version']

//   t.plan(7)

//   execa.stdout('./cli.js', ['michael']).then(function(result) {
//     assert_eq!(result, 'MKL	MXL', 'argument")
//   })

//   execa.stdout('./cli.js', ['detestable', 'vileness']).then(function(result) {
//     assert_eq!(result, 'TTSTPL\tTTSTPL FLNS\tFLNS', 'arguments")
//   })

//   execa.stdout('./cli.js', {input: input}).then(function(result) {
//     assert_eq!(result, 'TTSTPL\tTTSTPL FLNS\tFLNS', 'stdin")
//   })

//   input.write('detestable")

//   setImmediate(function() {
//     input.end(' vileness")
//   })

//   helps.forEach(function(flag) {
//     execa.stdout('./cli.js', [flag]).then(function(result) {
//       t.ok(/\s+Usage: double-metaphone/.test(result), flag)
//     })
//   })

//   versions.forEach(function(flag) {
//     execa.stdout('./cli.js', [flag]).then(function(result) {
//       assert_eq!(result, version, flag)
//     })
//   })
// })
