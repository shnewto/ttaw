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
    assert_eq!(double_metaphone("hugh").unwrap().get(0).unwrap(), "H")
}

#[test]
fn bough() {
    assert_eq!(double_metaphone("bough").unwrap().get(0).unwrap(), "P")
}

#[test]
fn broughton() {
    assert_eq!(
        double_metaphone("broughton").unwrap().get(0).unwrap(),
        "PRTN"
    )
}

#[test]
fn laugh() {
    assert_eq!(double_metaphone("laugh").unwrap().get(0).unwrap(), "LF")
}

#[test]
fn curagh() {
    assert_eq!(double_metaphone("curagh").unwrap().get(0).unwrap(), "KRK")
}

#[test]
fn weight() {
    assert_eq!(double_metaphone("weight").unwrap().get(0).unwrap(), "AT")
}

#[test]
fn agnize() {
    assert_eq!(
        double_metaphone("agnize").unwrap().get(0).unwrap().get(..3),
        Some("AKN")
    );

    assert_eq!(
        double_metaphone("agnize").unwrap().get(1).unwrap().get(..2),
        Some("AN")
    );
}

#[test]
fn tagliaro() {
    assert_eq!(
        double_metaphone("tagliaro").unwrap().get(0).unwrap(),
        "TKLR"
    );

    assert_eq!(double_metaphone("tagliaro").unwrap().get(1).unwrap(), "TLR");
}

#[test]
fn acceptingness() {
    assert!(double_metaphone("acceptingness")
        .unwrap()
        .get(0)
        .unwrap()
        .ends_with("NNS"));

    assert!(double_metaphone("acceptingness")
        .unwrap()
        .get(1)
        .unwrap()
        .ends_with("NKNS"));
}

#[test]
fn cagney() {
    assert_eq!(double_metaphone("cagney").unwrap().get(0).unwrap(), "KKN")
}

#[test]
fn gerben() {
    assert_eq!(
        double_metaphone("Gerben").unwrap().get(0).unwrap().get(..1),
        Some("K")
    );
    assert_eq!(
        double_metaphone("Gerben").unwrap().get(1).unwrap().get(..1),
        Some("J")
    );
}

#[test]
fn auger() {
    assert_eq!(
        double_metaphone("auger").unwrap().get(0).unwrap().get(1..2),
        Some("K")
    );
    assert_eq!(
        double_metaphone("auger").unwrap().get(1).unwrap().get(1..2),
        Some("J")
    );
}

#[test]
fn bulgy() {
    assert_eq!(
        double_metaphone("bulgy").unwrap().get(0).unwrap().get(2..3),
        Some("K")
    );
    assert_eq!(
        double_metaphone("bulgy").unwrap().get(1).unwrap().get(2..3),
        Some("J")
    );
}

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

#[test]
fn biaggi() {
    assert_eq!(
        double_metaphone("biaggi")
            .unwrap()
            .get(0)
            .unwrap()
            .get(1..2),
        Some("J")
    );
    assert_eq!(
        double_metaphone("biaggi")
            .unwrap()
            .get(1)
            .unwrap()
            .get(1..2),
        Some("K")
    );
}

#[test]
fn two_gs() {
    assert_eq!(double_metaphone("GG").unwrap().get(0).unwrap(), "K")
}

#[test]
fn one_g() {
    assert_eq!(double_metaphone("G").unwrap().get(0).unwrap(), "K")
}

#[test]
fn ha() {
    assert_eq!(double_metaphone("ha").unwrap().get(0).unwrap(), "H")
}

#[test]
fn aha() {
    assert_eq!(double_metaphone("aha").unwrap().get(0).unwrap(), "AH")
}

#[test]
fn one_h() {
    assert_eq!(double_metaphone("h").unwrap().get(0).unwrap(), "")
}

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

#[test]
fn jose() {
    assert_eq!(
        double_metaphone("Jose").unwrap().get(0).unwrap().get(..1),
        Some("H")
    )
}

#[test]
fn joseph() {
    assert_eq!(
        double_metaphone("Joseph").unwrap().get(0).unwrap().get(..1),
        Some("J")
    );
    assert_eq!(
        double_metaphone("Joseph").unwrap().get(1).unwrap().get(..1),
        Some("H")
    );
}

#[test]
fn jankelowicz() {
    assert_eq!(
        double_metaphone("Jankelowicz")
            .unwrap()
            .get(0)
            .unwrap()
            .get(..1),
        Some("J")
    );
    assert_eq!(
        double_metaphone("Jankelowicz")
            .unwrap()
            .get(1)
            .unwrap()
            .get(..1),
        Some("A")
    );
}

#[test]
fn bajador() {
    assert_eq!(
        double_metaphone("bajador")
            .unwrap()
            .get(0)
            .unwrap()
            .get(1..2),
        Some("J")
    );
    assert_eq!(
        double_metaphone("bajador")
            .unwrap()
            .get(1)
            .unwrap()
            .get(1..2),
        Some("H")
    );
}

#[test]
fn svaraj() {
    assert_eq!(double_metaphone("svaraj").unwrap().get(0).unwrap(), "SFRJ");
    assert_eq!(double_metaphone("svaraj").unwrap().get(1).unwrap(), "SFR");
}

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

#[test]
fn sjji() {
    assert_eq!(
        double_metaphone("sjji").unwrap().get(0).unwrap().get(..1),
        Some("S")
    )
}

#[test]
fn disject() {
    assert_eq!(double_metaphone("disject").unwrap().get(0).unwrap(), "TSKT")
}

#[test]
fn trekker() {
    assert_eq!(double_metaphone("trekker").unwrap().get(0).unwrap(), "TRKR")
}

#[test]
fn like() {
    assert_eq!(double_metaphone("like").unwrap().get(0).unwrap(), "LK")
}

#[test]
fn cabrillo() {
    assert_eq!(
        double_metaphone("cabrillo").unwrap().get(0).unwrap(),
        "KPRL"
    );
    assert_eq!(double_metaphone("cabrillo").unwrap().get(1).unwrap(), "KPR");
}

#[test]
fn villa() {
    assert_eq!(double_metaphone("villa").unwrap().get(0).unwrap(), "FL");
    assert_eq!(double_metaphone("villa").unwrap().get(1).unwrap(), "F");
}

#[test]
fn crevalle() {
    assert_eq!(
        double_metaphone("crevalle").unwrap().get(0).unwrap(),
        "KRFL"
    );
    assert_eq!(double_metaphone("crevalle").unwrap().get(1).unwrap(), "KRF");
}

#[test]
fn allegretto() {
    assert_eq!(
        double_metaphone("allegretto").unwrap().get(0).unwrap(),
        "ALKRT"
    );
    assert_eq!(
        double_metaphone("allegretto").unwrap().get(1).unwrap(),
        "AKRT"
    );
}

#[test]
fn allegros() {
    assert_eq!(
        double_metaphone("allegros").unwrap().get(0).unwrap(),
        "ALKRS"
    );
    assert_eq!(
        double_metaphone("allegros").unwrap().get(1).unwrap(),
        "AKRS"
    );
}

#[test]
fn two_lls() {
    assert_eq!(double_metaphone("ll").unwrap().get(0).unwrap(), "L")
}

#[test]
fn one_l() {
    assert_eq!(double_metaphone("l").unwrap().get(0).unwrap(), "L")
}

#[test]
fn thumb() {
    assert_eq!(double_metaphone("thumb").unwrap().get(0).unwrap(), "0M")
}

#[test]
fn dumber() {
    assert_eq!(double_metaphone("dumber").unwrap().get(0).unwrap(), "TMR")
}

#[test]
fn tow_mms() {
    assert_eq!(double_metaphone("mm").unwrap().get(0).unwrap(), "M")
}

#[test]
fn one_m() {
    assert_eq!(double_metaphone("m").unwrap().get(0).unwrap(), "M")
}

#[test]
fn two_nns() {
    assert_eq!(double_metaphone("nn").unwrap().get(0).unwrap(), "N")
}

#[test]
fn one_n() {
    assert_eq!(double_metaphone("n").unwrap().get(0).unwrap(), "N")
}

#[test]
fn top_tilda_n() {
    assert_eq!(double_metaphone("Ñ").unwrap().get(0).unwrap(), "N")
}

#[test]
fn ph() {
    assert_eq!(double_metaphone("ph").unwrap().get(0).unwrap(), "F")
}

#[test]
fn pb() {
    assert_eq!(double_metaphone("pb").unwrap().get(0).unwrap(), "P")
}

#[test]
fn twp_pps() {
    assert_eq!(double_metaphone("pp").unwrap().get(0).unwrap(), "P")
}

#[test]
fn one_p() {
    assert_eq!(double_metaphone("p").unwrap().get(0).unwrap(), "P")
}

#[test]
fn two_qqs() {
    assert_eq!(double_metaphone("qq").unwrap().get(0).unwrap(), "K")
}

#[test]
fn one_q() {
    assert_eq!(double_metaphone("q").unwrap().get(0).unwrap(), "K")
}

#[test]
fn xavier_drop_r() {
    assert_eq!(double_metaphone("Xavier").unwrap().get(0).unwrap(), "SF");
    assert_eq!(double_metaphone("Xavier").unwrap().get(1).unwrap(), "SFR");
}

#[test]
fn two_rrs() {
    assert_eq!(double_metaphone("rr").unwrap().get(0).unwrap(), "R")
}

#[test]
fn one_r() {
    assert_eq!(double_metaphone("r").unwrap().get(0).unwrap(), "R")
}

#[test]
fn island() {
    assert_eq!(double_metaphone("island").unwrap().get(0).unwrap(), "ALNT")
}

#[test]
fn sugar() {
    assert_eq!(
        double_metaphone("sugar").unwrap().get(0).unwrap().get(..1),
        Some("X")
    );
    assert_eq!(
        double_metaphone("sugar").unwrap().get(1).unwrap().get(..1),
        Some("S")
    );
}

#[test]
fn sholz() {
    assert_eq!(
        double_metaphone("Sholz").unwrap().get(0).unwrap().get(..1),
        Some("S")
    )
}

#[test]
fn sh() {
    assert_eq!(
        double_metaphone("sh").unwrap().get(0).unwrap().get(..1),
        Some("X")
    )
}

#[test]
fn sio() {
    assert_eq!(
        double_metaphone("sio").unwrap().get(0).unwrap().get(..1),
        Some("S")
    );
    assert_eq!(
        double_metaphone("sio").unwrap().get(1).unwrap().get(..1),
        Some("X")
    );
}

#[test]
fn sioricz() {
    assert_eq!(double_metaphone("sioricz").unwrap().get(0).unwrap(), "SRS");
    assert_eq!(double_metaphone("sioricz").unwrap().get(1).unwrap(), "SRX");
}

#[test]
fn sz() {
    assert_eq!(double_metaphone("sz").unwrap().get(0).unwrap(), "S");
    assert_eq!(double_metaphone("sz").unwrap().get(1).unwrap(), "X");
}

#[test]
fn sl() {
    assert_eq!(double_metaphone("sl").unwrap().get(0).unwrap(), "SL");
    assert_eq!(double_metaphone("sl").unwrap().get(1).unwrap(), "XL");
}

#[test]
fn schenker() {
    assert_eq!(
        double_metaphone("schenker").unwrap().get(0).unwrap(),
        "XNKR"
    );
    assert_eq!(
        double_metaphone("schenker").unwrap().get(1).unwrap(),
        "SKNKR"
    );
}

#[test]
fn schooner() {
    assert_eq!(
        double_metaphone("schooner").unwrap().get(0).unwrap(),
        "SKNR"
    );
    assert_eq!(
        double_metaphone("schooner").unwrap().get(1).unwrap(),
        "SKNR"
    );
}

#[test]
fn schlepp() {
    assert_eq!(double_metaphone("schlepp").unwrap().get(0).unwrap(), "XLP");
    assert_eq!(double_metaphone("schlepp").unwrap().get(1).unwrap(), "SLP");
}

#[test]
fn borscht() {
    assert_eq!(double_metaphone("borscht").unwrap().get(0).unwrap(), "PRXT")
}

#[test]
fn sci() {
    assert_eq!(double_metaphone("sci").unwrap().get(0).unwrap(), "S")
}

#[test]
fn scu() {
    assert_eq!(double_metaphone("scu").unwrap().get(0).unwrap(), "SK")
}

#[test]
fn ois() {
    assert_eq!(double_metaphone("ois").unwrap().get(0).unwrap(), "A");
    assert_eq!(double_metaphone("ois").unwrap().get(1).unwrap(), "AS");
}

#[test]
fn two_sss() {
    assert_eq!(double_metaphone("ss").unwrap().get(0).unwrap(), "S")
}

#[test]
fn one_s() {
    assert_eq!(double_metaphone("s").unwrap().get(0).unwrap(), "S")
}

#[test]
fn tion() {
    assert_eq!(double_metaphone("tion").unwrap().get(0).unwrap(), "XN")
}

#[test]
fn tia() {
    assert_eq!(double_metaphone("tia").unwrap().get(0).unwrap(), "X")
}

#[test]
fn tch() {
    assert_eq!(double_metaphone("tch").unwrap().get(0).unwrap(), "X")
}

#[test]
fn thom() {
    assert_eq!(double_metaphone("thom").unwrap().get(0).unwrap(), "TM")
}

#[test]
fn tham() {
    assert_eq!(double_metaphone("tham").unwrap().get(0).unwrap(), "TM")
}

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

#[test]
fn th() {
    assert_eq!(double_metaphone("th").unwrap().get(0).unwrap(), "0");
    assert_eq!(double_metaphone("th").unwrap().get(1).unwrap(), "T");
}

#[test]
fn two_tts() {
    assert_eq!(double_metaphone("tt").unwrap().get(0).unwrap(), "T")
}

#[test]
fn td() {
    assert_eq!(double_metaphone("td").unwrap().get(0).unwrap(), "T")
}

#[test]
fn one_t() {
    assert_eq!(double_metaphone("t").unwrap().get(0).unwrap(), "T")
}

#[test]
fn two_vvs() {
    assert_eq!(double_metaphone("vv").unwrap().get(0).unwrap(), "F")
}

#[test]
fn one_v() {
    assert_eq!(double_metaphone("v").unwrap().get(0).unwrap(), "F")
}

#[test]
fn awr() {
    assert_eq!(double_metaphone("awr").unwrap().get(0).unwrap(), "AR")
}

#[test]
fn wa() {
    assert_eq!(double_metaphone("wa").unwrap().get(0).unwrap(), "A");
    assert_eq!(double_metaphone("wa").unwrap().get(1).unwrap(), "F");
}

#[test]
fn wh() {
    assert_eq!(double_metaphone("wh").unwrap().get(0).unwrap(), "A")
}

#[test]
fn tsjaikowski() {
    assert_eq!(
        double_metaphone("Tsjaikowski").unwrap().get(0).unwrap(),
        "TSKSK"
    );
    assert_eq!(
        double_metaphone("Tsjaikowski").unwrap().get(1).unwrap(),
        "TSKFSK"
    );
}

#[test]
fn tsjaikowsky() {
    assert_eq!(
        double_metaphone("Tsjaikowsky").unwrap().get(0).unwrap(),
        "TSKSK"
    );
    assert_eq!(
        double_metaphone("Tsjaikowsky").unwrap().get(1).unwrap(),
        "TSKFSK"
    );
}

#[test]
fn schwa() {
    assert_eq!(double_metaphone("schwa").unwrap().get(0).unwrap(), "X");
    assert_eq!(double_metaphone("schwa").unwrap().get(1).unwrap(), "XF");
}

#[test]
fn arnow() {
    assert_eq!(double_metaphone("Arnow").unwrap().get(0).unwrap(), "ARN");
    assert_eq!(double_metaphone("Arnow").unwrap().get(1).unwrap(), "ARNF");
}

#[test]
fn filipowicz() {
    assert_eq!(
        double_metaphone("Filipowicz").unwrap().get(0).unwrap(),
        "FLPTS"
    );
    assert_eq!(
        double_metaphone("Filipowicz").unwrap().get(1).unwrap(),
        "FLPFX"
    );
}

#[test]
fn filipowitz() {
    assert_eq!(
        double_metaphone("Filipowitz").unwrap().get(0).unwrap(),
        "FLPTS"
    );
    assert_eq!(
        double_metaphone("Filipowitz").unwrap().get(1).unwrap(),
        "FLPFX"
    );
}

#[test]
fn w() {
    assert_eq!(double_metaphone("w").unwrap().get(0).unwrap(), "")
}

#[test]
fn matrix() {
    assert_eq!(double_metaphone("matrix").unwrap().get(0).unwrap(), "MTRKS")
}

#[test]
fn iauxa() {
    assert_eq!(double_metaphone("iauxa").unwrap().get(0).unwrap(), "AKS")
}

#[test]
fn eauxa() {
    assert_eq!(double_metaphone("eauxa").unwrap().get(0).unwrap(), "AKS")
}

#[test]
fn auxa() {
    assert_eq!(double_metaphone("auxa").unwrap().get(0).unwrap(), "AKS")
}

#[test]
fn ouxa() {
    assert_eq!(double_metaphone("ouxa").unwrap().get(0).unwrap(), "AKS")
}

#[test]
fn aux() {
    assert_eq!(double_metaphone("AUX").unwrap().get(0).unwrap(), "A")
}

#[test]
fn oux() {
    assert_eq!(double_metaphone("OUX").unwrap().get(0).unwrap(), "A")
}

#[test]
fn breaux() {
    assert_eq!(double_metaphone("breaux").unwrap().get(0).unwrap(), "PR")
}

#[test]
fn axc() {
    assert_eq!(double_metaphone("AXC").unwrap().get(0).unwrap(), "AKS")
}

#[test]
fn axx() {
    assert_eq!(double_metaphone("axx").unwrap().get(0).unwrap(), "AKS")
}

#[test]
fn axe() {
    assert_eq!(double_metaphone("axe").unwrap().get(0).unwrap(), "AKS")
}

#[test]
fn zhao() {
    assert_eq!(double_metaphone("zhao").unwrap().get(0).unwrap(), "J")
}

#[test]
fn zza() {
    assert_eq!(double_metaphone("zza").unwrap().get(0).unwrap(), "S");
    assert_eq!(double_metaphone("zza").unwrap().get(1).unwrap(), "TS");
}

#[test]
fn zzi() {
    assert_eq!(double_metaphone("zzi").unwrap().get(0).unwrap(), "S");
    assert_eq!(double_metaphone("zzi").unwrap().get(1).unwrap(), "TS");
}

#[test]
fn zzo() {
    assert_eq!(double_metaphone("zzo").unwrap().get(0).unwrap(), "S");
    assert_eq!(double_metaphone("zzo").unwrap().get(1).unwrap(), "TS");
}

#[test]
fn mazurkiewicz() {
    assert_eq!(
        double_metaphone("Mazurkiewicz").unwrap().get(0).unwrap(),
        "MSRKTS"
    );
    assert_eq!(
        double_metaphone("Mazurkiewicz").unwrap().get(1).unwrap(),
        "MTSRKFX"
    );
}

#[test]
fn two_zzs() {
    assert_eq!(double_metaphone("zz").unwrap().get(0).unwrap(), "S")
}

#[test]
fn one_z() {
    assert_eq!(double_metaphone("z").unwrap().get(0).unwrap(), "S");
}

#[test]
fn michael_full() {
    assert_eq!(double_metaphone("michael").unwrap().get(0).unwrap(), "MKL");
    assert_eq!(double_metaphone("michael").unwrap().get(1).unwrap(), "MXL");
}

#[test]
fn detestable() {
    assert_eq!(
        double_metaphone("detestable").unwrap().get(0).unwrap(),
        "TTSTPL"
    );
    assert_eq!(
        double_metaphone("detestable").unwrap().get(1).unwrap(),
        "TTSTPL"
    );
}

#[test]
fn vileness() {
    assert_eq!(
        double_metaphone("vileness").unwrap().get(0).unwrap(),
        "FLNS"
    );
    assert_eq!(
        double_metaphone("vileness").unwrap().get(1).unwrap(),
        "FLNS"
    );
}
