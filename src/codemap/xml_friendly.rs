use super::Encoding;

pub const ENCODE_MAP: [u8; 91] = [
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
    b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
    b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
    b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'!', b'#',
    b'$', b'%', b'&', b'(', b')', b'*', b'+', b',', b'.', b'/', b':', b';', b'-', b'=', b'\\',
    b'?', b'@', b'[', b']', b'^', b'_', b'`', b'{', b'|', b'}', b'~', b'\'',
];

pub const DECODE_MAP: [u8; 256] = [
    91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91,
    91, 91, 91, 91, 91, 91, 91, 91, 91, 62, 91, 63, 64, 65, 66, 90, 67, 68, 69, 70, 71, 76, 72, 73,
    52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 74, 75, 91, 77, 91, 79, 80, 0, 1, 2, 3, 4, 5, 6, 7, 8,
    9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 81, 78, 82, 83, 84, 85, 26,
    27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
    51, 86, 87, 88, 89, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91,
    91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91,
    91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91,
    91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91,
    91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91,
    91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91,
];

pub struct XmlFriendly;

impl Encoding for XmlFriendly {
    #[inline(always)]
    fn encode_byte(key: u8) -> u8 {
        debug_assert!(key < 91);
        ENCODE_MAP[key as usize]
    }

    #[inline(always)]
    fn decode_byte(key: u8) -> u8 {
        DECODE_MAP[key as usize]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_integrity() {
        use super::XmlFriendly;
        use crate::iter::{DecodeGenericIterator, EncodeGenericIterator};

        let mut source: [u8; 1024] = [0; 1024];
        let mut bytes = (0..=255).cycle();

        for i in 0..1024 {
            source[i] = bytes.next().unwrap();
        }

        let encoded: String = EncodeGenericIterator::<XmlFriendly, _>::new(source.iter().copied())
            .as_char_iter()
            .collect();
        let decoded: Vec<u8> =
            DecodeGenericIterator::<XmlFriendly, _>::new(encoded.as_bytes().into_iter().copied())
                .collect();

        assert_eq!(&source, &decoded[..]);
        assert_eq!(
            ":C#(:C?hVB$MSiVEwndBAMZRxwFfBB;IW-}YQV!A_v$Y_c%zr4cYQPFl0,@heMAJ-:N[*T+/SFGr*`b4PD}vgYqU\\cW0P*1NwV,O{cQ5u0m900[8@n4,wh?DP-2+~jQSW6nmLm1o.J,?jTs%2-WF%qb=oh|}.C+W`EI!bv'XJ5KIV-G+aX]c[z$8)@aR67gb7p(`r4kHjOraEr8:A8y0G9KsDm7jpa{fh\\hT8%;@!9;s\\JX?#GT-W+vbf`A2a^wkFZCr-:V$}SR##&-^lr-Jn?_K5qh.JyLp+99&B_6vZ&x[uhn}L@sh3}g__~#A6Fa'*B*TTXFH*(aCQV|LgGr4\\uVQPz2xwn.)`KPDv+lZ0i@G[53jwP@nNE2i~RR2W-mCLU2-.b+rjBrL1EY7$YaXpz{0::+6`WHabdA8J;L!U6FQbp^]@h#G(Ic,5ph#77&xrmlrjgs@DZ7UB\\xQGrL@D48~oI|(fGj/7v:a#?:I@[VZ$YSoW0w#dNASa%x.FrBH-4UK~kQb#w;dm9=#m9`u58gkJgM=+?8eB&7^Z_wtuPmiKIu|2+fe^R%2`}%[qad2R;F(zp[#f.An;sMV,?)NU%G/)3c{Q#}=h_qy?}W]On2rx[,s|}Q|tW$AqI\\WVEPt1BwP.4`EQtuulN0c[w@h3XwJ[[O+1W~LQGW0o@KO1c.D+fl`r12+Xv$Sb%ob{o:)*K|}GObX'LJzLyU0G6aR^+@b$w):b75jgP7j&lrgm`iIs)DT8!A2xEGlKgDg8=oC{Th.i97p;)!3-{?;W(#AJOLX*H!_Aw6dYMwJw|m+Katz20g&^j$S`+$brscSR5EN07@bf2B;;!N;+9*xU^Fl)lbgR[|ph&r{=QWuOV3?vJ,I|+Phuom3zE[k@b4%x;@*O$2$'=R_VungM@0Q:|+/k9rp2$Y~%_a1oV|]/s,}{]H4c~A'1WlB0W]'`E:y+u`QI)aFAwJ*KOUiFEbj]i@J%_&CbX5Rhz71%BrOlfjargDB7IB.y6FTL)Dy7pq:{3h'!M;LlH\\[lZ`c4Gh2K{L7*Z9wAe7%Yex#v*lQLssP2ifM_1%._0%#r!d.QnFx0=[[ekAV-@Mn+r+`SLFB)Tc)PJ|Fhwqg\\iVKOD2Zwb.!{0Q*u6lT1-[.@t3dx5?PP]1c'7QeW$o{L!10.P+lkrs=2]X1%&b~on{u/a+i`KHUc,'jJ#L4TKH]ad^-[;$.){b%6^gn7v&rs@kTjUs:E(8{A&xKH`K4Ds8^py{rh_i(8~/C",
            encoded
        );
    }
}
