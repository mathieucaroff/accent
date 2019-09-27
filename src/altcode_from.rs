use crate::structure::AltCode;

impl<'a> AltCode<'a> {
    #[allow(dead_code)]
    pub fn from_accent0(chr: char) -> Option<AltCode<'a>> {
        match match chr {
            '€' => "0128", // euro
            'Œ' => "0140",  // OE ligature
            '–' => "0150", // en dash
            '―' => "0151", // em dash
            'œ' => "0156",  // oe ligature
            'Æ' => "0198",  // AE ligature

            'À' => "0192", // A grave
            'Á' => "0193", // A acute
            'Â' => "0194", // A circumflex
            'Ã' => "0195", // A tilde
            'Ä' => "0196", // A umlaut
            'Ç' => "0199", // C cedilla
            'È' => "0200", // E grave
            'É' => "0201", // E acute
            'Ê' => "0202", // E circumflex
            'Ë' => "0203", // E umlaut
            'Ì' => "0204", // I grave
            'Í' => "0205", // I acute
            'Î' => "0206", // I circumflex
            'Ï' => "0207", // I umlaut
            'Ñ' => "165",  // N tilde
            'Ò' => "0210", // O grave
            'Ó' => "0211", // O acute
            'Ô' => "0212", // O circumflex
            'Õ' => "0213", // O tilde
            'Ö' => "0214", // O umlaut
            'Š' => "0138", // S caron
            'Ú' => "0218", // U acute
            'Û' => "0219", // U circumflex
            'Ü' => "0220", // U umlaut
            'Ù' => "0217", // U grave
            'Ý' => "0221", // Y acute
            'Ÿ' => "0159", // Y umlaut
            'Ž' => "0142", // Z caron

            'ß' => "0223", // german double s

            'à' => "0224", // a grave
            'á' => "0225", // a acute
            'â' => "0226", // a circumflex
            'ã' => "0227", // a tilde
            'ä' => "0228", // a umlaut
            'ç' => "0231", // c cedilla
            'è' => "0232", // e grave
            'é' => "0233", // e acute
            'ê' => "0234", // e circumflex
            'ë' => "0235", // e umlaut
            'ì' => "0236", // i grave
            'í' => "0237", // i acute
            'î' => "0238", // i circumflex
            'ï' => "0239", // i umlaut
            'ñ' => "164",  // n tilde
            'ò' => "0242", // o grave
            'ó' => "0243", // o acute
            'ô' => "0244", // o circumflex
            'õ' => "0245", // o tilde
            'ö' => "0246", // o umlaut
            'š' => "0154", // s caron
            'ù' => "0249", // u grave
            'ú' => "0250", // u acute
            'û' => "0251", // u circumflex
            'ü' => "0252", // u umlaut
            'ý' => "0253", // y acute
            'ÿ' => "0255", // y umlaut
            'ž' => "0158", // z caron
            _ => "",
        } {
            "" => None,
            code => Some(AltCode::Digits(code)),
        }
    }
    #[allow(dead_code)]
    pub fn from_accent1(chr: char) -> Option<AltCode<'a>> {
        // r"^.\n\t\n\n\d+.*\n(\t\t\n)?\n"
        // ""

        // r"(?:^|(?<=\s))(\S)\n\t\n\nALT\+([\d|A]+)\n(\t\t\n)?\n?"
        // "            '$1' => "$2",\n"

        match match chr {
            'Ã' => "0195",
            'Å' => "143",
            'å' => "134", // "0229"
            'Ä' => "142",
            'ä' => "132",
            'À' => "0192",
            'à' => "133",
            'Á' => "0193",
            'á' => "160",
            'Â' => "0194",
            'â' => "131",
            'Ç' => "128",
            'ç' => "135",
            'É' => "144",
            'é' => "130",
            'È' => "0200",
            'è' => "138",
            'Ê' => "202",
            'ê' => "136",
            'Ë' => "203",
            'ë' => "137",
            'Ï' => "0207",
            'ï' => "139",
            'Î' => "0206",
            'î' => "140",
            'Í' => "0205",
            'í' => "161",
            'Ì' => "0204",
            'ì' => "141",
            'Ñ' => "165",
            'ñ' => "164",
            'Ö' => "153",
            'ö' => "148",
            'Ô' => "212",
            'ô' => "147",
            'Ò' => "0210",
            'ò' => "149",
            'Ó' => "0211",
            'ó' => "162",
            'Ø' => "0216",
            'Ü' => "154",
            'ü' => "129",
            'Ū' => "016A",
            'Û' => "0219",
            'û' => "150",
            'Ù' => "0217",
            'ù' => "151",
            'ú' => "163",
            'ÿ' => "152",
            'Ý' => "0221",  // Y acute
            'ý' => "0253",  // y acute
            'ß' => "225",   // german double s "0223"
            'Œ' => "0140",  // OE ligature
            'œ' => "0156",  // oe ligature
            '€' => "0128", // euro
            'æ' => "0230",  // ae ligature
            'Æ' => "0198",  // AE ligature
            '–' => "0150", // en dash
            '―' => "0151", // em dash
            _ => "",
        } {
            "" => None,
            code => Some(AltCode::Digits(code)),
        }
    }
}
