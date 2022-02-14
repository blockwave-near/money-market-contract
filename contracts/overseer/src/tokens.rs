use crate::*;

pub type Token = (AccountId, Balance);
pub type Tokens = Vec<Token>;

pub trait TokensMath {
    fn sub(&mut self, collaterals: Tokens);
    fn add(&mut self, collaterals: Tokens);
    fn assert_duplicate_token(&self);
}

impl TokensMath for Tokens {
    fn sub(&mut self, tokens: Tokens) {
        self.sort_by(|a, b| a.0.cmp(&b.0));
        self.assert_duplicate_token();

        let mut tokens = tokens;
        tokens.sort_by(|a, b| a.0.cmp(&b.0));
        tokens.assert_duplicate_token();

        let mut i = 0;
        let mut j = 0;
        while i < self.len() && j < tokens.len() {
            if self[i].0 == tokens[j].0 {
                if self[i].1 < tokens[j].1 {
                    env::panic("Subtraction underflow".as_bytes());
                }

                self[i].1 = self[i].1 - tokens[j].1;

                i += 1;
                j += 1;
            } else if self[i].0.cmp(&tokens[j].0) == std::cmp::Ordering::Less {
                i += 1;
            } else {
                env::panic("Subtraction underflow".as_bytes());
            }
        }

        if j != tokens.len() {
            env::panic("Subtraction underflow".as_bytes());
        }

        // remove zero tokens
        self.retain(|v| v.1 > 0);
    }

    fn add(&mut self, tokens: Tokens) {
        self.sort_by(|a, b| a.0.cmp(&b.0));
        self.assert_duplicate_token();

        let mut tokens = tokens;
        tokens.sort_by(|a, b| a.0.cmp(&b.0));
        tokens.assert_duplicate_token();

        let mut tmp_tokens: Tokens = vec![];

        let mut i = 0;
        let mut j = 0;
        while i < self.len() && j < tokens.len() {
            if self[i].0 == tokens[j].0 {
                tmp_tokens.push((self[i].0.clone(), self[i].1 + tokens[j].1));

                i += 1;
                j += 1;
            } else if self[i].0.cmp(&tokens[j].0) == std::cmp::Ordering::Greater {
                tmp_tokens.push((tokens[j].0.clone(), tokens[j].1));

                j += 1;
            } else {
                tmp_tokens.push((self[i].0.clone(), self[i].1));

                i += 1;
            }
        }

        while j < tokens.len() {
            tmp_tokens.push((tokens[j].0.clone(), tokens[j].1));
            j += 1;
        }

        while i < self.len() {
            tmp_tokens.push((self[i].0.clone(), self[i].1));
            i += 1;
        }

        // remove zero tokens
        tmp_tokens.retain(|v| v.1 > 0);

        self.clear();
        self.extend(tmp_tokens);
    }

    fn assert_duplicate_token(&self) {
        if self.len() > 1 {
            let mut before_token = self[0].0.clone();

            let mut i = 1;
            while i < self.len() {
                let next_token = self[i].0.clone();
                if before_token == next_token {
                    panic!("duplicate token address");
                }

                before_token = next_token;
                i += 1;
            }
        }
    }
}
