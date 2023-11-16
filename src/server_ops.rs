use crate::constants::ADD_MESSAGE_ID_ID;

pub fn is_update_subscriptions(message_id: &[u8]) -> bool {
    message_id.iter().zip(ADD_MESSAGE_ID_ID.iter()).all(|(a, b)| a == b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_id_is_update_sub() {
        let message_id = ADD_MESSAGE_ID_ID;
        assert!(is_update_subscriptions(message_id));
    }
}