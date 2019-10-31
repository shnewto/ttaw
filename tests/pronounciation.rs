extern crate ttaw;

use ttaw::pronounciation::{double_metaphone, DoubleMetaphone};

#[test]
fn ptah() {
    assert_eq!(
        double_metaphone("ptah"),
        DoubleMetaphone {
            primary: "PT".to_string(),
            secondary: "PT".to_string()
        }
    )
}

#[test]
fn ceasar() {
    assert_eq!(
        double_metaphone("ceasar"),
        DoubleMetaphone {
            primary: "SSR".to_string(),
            secondary: "SSR".to_string()
        }
    )
}

#[test]
fn ach() {
    assert_eq!(
        double_metaphone("ach"),
        DoubleMetaphone {
            primary: "AK".to_string(),
            secondary: "AK".to_string()
        }
    )
}

#[test]
fn chemical() {
    assert_eq!(
        double_metaphone("chemical"),
        DoubleMetaphone {
            primary: "KMKL".to_string(),
            secondary: "KMKL".to_string()
        }
    )
}

#[test]
fn choral() {
    assert_eq!(
        double_metaphone("choral"),
        DoubleMetaphone {
            primary: "KRL".to_string(),
            secondary: "KRL".to_string()
        }
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
    assert_eq!(double_metaphone("gnarl").primary.get(..1), Some("N"));
}

#[test]
fn knack() {
    assert_eq!(double_metaphone("knack").primary.get(..1), Some("N"));
}

#[test]
fn pneumatic() {
    assert_eq!(double_metaphone("pneumatic").primary.get(..1), Some("N"));
}

#[test]
fn wrack() {
    assert_eq!(double_metaphone("wrack").primary.get(..1), Some("R"));
}

#[test]
fn psycho() {
    assert_eq!(double_metaphone("psycho").primary.get(..1), Some("S"));
}

#[test]
fn xavier() {
    assert_eq!(double_metaphone("Xavier").primary.get(..1), Some("S"));
}

#[test]
fn vowels() {
    assert_eq!(double_metaphone("a").primary, "A");
    assert_eq!(double_metaphone("e").primary, "A");
    assert_eq!(double_metaphone("i").primary, "A");
    assert_eq!(double_metaphone("o").primary, "A");
    assert_eq!(double_metaphone("u").primary, "A");
    assert_eq!(double_metaphone("y").primary, "A");
}

#[test]
fn drop_initial() {
    assert_eq!(double_metaphone("ba").primary.len(), 1);
    assert_eq!(double_metaphone("be").primary.len(), 1);
    assert_eq!(double_metaphone("bi").primary.len(), 1);
    assert_eq!(double_metaphone("bo").primary.len(), 1);
    assert_eq!(double_metaphone("bu").primary.len(), 1);
    assert_eq!(double_metaphone("by").primary.len(), 1);
}

#[test]
fn b_to_p() {
    assert_eq!(double_metaphone("b").primary.get(..1), Some("P"));
    assert_eq!(double_metaphone("bb").primary.get(..1), Some("P"));
}

#[test]
fn c_cedilla_to_s() {
    assert_eq!(double_metaphone("Ç").primary.get(..1), Some("S"));
}

#[test]
fn when_c_to_k() {
    assert_eq!(double_metaphone("ACH").primary.get(1..2), Some("K"));
    assert_ne!(double_metaphone("AACH").primary.get(2..3), Some("K"));
    assert_ne!(double_metaphone("ACHI").primary.get(1..2), Some("K"));
    assert_eq!(double_metaphone("ACHB").primary.get(1..2), Some("K"));
    assert_eq!(double_metaphone("MACHER").secondary.get(1..2), Some("K"));
    assert_eq!(double_metaphone("BACHER").secondary.get(1..2), Some("K"));
}

#[test]
fn caesar() {
    assert_eq!(double_metaphone("CAESAR").primary.get(..1), Some("S"));
}

#[test]
fn chianti() {
    assert_eq!(double_metaphone("chianti").primary.get(..1), Some("K"));
}

#[test]
fn michael() {
    assert_eq!(double_metaphone("michael").primary.get(1..2), Some("K"));

    assert_eq!(double_metaphone("michael").secondary.get(1..2), Some("X"));
}

#[test]
fn chiastic() {
    assert_eq!(double_metaphone("chiastic").primary.get(..1), Some("K"));
}

#[test]
fn chemical_c_to_k() {
    assert_eq!(double_metaphone("chemical").primary.get(..1), Some("K"));
}

#[test]
fn choral_c_to_k() {
    assert_eq!(double_metaphone("choral").primary.get(..1), Some("K"));
}

#[test]
fn chyme_c_to_k() {
    assert_eq!(double_metaphone("chyme").primary.get(..1), Some("K"));
}

#[test]
fn character_c_to_k() {
    assert_eq!(double_metaphone("character").primary.get(..1), Some("K"));
}

#[test]
fn charisma_c_to_k() {
    assert_eq!(double_metaphone("charisma").primary.get(..1), Some("K"));
}

#[test]
fn von_ch_c_to_k() {
    assert_eq!(double_metaphone("von ch").primary.get(2..3), Some("K"));
}

#[test]
fn schooner_c_to_k() {
    assert_eq!(double_metaphone("schooner").primary.get(1..2), Some("K"));
}

#[test]
fn orchestra_c_to_k() {
    assert_eq!(double_metaphone("orchestra").primary.get(2..3), Some("K"));
}

#[test]
fn architect_c_to_k() {
    assert_eq!(double_metaphone("architect").primary.get(2..3), Some("K"));
}

#[test]
fn arch_not_c_to_k() {
    assert_ne!(double_metaphone("arch").primary.get(2..3), Some("K"));
}

#[test]
fn orchid_c_to_k() {
    assert_eq!(double_metaphone("orchid").primary.get(2..3), Some("K"));
}

#[test]
fn chthonic_c_to_k() {
    assert_eq!(double_metaphone("chthonic").primary.get(..1), Some("K"));
}

#[test]
fn fuchsia_c_to_k() {
    assert_eq!(double_metaphone("fuchsia").primary.get(1..2), Some("K"));
}

#[test]
fn chloride_c_to_k() {
    assert_eq!(double_metaphone("chloride").primary.get(..1), Some("K"));
}

#[test]
fn chroma_c_to_k() {
    assert_eq!(double_metaphone("chroma").primary.get(..1), Some("K"));
}

#[test]
fn tichner_c_to_k() {
    assert_eq!(double_metaphone("tichner").secondary.get(1..2), Some("K"));
}

#[test]
fn mchugh_c_to_k() {
    assert_eq!(double_metaphone("McHugh").primary.get(1..2), Some("K"));
}

#[test]
fn chore() {
    assert_eq!(double_metaphone("chore").primary.get(..1), Some("X"));
}

#[test]
fn h_after_c() {
    assert_eq!(double_metaphone("achievement").primary.get(1..2), Some("X"));

    assert_eq!(
        double_metaphone("achievement").secondary.get(1..2),
        Some("K")
    );
}

#[test]
fn czerny() {
    assert_eq!(double_metaphone("czerny").primary.get(..1), Some("S"));

    assert_eq!(double_metaphone("czerny").secondary.get(..1), Some("X"));
}

#[test]
fn focaccia() {
    assert_eq!(double_metaphone("focaccia").primary.get(2..3), Some("X"));
}

#[test]
fn accident() {
    assert_eq!(double_metaphone("accident").primary.get(1..2), Some("K"));
    assert_eq!(double_metaphone("accident").primary.get(2..3), Some("S"));
}

#[test]
fn accede() {
    assert_eq!(double_metaphone("accede").primary.get(1..2), Some("K"));
    assert_eq!(double_metaphone("accede").primary.get(2..3), Some("S"));
}

#[test]
fn succeed() {
    assert_eq!(double_metaphone("succeed").primary.get(1..2), Some("K"));
    assert_eq!(double_metaphone("succeed").primary.get(2..3), Some("S"));
}

#[test]
fn bertucci() {
    assert_eq!(double_metaphone("bertucci").primary.get(3..4), Some("X"));
}

#[test]
fn hiccups_c_to_k() {
    assert_eq!(double_metaphone("hiccups").primary.get(1..2), Some("K"));
}

#[test]
fn knack_c_to_k() {
    assert_eq!(double_metaphone("knack").primary.get(1..2), Some("K"));
}

#[test]
fn ancient() {
    assert_eq!(double_metaphone("ancient").primary.get(2..3), Some("S"));

    assert_eq!(double_metaphone("ancient").secondary.get(2..3), Some("X"));
}

#[test]
fn delicious() {
    assert_eq!(double_metaphone("delicious").primary.get(2..3), Some("S"));

    assert_eq!(double_metaphone("delicious").secondary.get(2..3), Some("X"));
}

#[test]
fn acicula() {
    assert_eq!(double_metaphone("acicula").primary.get(1..2), Some("S"));
}

#[test]
fn abduce() {
    assert_eq!(double_metaphone("abduce").primary.get(3..4), Some("S"));
}

#[test]
fn acyl() {
    assert_eq!(double_metaphone("acyl").primary.get(1..2), Some("S"));
}

#[test]
fn maccaffery() {
    assert_eq!(double_metaphone("Mac Caffrey").primary.get(1..2), Some("K"))
}

#[test]
fn macgregor() {
    assert_eq!(double_metaphone("Mac Gregor").primary.get(1..2), Some("K"))
}

#[test]
fn macquillan() {
    assert_eq!(double_metaphone("Mac Quillan").primary.get(1..2), Some("K"))
}

#[test]
fn aback() {
    assert_eq!(double_metaphone("aback").primary.get(2..3), Some("K"))
}

#[test]
fn acquit() {
    assert_eq!(double_metaphone("acquit").primary.get(1..2), Some("K"))
}

#[test]
fn acclimate() {
    assert_eq!(double_metaphone("acclimate").primary.get(1..2), Some("K"))
}

#[test]
fn edge() {
    assert_eq!(double_metaphone("edge").primary.get(1..2), Some("J"))
}

#[test]
fn pidgin() {
    assert_eq!(double_metaphone("pidgin").primary.get(1..2), Some("J"))
}

#[test]
fn edgy() {
    assert_eq!(double_metaphone("edgy").primary.get(1..2), Some("J"))
}

#[test]
fn edgar() {
    assert_eq!(double_metaphone("Edgar").primary.get(1..3), Some("TK"))
}

#[test]
fn width() {
    assert_eq!(double_metaphone("width").primary.get(1..2), Some("T"))
}

#[test]
fn add() {
    assert_eq!(double_metaphone("add").primary.get(1..2), Some("T"))
}

#[test]
fn abduce_slice() {
    assert_eq!(double_metaphone("Abduce").primary.get(2..3), Some("T"))
}

#[test]
fn affect() {
    assert_eq!(double_metaphone("affect").primary.get(1..2), Some("F"))
}

#[test]
fn abaft() {
    assert_eq!(double_metaphone("abaft").primary.get(2..3), Some("F"))
}

#[test]
fn aargh() {
    assert_eq!(double_metaphone("aargh").primary.get(2..3), Some("K"))
}

#[test]
fn ghislane() {
    assert_eq!(double_metaphone("ghislane").primary.get(..1), Some("J"))
}

#[test]
fn ghoul() {
    assert_eq!(double_metaphone("ghoul").primary.get(..1), Some("K"))
}

#[test]
fn hugh() {
    assert_eq!(double_metaphone("hugh").primary, "H")
}

#[test]
fn bough() {
    assert_eq!(double_metaphone("bough").primary, "P")
}

#[test]
fn broughton() {
    assert_eq!(double_metaphone("broughton").primary, "PRTN")
}

#[test]
fn laugh() {
    assert_eq!(double_metaphone("laugh").primary, "LF")
}

#[test]
fn curagh() {
    assert_eq!(double_metaphone("curagh").primary, "KRK")
}

#[test]
fn weight() {
    assert_eq!(double_metaphone("weight").primary, "AT")
}

#[test]
fn agnize() {
    assert_eq!(double_metaphone("agnize").primary.get(..3), Some("AKN"));

    assert_eq!(double_metaphone("agnize").secondary.get(..2), Some("AN"));
}

#[test]
fn tagliaro() {
    assert_eq!(double_metaphone("tagliaro").primary, "TKLR");

    assert_eq!(double_metaphone("tagliaro").secondary, "TLR");
}

#[test]
fn acceptingness() {
    assert!(double_metaphone("acceptingness").primary.ends_with("NNS"));

    assert!(double_metaphone("acceptingness")
        .secondary
        .ends_with("NKNS"));
}

#[test]
fn cagney() {
    assert_eq!(double_metaphone("cagney").primary, "KKN")
}

#[test]
fn gerben() {
    assert_eq!(double_metaphone("Gerben").primary.get(..1), Some("K"));
    assert_eq!(double_metaphone("Gerben").secondary.get(..1), Some("J"));
}

#[test]
fn auger() {
    assert_eq!(double_metaphone("auger").primary.get(1..2), Some("K"));
    assert_eq!(double_metaphone("auger").secondary.get(1..2), Some("J"));
}

#[test]
fn bulgy() {
    assert_eq!(double_metaphone("bulgy").primary.get(2..3), Some("K"));
    assert_eq!(double_metaphone("bulgy").secondary.get(2..3), Some("J"));
}

#[test]
fn altogether() {
    assert_eq!(double_metaphone("altogether").primary.get(3..4), Some("K"))
}

#[test]
fn vanagema() {
    assert_eq!(double_metaphone("Van Agema").primary.get(2..3), Some("K"))
}

#[test]
fn vongoggin() {
    assert_eq!(double_metaphone("Von Goggin").primary.get(3..4), Some("K"))
}

#[test]
fn tangier() {
    assert_eq!(double_metaphone("tangier").primary.get(2..3), Some("J"))
}

#[test]
fn biaggi() {
    assert_eq!(double_metaphone("biaggi").primary.get(1..2), Some("J"));
    assert_eq!(double_metaphone("biaggi").secondary.get(1..2), Some("K"));
}

#[test]
fn two_gs() {
    assert_eq!(double_metaphone("GG").primary, "K")
}

#[test]
fn one_g() {
    assert_eq!(double_metaphone("G").primary, "K")
}

#[test]
fn ha() {
    assert_eq!(double_metaphone("ha").primary, "H")
}

#[test]
fn aha() {
    assert_eq!(double_metaphone("aha").primary, "AH")
}

#[test]
fn one_h() {
    assert_eq!(double_metaphone("h").primary, "")
}

#[test]
fn sanjacinto() {
    assert_eq!(double_metaphone("San Jacinto").primary.get(2..3), Some("H"))
}

#[test]
fn jose() {
    assert_eq!(double_metaphone("Jose").primary.get(..1), Some("H"))
}

#[test]
fn joseph() {
    assert_eq!(double_metaphone("Joseph").primary.get(..1), Some("J"));
    assert_eq!(double_metaphone("Joseph").secondary.get(..1), Some("H"));
}

#[test]
fn jankelowicz() {
    assert_eq!(double_metaphone("Jankelowicz").primary.get(..1), Some("J"));
    assert_eq!(
        double_metaphone("Jankelowicz").secondary.get(..1),
        Some("A")
    );
}

#[test]
fn bajador() {
    assert_eq!(double_metaphone("bajador").primary.get(1..2), Some("J"));
    assert_eq!(double_metaphone("bajador").secondary.get(1..2), Some("H"));
}

#[test]
fn svaraj() {
    assert_eq!(double_metaphone("svaraj").primary, "SFRJ");
    assert_eq!(double_metaphone("svaraj").secondary, "SFR");
}

#[test]
fn abject() {
    assert_eq!(double_metaphone("abject").primary.get(2..3), Some("J"))
}

#[test]
fn sjji() {
    assert_eq!(double_metaphone("sjji").primary.get(..1), Some("S"))
}

#[test]
fn disject() {
    assert_eq!(double_metaphone("disject").primary, "TSKT")
}

#[test]
fn trekker() {
    assert_eq!(double_metaphone("trekker").primary, "TRKR")
}

#[test]
fn like() {
    assert_eq!(double_metaphone("like").primary, "LK")
}

#[test]
fn cabrillo() {
    assert_eq!(double_metaphone("cabrillo").primary, "KPRL");
    assert_eq!(double_metaphone("cabrillo").secondary, "KPR");
}

#[test]
fn villa() {
    assert_eq!(double_metaphone("villa").primary, "FL");
    assert_eq!(double_metaphone("villa").secondary, "F");
}

#[test]
fn crevalle() {
    assert_eq!(double_metaphone("crevalle").primary, "KRFL");
    assert_eq!(double_metaphone("crevalle").secondary, "KRF");
}

#[test]
fn allegretto() {
    assert_eq!(double_metaphone("allegretto").primary, "ALKRT");
    assert_eq!(double_metaphone("allegretto").secondary, "AKRT");
}

#[test]
fn allegros() {
    assert_eq!(double_metaphone("allegros").primary, "ALKRS");
    assert_eq!(double_metaphone("allegros").secondary, "AKRS");
}

#[test]
fn two_lls() {
    assert_eq!(double_metaphone("ll").primary, "L")
}

#[test]
fn one_l() {
    assert_eq!(double_metaphone("l").primary, "L")
}

#[test]
fn thumb() {
    assert_eq!(double_metaphone("thumb").primary, "0M")
}

#[test]
fn dumber() {
    assert_eq!(double_metaphone("dumber").primary, "TMR")
}

#[test]
fn tow_mms() {
    assert_eq!(double_metaphone("mm").primary, "M")
}

#[test]
fn one_m() {
    assert_eq!(double_metaphone("m").primary, "M")
}

#[test]
fn two_nns() {
    assert_eq!(double_metaphone("nn").primary, "N")
}

#[test]
fn one_n() {
    assert_eq!(double_metaphone("n").primary, "N")
}

#[test]
fn top_tilda_n() {
    assert_eq!(double_metaphone("Ñ").primary, "N")
}

#[test]
fn ph() {
    assert_eq!(double_metaphone("ph").primary, "F")
}

#[test]
fn pb() {
    assert_eq!(double_metaphone("pb").primary, "P")
}

#[test]
fn twp_pps() {
    assert_eq!(double_metaphone("pp").primary, "P")
}

#[test]
fn one_p() {
    assert_eq!(double_metaphone("p").primary, "P")
}

#[test]
fn two_qqs() {
    assert_eq!(double_metaphone("qq").primary, "K")
}

#[test]
fn one_q() {
    assert_eq!(double_metaphone("q").primary, "K")
}

#[test]
fn xavier_drop_r() {
    assert_eq!(double_metaphone("Xavier").primary, "SF");
    assert_eq!(double_metaphone("Xavier").secondary, "SFR");
}

#[test]
fn two_rrs() {
    assert_eq!(double_metaphone("rr").primary, "R")
}

#[test]
fn one_r() {
    assert_eq!(double_metaphone("r").primary, "R")
}

#[test]
fn island() {
    assert_eq!(double_metaphone("island").primary, "ALNT")
}

#[test]
fn sugar() {
    assert_eq!(double_metaphone("sugar").primary.get(..1), Some("X"));
    assert_eq!(double_metaphone("sugar").secondary.get(..1), Some("S"));
}

#[test]
fn sholz() {
    assert_eq!(double_metaphone("Sholz").primary.get(..1), Some("S"))
}

#[test]
fn sh() {
    assert_eq!(double_metaphone("sh").primary.get(..1), Some("X"))
}

#[test]
fn sio() {
    assert_eq!(double_metaphone("sio").primary.get(..1), Some("S"));
    assert_eq!(double_metaphone("sio").secondary.get(..1), Some("X"));
}

#[test]
fn sioricz() {
    assert_eq!(double_metaphone("sioricz").primary, "SRS");
    assert_eq!(double_metaphone("sioricz").secondary, "SRX");
}

#[test]
fn sz() {
    assert_eq!(double_metaphone("sz").primary, "S");
    assert_eq!(double_metaphone("sz").secondary, "X");
}

#[test]
fn sl() {
    assert_eq!(double_metaphone("sl").primary, "SL");
    assert_eq!(double_metaphone("sl").secondary, "XL");
}

#[test]
fn schenker() {
    assert_eq!(double_metaphone("schenker").primary, "XNKR");
    assert_eq!(double_metaphone("schenker").secondary, "SKNKR");
}

#[test]
fn schooner() {
    assert_eq!(double_metaphone("schooner").primary, "SKNR");
    assert_eq!(double_metaphone("schooner").secondary, "SKNR");
}

#[test]
fn schlepp() {
    assert_eq!(double_metaphone("schlepp").primary, "XLP");
    assert_eq!(double_metaphone("schlepp").secondary, "SLP");
}

#[test]
fn borscht() {
    assert_eq!(double_metaphone("borscht").primary, "PRXT")
}

#[test]
fn sci() {
    assert_eq!(double_metaphone("sci").primary, "S")
}

#[test]
fn scu() {
    assert_eq!(double_metaphone("scu").primary, "SK")
}

#[test]
fn ois() {
    assert_eq!(double_metaphone("ois").primary, "A");
    assert_eq!(double_metaphone("ois").secondary, "AS");
}

#[test]
fn two_sss() {
    assert_eq!(double_metaphone("ss").primary, "S")
}

#[test]
fn one_s() {
    assert_eq!(double_metaphone("s").primary, "S")
}

#[test]
fn tion() {
    assert_eq!(double_metaphone("tion").primary, "XN")
}

#[test]
fn tia() {
    assert_eq!(double_metaphone("tia").primary, "X")
}

#[test]
fn tch() {
    assert_eq!(double_metaphone("tch").primary, "X")
}

#[test]
fn thom() {
    assert_eq!(double_metaphone("thom").primary, "TM")
}

#[test]
fn tham() {
    assert_eq!(double_metaphone("tham").primary, "TM")
}

#[test]
fn vongoethals() {
    assert_eq!(
        double_metaphone("Von Goethals").primary.get(3..4),
        Some("T")
    )
}

#[test]
fn vonmatthes() {
    assert_eq!(double_metaphone("Von Matthes").primary.get(3..4), Some("T"))
}

#[test]
fn th() {
    assert_eq!(double_metaphone("th").primary, "0");
    assert_eq!(double_metaphone("th").secondary, "T");
}

#[test]
fn two_tts() {
    assert_eq!(double_metaphone("tt").primary, "T")
}

#[test]
fn td() {
    assert_eq!(double_metaphone("td").primary, "T")
}

#[test]
fn one_t() {
    assert_eq!(double_metaphone("t").primary, "T")
}

#[test]
fn two_vvs() {
    assert_eq!(double_metaphone("vv").primary, "F")
}

#[test]
fn one_v() {
    assert_eq!(double_metaphone("v").primary, "F")
}

#[test]
fn awr() {
    assert_eq!(double_metaphone("awr").primary, "AR")
}

#[test]
fn wa() {
    assert_eq!(double_metaphone("wa").primary, "A");
    assert_eq!(double_metaphone("wa").secondary, "F");
}

#[test]
fn wh() {
    assert_eq!(double_metaphone("wh").primary, "A")
}

#[test]
fn tsjaikowski() {
    assert_eq!(double_metaphone("Tsjaikowski").primary, "TSKSK");
    assert_eq!(double_metaphone("Tsjaikowski").secondary, "TSKFSK");
}

#[test]
fn tsjaikowsky() {
    assert_eq!(double_metaphone("Tsjaikowsky").primary, "TSKSK");
    assert_eq!(double_metaphone("Tsjaikowsky").secondary, "TSKFSK");
}

#[test]
fn schwa() {
    assert_eq!(double_metaphone("schwa").primary, "X");
    assert_eq!(double_metaphone("schwa").secondary, "XF");
}

#[test]
fn arnow() {
    assert_eq!(double_metaphone("Arnow").primary, "ARN");
    assert_eq!(double_metaphone("Arnow").secondary, "ARNF");
}

#[test]
fn filipowicz() {
    assert_eq!(double_metaphone("Filipowicz").primary, "FLPTS");
    assert_eq!(double_metaphone("Filipowicz").secondary, "FLPFX");
}

#[test]
fn filipowitz() {
    assert_eq!(double_metaphone("Filipowitz").primary, "FLPTS");
    assert_eq!(double_metaphone("Filipowitz").secondary, "FLPFX");
}

#[test]
fn w() {
    assert_eq!(double_metaphone("w").primary, "")
}

#[test]
fn matrix() {
    assert_eq!(double_metaphone("matrix").primary, "MTRKS")
}

#[test]
fn iauxa() {
    assert_eq!(double_metaphone("iauxa").primary, "AKS")
}

#[test]
fn eauxa() {
    assert_eq!(double_metaphone("eauxa").primary, "AKS")
}

#[test]
fn auxa() {
    assert_eq!(double_metaphone("auxa").primary, "AKS")
}

#[test]
fn ouxa() {
    assert_eq!(double_metaphone("ouxa").primary, "AKS")
}

#[test]
fn aux() {
    assert_eq!(double_metaphone("AUX").primary, "A")
}

#[test]
fn oux() {
    assert_eq!(double_metaphone("OUX").primary, "A")
}

#[test]
fn breaux() {
    assert_eq!(double_metaphone("breaux").primary, "PR")
}

#[test]
fn axc() {
    assert_eq!(double_metaphone("AXC").primary, "AKS")
}

#[test]
fn axx() {
    assert_eq!(double_metaphone("axx").primary, "AKS")
}

#[test]
fn axe() {
    assert_eq!(double_metaphone("axe").primary, "AKS")
}

#[test]
fn zhao() {
    assert_eq!(double_metaphone("zhao").primary, "J")
}

#[test]
fn zza() {
    assert_eq!(double_metaphone("zza").primary, "S");
    assert_eq!(double_metaphone("zza").secondary, "TS");
}

#[test]
fn zzi() {
    assert_eq!(double_metaphone("zzi").primary, "S");
    assert_eq!(double_metaphone("zzi").secondary, "TS");
}

#[test]
fn zzo() {
    assert_eq!(double_metaphone("zzo").primary, "S");
    assert_eq!(double_metaphone("zzo").secondary, "TS");
}

#[test]
fn mazurkiewicz() {
    assert_eq!(double_metaphone("Mazurkiewicz").primary, "MSRKTS");
    assert_eq!(double_metaphone("Mazurkiewicz").secondary, "MTSRKFX");
}

#[test]
fn two_zzs() {
    assert_eq!(double_metaphone("zz").primary, "S")
}

#[test]
fn one_z() {
    assert_eq!(double_metaphone("z").primary, "S");
}

#[test]
fn michael_full() {
    assert_eq!(double_metaphone("michael").primary, "MKL");
    assert_eq!(double_metaphone("michael").secondary, "MXL");
}

#[test]
fn detestable() {
    assert_eq!(double_metaphone("detestable").primary, "TTSTPL");
    assert_eq!(double_metaphone("detestable").secondary, "TTSTPL");
}

#[test]
fn vileness() {
    assert_eq!(double_metaphone("vileness").primary, "FLNS");
    assert_eq!(double_metaphone("vileness").secondary, "FLNS");
}
