#[cfg(test)]
mod tests {
    #[test]
    fn hash() {
        assert_eq!(altv_sdk::hash("tyrant"), 0xE99011C2);
        assert_eq!(altv_sdk::hash("mp_m_freemode_01"), 0x705E61F2);
    }
}
