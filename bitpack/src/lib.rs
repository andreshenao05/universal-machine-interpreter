pub mod bitpack;
pub use crate::bitpack::fitss;
pub use crate::bitpack::fitsu;
pub use crate::bitpack::gets;
pub use crate::bitpack::getu;
pub use crate::bitpack::news;
pub use crate::bitpack::newu;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_fitsu() {
        assert!(fitsu(0, 0));
        assert!(!fitsu(1, 0));
        assert!(fitsu(7, 3));
        assert!(!fitsu(8, 3));
    }

    #[test]
    fn test_fitss() {
        assert!(fitss(0, 0));
        assert!(!fitss(1, 0));
        assert!(fitss(15, 5));
        assert!(fitss(-16, 5));
        assert!(!fitss(16, 5));
        assert!(!fitss(-17, 5));
    }

    #[test]
    fn test_getu() {
        let word = 0b110110u64;
        assert_eq!(getu(word, 3, 1), Some(0b011));
        assert_eq!(getu(word, 2, 4), Some(0b11));
        assert_eq!(getu(word, 0, 5), Some(0));
    }

    #[test]
    fn test_gets() {
        let word = 0b11101u64;
        assert_eq!(gets(word, 5, 0), Some(-3));

        let word2 = 0b00101u64;
        assert_eq!(gets(word2, 5, 0), Some(5));
    }

    #[test]
    fn test_newu_getu_roundtrip() {
        let word = 0u64;
        let packed = newu(word, 5, 3, 17).unwrap();
        assert_eq!(getu(packed, 5, 3), Some(17));
    }

    #[test]
    fn test_news_gets_roundtrip() {
        let word = 0u64;
        let packed = news(word, 5, 3, -3).unwrap();
        assert_eq!(gets(packed, 5, 3), Some(-3));
    }

    #[test]
    fn test_invalid_field() {
        assert_eq!(getu(0, 10, 60), None);
        assert_eq!(gets(0, 10, 60), None);
        assert_eq!(newu(0, 10, 60, 3), None);
        assert_eq!(news(0, 10, 60, -2), None);
    }

    #[test]
    fn test_value_does_not_fit() {
        assert_eq!(newu(0, 3, 0, 8), None);
        assert_eq!(news(0, 5, 0, 16), None);
        assert_eq!(news(0, 5, 0, -17), None);
    }

    #[test]
    fn test_other_bits_unchanged() {
        let word = 0b1111111111u64;
        let packed = newu(word, 3, 2, 0).unwrap();
        assert_eq!(getu(packed, 3, 2), Some(0));
        assert_eq!(getu(packed, 2, 0), Some(0b11));
        assert_eq!(getu(packed, 5, 5), Some(0b11111));
    }
}
