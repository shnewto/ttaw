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

//   t.test(
//     'should transform the C to S and X, when followed by Z and not preceded by WI',
//     function(st) {
//       var phonetics = m('czerny')

//       st.equal(phonetics[0].charAt(0), 'S')
//       st.equal(phonetics[1].charAt(0), 'X')

//       st.end()
//     }
//   )

//   t.equal(
//     m('focaccia')[0].charAt(2),
//     'X',
//     'should transform the C to X, when followed by CIA'
//   )

//   t.test(
//     'should transform the C to KS, when in an initial ACC, followed by either E, I, or H (but not HU)',
//     function(st) {
//       var phonetics = m('accident')

//       st.equal(phonetics[0].charAt(1), 'K')
//       st.equal(phonetics[0].charAt(2), 'S')

//       phonetics = m('accede')

//       st.equal(phonetics[0].charAt(1), 'K')
//       st.equal(phonetics[0].charAt(2), 'S')

//       st.end()
//     }
//   )

//   t.test('should transform the C to KS, when in UCCEE or UCCES', function(st) {
//     var phonetics = m('succeed')

//     st.equal(phonetics[0].charAt(1), 'K')
//     st.equal(phonetics[0].charAt(2), 'S')

//     st.end()
//   })

//   t.test(
//     'should transform the C to X, when followed by C (but not in an initial MCC), either E, I, or H (but not HU)',
//     function(st) {
//       st.equal(m('bacci')[0].charAt(1), 'X')
//       st.equal(m('bertucci')[0].charAt(3), 'X')

//       st.end()
//     }
//   )

//   t.equal(
//     m('hiccups')[0].charAt(1),
//     'K',
//     'should transform the C to K, when followed by C (but not in an initial MCC)'
//   )
//   t.equal(
//     m('knack')[0].charAt(1),
//     'K',
//     'should transform the C to K, when followed by either G, K, or Q'
//   )

//   t.test(
//     'should transform the C to S and X, when followed by I and either E, or O',
//     function(st) {
//       var phonetics = m('ancient')

//       st.equal(phonetics[0].charAt(2), 'S')
//       st.equal(phonetics[1].charAt(2), 'X')

//       phonetics = m('delicious')

//       st.equal(phonetics[0].charAt(2), 'S')
//       st.equal(phonetics[1].charAt(2), 'X')

//       st.end()
//     }
//   )

//   t.test(
//     'should transform the C to S, when followed by either I, E, or Y',
//     function(st) {
//       st.equal(m('acicula')[0].charAt(1), 'S')
//       st.equal(m('abduce')[0].charAt(3), 'S')
//       st.equal(m('acyl')[0].charAt(1), 'S')

//       st.end()
//     }
//   )

//   t.equal(m('Mac Caffrey')[0].charAt(1), 'K', 'should transform "C C" to K')
//   t.equal(m('Mac Gregor')[0].charAt(1), 'K', 'should transform "C G" to K')
//   t.equal(m('Mac Quillan')[0].charAt(1), 'K', 'should transform "C G" to K')
//   t.equal(m('aback')[0].charAt(2), 'K', 'should transform CK to K')
//   t.equal(m('acquit')[0].charAt(1), 'K', 'should transform CQ to K')
//   t.equal(
//     m('acclimate')[0].charAt(1),
//     'K',
//     'should transform CC to K, when not followed by E or I'
//   )
//   t.equal(m('edge')[0].charAt(1), 'J', 'should transform DGE to J')
//   t.equal(m('pidgin')[0].charAt(1), 'J', 'should transform DGI to J')
//   t.equal(m('edgy')[0].charAt(1), 'J', 'should transform DGY to J')
//   t.equal(m('Edgar')[0].slice(1, 3), 'TK', 'should transform DG to TK')
//   t.equal(m('width')[0].charAt(1), 'T', 'should transform DT to T')
//   t.equal(m('add')[0].charAt(1), 'T', 'should transform DD to T')
//   t.equal(m('Abduce')[0].charAt(2), 'T', 'should transform D to T')
//   t.equal(m('affect')[0].charAt(1), 'F', 'should transform FF to F')
//   t.equal(m('abaft')[0].charAt(2), 'F', 'should transform F to F')
//   t.equal(
//     m('aargh')[0].charAt(2),
//     'K',
//     'should transform GH to K when preceded by a consonant'
//   )
//   t.equal(m('ghislane')[0].charAt(0), 'J', 'should transform initial GHI to J')
//   t.equal(m('ghoul')[0].charAt(0), 'K', 'should transform initial GH to K')
//   t.equal(m('hugh')[0], 'H', 'should drop GH in B.GH, H.GH, or D.GH')
//   t.equal(m('bough')[0], 'P', 'should drop GH in B..GH, H..GH, or D..GH')
//   t.equal(m('broughton')[0], 'PRTN', 'should drop GH in B...GH or H...GH')
//   t.equal(
//     m('laugh')[0],
//     'LF',
//     'should transform GH to F in C.UGH, G.UGH, L.UGH, R.UGH, T.UGH'
//   )
//   t.equal(
//     m('curagh')[0],
//     'KRK',
//     'should transform GH to K, when preceded by anything other than I'
//   )
//   t.equal(m('weight')[0], 'AT', 'should drop GH')

//   t.test(
//     'should transform GN to KN and N, when preceded by a vowel and ^, and not Slavo-Germanic',
//     function(st) {
//       var phonetics = m('agnize')

//       st.equal(phonetics[0].slice(0, 3), 'AKN')
//       st.equal(phonetics[1].slice(0, 2), 'AN')

//       st.end()
//     }
//   )

//   t.deepEqual(
//     m('tagliaro'),
//     ['TKLR', 'TLR'],
//     'should transform GLI to KL and L'
//   )

//   t.test(
//     'should transform GN to N and KN, when not followed by EY and Y, and not Slavo-Germanic',
//     function(st) {
//       var phonetics = m('acceptingness')

//       st.equal(phonetics[0].slice(-3), 'NNS')
//       st.equal(phonetics[1].slice(-4), 'NKNS')

//       st.end()
//     }
//   )

//   t.equal(m('cagney')[0], 'KKN', 'should transform GN to KN')

//   t.test(
//     'should transform an initial GY., GES, GEP, GEB, GEL, GEY, GIB, GIL, GIN, GIE, GEI, and GER to K and J',
//     function(st) {
//       var phonetics = m('Gerben')

//       st.equal(phonetics[0].charAt(0), 'K')
//       st.equal(phonetics[1].charAt(0), 'J')

//       st.end()
//     }
//   )

//   t.test(
//     'should transform GER to K and J, when not in DANGER, RANGER, and MANGER, and not preceded by E and I',
//     function(st) {
//       var phonetics = m('auger')

//       st.equal(phonetics[0].charAt(1), 'K')
//       st.equal(phonetics[1].charAt(1), 'J')

//       st.end()
//     }
//   )

//   t.test(
//     'should transform GY to K and J, when not preceded by E, I, R, and O',
//     function(st) {
//       var phonetics = m('bulgy')

//       st.equal(phonetics[0].charAt(2), 'K')
//       st.equal(phonetics[1].charAt(2), 'J')

//       st.end()
//     }
//   )

//   t.equal(
//     m('altogether')[0].charAt(3),
//     'K',
//     'should transform the G in GET to K'
//   )
//   t.equal(
//     m('Van Agema')[0].charAt(2),
//     'K',
//     'should transform G to K, when Germanic and followed by E, I, or Y'
//   )
//   t.equal(
//     m('Von Goggin')[0].charAt(3),
//     'K',
//     'should transform G to K, when Germanic, preceded by A or O, and followed by GI'
//   )
//   t.equal(
//     m('tangier')[0].charAt(2),
//     'J',
//     'should transform G to J, when followed by "IER "'
//   )

//   t.test(
//     'should transform G to J and K, when followed by E, I, or Y, or preceded by A or O and followed by GI',
//     function(st) {
//       var phonetics = m('biaggi')

//       st.equal(phonetics[0].charAt(1), 'J')
//       st.equal(phonetics[1].charAt(1), 'K')

//       st.end()
//     }
//   )

//   t.equal(m('GG')[0], 'K', 'should transform GG to K')
//   t.equal(m('G')[0], 'K', 'should transform G to K')
//   t.equal(m('ha')[0], 'H', 'should keep H when initial and followed by a vowel')
//   t.equal(
//     m('aha')[0],
//     'AH',
//     'should keep H when both followed and preceded by a vowel'
//   )
//   t.equal(m('h')[0], '', 'should drop H')
//   t.equal(
//     m('San Jacinto')[0].charAt(2),
//     'H',
//     'should transform J to H when obviously spanish (an initial "SAN ")'
//   )
//   t.equal(
//     m('Jose')[0].charAt(0),
//     'H',
//     'should transform J to H in an initial "J... "'
//   )

//   t.test('should transform the J to J and H, when in JOSE', function(st) {
//     var phonetics = m('Joseph')

//     st.equal(phonetics[0].charAt(0), 'J')
//     st.equal(phonetics[1].charAt(0), 'H')

//     st.end()
//   })

//   t.test('should transform the J to J and H, when in JOSE', function(st) {
//     var phonetics = m('Jankelowicz')

//     st.equal(phonetics[0].charAt(0), 'J')
//     st.equal(phonetics[1].charAt(0), 'A')

//     st.end()
//   })

//   t.test(
//     'should transform J to J and H, when preceded by a vowel, followed by A or O, and not Slavo-Germanic',
//     function(st) {
//       var phonetics = m('bajador')

//       st.equal(phonetics[0].charAt(1), 'J')
//       st.equal(phonetics[1].charAt(1), 'H')

//       st.end()
//     }
//   )

//   t.test('should both keep and drop a final J', function(st) {
//     var phonetics = m('svaraj')

//     st.equal(phonetics[0], 'SFRJ')
//     st.equal(phonetics[1], 'SFR')

//     st.end()
//   })

//   t.equal(
//     m('abject')[0].charAt(2),
//     'J',
//     'should keep J when not preceded by S, K, and L, and not followed by L, T, K, S, N, M, B, and Z'
//   )
//   t.equal(m('sjji')[0].charAt(0), 'S', 'should drop JJ')
//   t.equal(m('disject')[0], 'TSKT', 'should drop J')
//   t.equal(m('trekker')[0], 'TRKR', 'should transform KK to K')
//   t.equal(m('like')[0], 'LK', 'should keep K')

//   t.test(
//     'should both transform LL to L, and drop it, when in a final ILLO, ILLA and ALLE',
//     function(st) {
//       st.deepEqual(m('cabrillo'), ['KPRL', 'KPR'])
//       st.deepEqual(m('villa'), ['FL', 'F'])
//       st.deepEqual(m('crevalle'), ['KRFL', 'KRF'])

//       st.end()
//     }
//   )

//   t.test(
//     'should both transform the LL to L, and drop it, in ALLE, when the given value ends in A, O, AS, or OS',
//     function(st) {
//       st.deepEqual(m('allegretto'), ['ALKRT', 'AKRT'])
//       st.deepEqual(m('allegros'), ['ALKRS', 'AKRS'])
//       st.end()
//     }
//   )

//   t.equal(m('ll')[0], 'L', 'should transform LL to L')
//   t.equal(m('l')[0], 'L', 'should keep L')
//   t.equal(m('thumb')[0], '0M', 'should transform a final UMB to M')
//   t.equal(
//     m('dumber')[0],
//     'TMR',
//     'should transform UMB to M when followed by ER'
//   )
//   t.equal(m('mm')[0], 'M', 'should transform MM to M')
//   t.equal(m('m')[0], 'M', 'should keep M')
//   t.equal(m('nn')[0], 'N', 'should transform NN to N')
//   t.equal(m('n')[0], 'N', 'should keep N')
//   t.equal(m('Ñ')[0], 'N', 'should transform Ñ to N')
//   t.equal(m('ph')[0], 'F', 'should transform PH to F')
//   t.equal(m('pb')[0], 'P', 'should transform PB to P')
//   t.equal(m('pp')[0], 'P', 'should transform PP to P')
//   t.equal(m('p')[0], 'P', 'should keep P')
//   t.equal(m('qq')[0], 'K', 'should transform QQ to K')
//   t.equal(m('q')[0], 'K', 'should transform Q to K')

//   t.deepEqual(
//     m('Xavier'),
//     ['SF', 'SFR'],
//     'should both drop and keep a final R when preceded by IE, in turn not preceded by ME and MA'
//   )

//   t.equal(m('rr')[0], 'R', 'should transform RR to R')
//   t.equal(m('r')[0], 'R', 'should keep R')
//   t.equal(
//     m('island')[0],
//     'ALNT',
//     'should drop S when preceded by I or Y and followed by L'
//   )
//   t.equal(
//     m('island')[0],
//     'ALNT',
//     'should drop S when preceded by I or Y and followed by L'
//   )

//   t.test('should transform the S to X and S in an initial SUGAR', function(st) {
//     var phonetics = m('sugar')

//     st.equal(phonetics[0].charAt(0), 'X')
//     st.equal(phonetics[1].charAt(0), 'S')

//     st.end()
//   })

//   t.equal(
//     m('Sholz')[0].charAt(0),
//     'S',
//     'should transform the SH to S in SHEIM, SHOEK, SHOLM, SHOLZ'
//   )
//   t.equal(m('sh')[0].charAt(0), 'X', 'should transform the SH to X')

//   t.deepEqual(
//     m('sio'),
//     ['S', 'X'],
//     'should transform SIO and SIA to S and X, when not Slavo-Germanic'
//   )

//   t.deepEqual(
//     m('sioricz'),
//     ['SRS', 'SRX'],
//     'should transform SIO and SIA to S, when Slavo-Germanic'
//   )

//   t.deepEqual(m('sz'), ['S', 'X'], 'should transform SZ to X and S')

//   t.deepEqual(
//     m('sl'),
//     ['SL', 'XL'],
//     'should transform S to X and S when followed by L, M, N, or W'
//   )

//   t.deepEqual(
//     m('schenker'),
//     ['XNKR', 'SKNKR'],
//     'should transform SCH to X and SK when followed by ER or EN'
//   )

//   t.deepEqual(
//     m('schooner'),
//     ['SKNR', 'SKNR'],
//     'should transform SCH to SK when followed by OO, UY, ED, or EM'
//   )

//   t.deepEqual(
//     m('schlepp'),
//     ['XLP', 'SLP'],
//     'should transform SCH to X and S, when initial, and not followed by a non-vowel and W'
//   )

//   t.equal(m('borscht')[0], 'PRXT', 'should transform SCH to X')
//   t.equal(m('sci')[0], 'S', 'should transform SCI, SCE, and SCY to S')
//   t.equal(m('scu')[0], 'SK', 'should transform SC. to SK')

//   t.deepEqual(
//     m('ois'),
//     ['A', 'AS'],
//     'should drop and keep S, when final and preceded by AI or OI'
//   )

//   t.equal(m('ss')[0], 'S', 'should transform SS to S')
//   t.equal(m('s')[0], 'S', 'should keep S')
//   t.equal(m('tion')[0], 'XN', 'should transform TIO to X, when followed by N')
//   t.equal(m('tia')[0], 'X', 'should transform TIA and TCH to X')
//   t.equal(m('tch')[0], 'X', 'should transform TIA and TCH to X')
//   t.equal(
//     m('thom')[0],
//     'TM',
//     'should transform TH to T, when followed by OM or AM (1)'
//   )
//   t.equal(
//     m('tham')[0],
//     'TM',
//     'should transform TH to T, when followed by OM or AM (2)'
//   )
//   t.equal(
//     m('Von Goethals')[0].charAt(3),
//     'T',
//     'should transform TH to T, when Germanic'
//   )
//   t.equal(
//     m('Von Matthes')[0].charAt(3),
//     'T',
//     'should transform TT to T, when Germanic and followed by H'
//   )

//   t.deepEqual(m('th'), ['0', 'T'], 'should transform TH to 0 and T')

//   t.equal(m('tt')[0], 'T', 'should transform TT to T')
//   t.equal(m('td')[0], 'T', 'should transform TD to T')
//   t.equal(m('t')[0], 'T', 'should keep T')
//   t.equal(m('vv')[0], 'F', 'should transform VV to F')
//   t.equal(m('v')[0], 'F', 'should transform V to F')
//   t.equal(m('awr')[0], 'AR', 'should transform WR to R')

//   t.deepEqual(
//     m('wa'),
//     ['A', 'F'],
//     'should transform W to A and F, when initial and followed by a vowel'
//   )

//   t.equal(
//     m('wh')[0],
//     'A',
//     'should transform W to A, when initial and followed by H'
//   )

//   t.test(
//     'should both drop and transform W to F, when in EWSKI, EWSKY, OWSKI, or OWSKY',
//     function(st) {
//       st.deepEqual(m('Tsjaikowski'), ['TSKSK', 'TSKFSK'])
//       st.deepEqual(m('Tsjaikowsky'), ['TSKSK', 'TSKFSK'])

//       st.end()
//     }
//   )

//   t.deepEqual(
//     m('schwa'),
//     ['X', 'XF'],
//     'should both drop and transform W to F, when the value starts with SCH'
//   )

//   t.deepEqual(
//     m('Arnow'),
//     ['ARN', 'ARNF'],
//     'should both drop and transform W to F, when final and preceded by a vowel'
//   )

//   t.test(
//     'should transform W to TS and FX, when followed by ICZ or ITZ',
//     function(st) {
//       st.deepEqual(m('Filipowicz'), ['FLPTS', 'FLPFX'])
//       st.deepEqual(m('Filipowitz'), ['FLPTS', 'FLPFX'])

//       st.end()
//     }
//   )

//   t.equal(m('w')[0], '', 'should drop W')
//   t.equal(m('matrix')[0], 'MTRKS', 'should transform X to KS, when final')

//   t.test(
//     'should transform X to KS, when *NOT* preceded by IAU, EAU, AU, or OU',
//     function(st) {
//       st.equal(m('iauxa')[0], 'AKS')
//       st.equal(m('eauxa')[0], 'AKS')
//       st.equal(m('auxa')[0], 'AKS')
//       st.equal(m('ouxa')[0], 'AKS')

//       st.end()
//     }
//   )

//   t.equal(m('AUX')[0], 'A', 'should drop `UX` in `AUX`')
//   t.equal(m('OUX')[0], 'A', 'should drop `UX` in `OUX`')
//   t.equal(m('breaux')[0], 'PR', 'should drop `aux` in `breaux`')

//   t.equal(m('AXC')[0], 'AKS', 'should *not* drop XC')
//   t.equal(m('axx')[0], 'AKS', 'should *not* drop XX')
//   t.equal(m('axe')[0], 'AKS', 'should *not* drop X')
//   t.equal(m('zhao')[0], 'J', 'should transform ZH to J')

//   t.test(
//     'should transform Z to S and TS, when followed by ZA, ZI, or ZO',
//     function(st) {
//       st.deepEqual(m('zza'), ['S', 'TS'])
//       st.deepEqual(m('zzi'), ['S', 'TS'])
//       st.deepEqual(m('zzo'), ['S', 'TS'])

//       st.end()
//     }
//   )

//   t.deepEqual(
//     m('Mazurkiewicz'),
//     ['MSRKTS', 'MTSRKFX'],
//     'should transform Z to S and TS, when not initial, not Slavo-Germanic, and not preceded by T'
//   )

//   t.equal(m('zz')[0], 'S', 'should transform ZZ to S')
//   t.equal(m('z')[0], 'S', 'should transform Z to S')

//   t.end()
// })

// test('cli', function(t) {
//   var input = new PassThrough()
//   var helps = ['-h', '--help']
//   var versions = ['-v', '--version']

//   t.plan(7)

//   execa.stdout('./cli.js', ['michael']).then(function(result) {
//     t.equal(result, 'MKL	MXL', 'argument')
//   })

//   execa.stdout('./cli.js', ['detestable', 'vileness']).then(function(result) {
//     t.equal(result, 'TTSTPL\tTTSTPL FLNS\tFLNS', 'arguments')
//   })

//   execa.stdout('./cli.js', {input: input}).then(function(result) {
//     t.equal(result, 'TTSTPL\tTTSTPL FLNS\tFLNS', 'stdin')
//   })

//   input.write('detestable')

//   setImmediate(function() {
//     input.end(' vileness')
//   })

//   helps.forEach(function(flag) {
//     execa.stdout('./cli.js', [flag]).then(function(result) {
//       t.ok(/\s+Usage: double-metaphone/.test(result), flag)
//     })
//   })

//   versions.forEach(function(flag) {
//     execa.stdout('./cli.js', [flag]).then(function(result) {
//       t.equal(result, version, flag)
//     })
//   })
// })
