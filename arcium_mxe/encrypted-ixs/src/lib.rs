use arcis_imports::*;

#[encrypted]
mod circuits {
    use arcis_imports::*;

    pub struct WalletInputs {
        pub wallet_age_score: u8,  
        pub tx_count_score: u8,    
    }

    #[instruction]
    pub fn compute_score(input_ctxt: Enc<Shared, WalletInputs>) -> u8 {
        let input = input_ctxt.to_arcis();

        let weighted_score: u16 = (input.wallet_age_score as u16 * 50 / 100)
                                + (input.tx_count_score as u16 * 50 / 100);

        let score: u8 = if weighted_score > 100 { 100 } else { weighted_score as u8 };

        score.reveal()
    }
}