//! NETSHOPPING native functions
//! 
//! Functions for the netshopping category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// ## Return value



pub fn _net_gameserver_delete_character_slot_get_status_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A6D923DFFC9BD89u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A6D923DFFC9BD89u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _net_gameserver_delete_character_slot_get_status_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A6D923DFFC9BD89u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A6D923DFFC9BD89u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
bool is always true in game scripts  
```



pub fn _net_gameserver_get_price_safe(
        
        
            itemHash: 
        , 
        
        
            categoryHash: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC27009422FCCA88Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC27009422FCCA88Du64;
        
        let result = invoke_raw!(
            hash,
                itemHash, 
                categoryHash, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _net_gameserver_get_price_raw(
        itemHash: , 
        categoryHash: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC27009422FCCA88Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC27009422FCCA88Du64;

        invoke_raw_typed!(
            hash,
                itemHash, 
                categoryHash, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn _net_gameserver_checkout_start_safe(
        
        
            transactionId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x39BE7CEA8D9CC8E6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x39BE7CEA8D9CC8E6u64;
        
        let result = invoke_raw!(
            hash,
                transactionId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _net_gameserver_checkout_start_raw(
        transactionId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x39BE7CEA8D9CC8E6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x39BE7CEA8D9CC8E6u64;

        invoke_raw_typed!(
            hash,
                transactionId
        )
    }
}

/// ## Parameters
*



pub fn _net_gameserver_end_service_safe(
        
        
            transactionId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE2A99A9B524BEFFFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE2A99A9B524BEFFFu64;
        
        let result = invoke_raw!(
            hash,
                transactionId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _net_gameserver_end_service_raw(
        transactionId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE2A99A9B524BEFFFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE2A99A9B524BEFFFu64;

        invoke_raw_typed!(
            hash,
                transactionId
        )
    }
}

/// ## Parameters
*



pub fn _0xc13c38e47ea5df31_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC13C38E47EA5DF31u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC13C38E47EA5DF31u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xc13c38e47ea5df31_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC13C38E47EA5DF31u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC13C38E47EA5DF31u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn _net_gameserver_is_session_refresh_pending_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x810E8431C0614BF9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x810E8431C0614BF9u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _net_gameserver_is_session_refresh_pending_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x810E8431C0614BF9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x810E8431C0614BF9u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Note: only one of the arguments can be set to true at a time
```



pub fn _net_gameserver_get_balance_safe(
        
        
            inventory: 
        , 
        
        
            playerbalance: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x35A1B3E1D1315CFAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x35A1B3E1D1315CFAu64;
        
        let result = invoke_raw!(
            hash,
                inventory, 
                playerbalance
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _net_gameserver_get_balance_raw(
        inventory: , 
        playerbalance: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x35A1B3E1D1315CFAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x35A1B3E1D1315CFAu64;

        invoke_raw_typed!(
            hash,
                inventory, 
                playerbalance
        )
    }
}

/// ## Return value



pub fn _0x85f6c9aba1de2bcf_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x85F6C9ABA1DE2BCFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x85F6C9ABA1DE2BCFu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x85f6c9aba1de2bcf_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x85F6C9ABA1DE2BCFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x85F6C9ABA1DE2BCFu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0x0395cb47b022e62c_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0395CB47B022E62Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0395CB47B022E62Cu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x0395cb47b022e62c_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0395CB47B022E62Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0395CB47B022E62Cu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Same as 0x350AA5EBC03D3BD2
```



pub fn _net_gameserver_transfer_cash_get_status_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x23789E777D14CE44u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x23789E777D14CE44u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _net_gameserver_transfer_cash_get_status_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x23789E777D14CE44u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x23789E777D14CE44u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _net_gameserver_basket_start_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x279F08B1A4B29B7Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x279F08B1A4B29B7Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _net_gameserver_basket_start_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x279F08B1A4B29B7Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x279F08B1A4B29B7Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _net_gameserver_transfer_bank_to_wallet_safe(
        
        
            charSlot: 
        , 
        
        
            amount: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD47A2C1BA117471Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD47A2C1BA117471Du64;
        
        let result = invoke_raw!(
            hash,
                charSlot, 
                amount
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _net_gameserver_transfer_bank_to_wallet_raw(
        charSlot: , 
        amount: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD47A2C1BA117471Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD47A2C1BA117471Du64;

        invoke_raw_typed!(
            hash,
                charSlot, 
                amount
        )
    }
}

/// ## Parameters
*



pub fn _net_gameserver_basket_add_item_safe(
        
        
            itemData: 
        , 
        
        
            quantity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF30980718C8ED876u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF30980718C8ED876u64;
        
        let result = invoke_raw!(
            hash,
                itemData, 
                quantity
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _net_gameserver_basket_add_item_raw(
        itemData: , 
        quantity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF30980718C8ED876u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF30980718C8ED876u64;

        invoke_raw_typed!(
            hash,
                itemData, 
                quantity
        )
    }
}

/// ## Parameters
*



pub fn _net_gameserver_session_apply_received_data_safe(
        
        
            charSlot: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F41D51BA3BCD1F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F41D51BA3BCD1F1u64;
        
        let result = invoke_raw!(
            hash,
                charSlot
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _net_gameserver_session_apply_received_data_raw(
        charSlot: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F41D51BA3BCD1F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F41D51BA3BCD1F1u64;

        invoke_raw_typed!(
            hash,
                charSlot
        )
    }
}

/// ## Parameters
*



pub fn _net_gameserver_get_catalog_state_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCF38DAFBB49EDE5Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCF38DAFBB49EDE5Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _net_gameserver_get_catalog_state_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCF38DAFBB49EDE5Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCF38DAFBB49EDE5Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _net_gameserver_set_telemetry_nonce_seed_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9507D4271988E1AEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9507D4271988E1AEu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _net_gameserver_set_telemetry_nonce_seed_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9507D4271988E1AEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9507D4271988E1AEu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _net_gameserver_catalog_item_exists_hash_safe(
        
        
            hash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x247F0F73A182EA0Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x247F0F73A182EA0Bu64;
        
        let result = invoke_raw!(
            hash,
                hash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _net_gameserver_catalog_item_exists_hash_raw(
        hash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x247F0F73A182EA0Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x247F0F73A182EA0Bu64;

        invoke_raw_typed!(
            hash,
                hash
        )
    }
}

/// ## Return value



pub fn _net_gameserver_basket_is_full_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x27F76CC6C55AD30Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x27F76CC6C55AD30Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _net_gameserver_basket_is_full_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x27F76CC6C55AD30Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x27F76CC6C55AD30Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Same as 0x23789E777D14CE44
```



pub fn _net_gameserver_transfer_cash_get_status_2_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x350AA5EBC03D3BD2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x350AA5EBC03D3BD2u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _net_gameserver_transfer_cash_get_status_2_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x350AA5EBC03D3BD2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x350AA5EBC03D3BD2u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _net_gameserver_begin_service_safe(
        
        
            transactionId: 
        , 
        
        
            categoryHash: 
        , 
        
        
            itemHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3C5FD37B5499582Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3C5FD37B5499582Eu64;
        
        let result = invoke_raw!(
            hash,
                transactionId, 
                categoryHash, 
                itemHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _net_gameserver_begin_service_raw(
        transactionId: , 
        categoryHash: , 
        itemHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3C5FD37B5499582Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3C5FD37B5499582Eu64;

        invoke_raw_typed!(
            hash,
                transactionId, 
                categoryHash, 
                itemHash
        )
    }
}

/// ## Return value



pub fn _net_gameserver_catalog_is_ready_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3C4487461E9B0DCBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3C4487461E9B0DCBu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _net_gameserver_catalog_is_ready_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3C4487461E9B0DCBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3C4487461E9B0DCBu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _net_gameserver_transfer_wallet_to_bank_safe(
        
        
            charSlot: 
        , 
        
        
            amount: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC2F7FE5309181C7Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC2F7FE5309181C7Du64;
        
        let result = invoke_raw!(
            hash,
                charSlot, 
                amount
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _net_gameserver_transfer_wallet_to_bank_raw(
        charSlot: , 
        amount: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC2F7FE5309181C7Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC2F7FE5309181C7Du64;

        invoke_raw_typed!(
            hash,
                charSlot, 
                amount
        )
    }
}

/// ```
NET_GAMESERVER_*

Checks if the transaction status is equal to 3.

NativeDB Introduced: v1365
```



pub fn _0x79edac677ca62f81_safe(
        
        
            transactionId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x79EDAC677CA62F81u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x79EDAC677CA62F81u64;
        
        let result = invoke_raw!(
            hash,
                transactionId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x79edac677ca62f81_raw(
        transactionId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x79EDAC677CA62F81u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x79EDAC677CA62F81u64;

        invoke_raw_typed!(
            hash,
                transactionId
        )
    }
}

/// ## Parameters
*



pub fn _net_gameserver_get_transaction_manager_data_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x897433D292B44130u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x897433D292B44130u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _net_gameserver_get_transaction_manager_data_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x897433D292B44130u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x897433D292B44130u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _0x72eb7ba9b69bf6ab_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x72EB7BA9B69BF6ABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x72EB7BA9B69BF6ABu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x72eb7ba9b69bf6ab_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x72EB7BA9B69BF6ABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x72EB7BA9B69BF6ABu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _net_gameserver_start_session_safe(
        
        
            charSlot: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA135AC892A58FC07u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA135AC892A58FC07u64;
        
        let result = invoke_raw!(
            hash,
                charSlot
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _net_gameserver_start_session_raw(
        charSlot: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA135AC892A58FC07u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA135AC892A58FC07u64;

        invoke_raw_typed!(
            hash,
                charSlot
        )
    }
}

/// ## Parameters
*



pub fn _0x170910093218c8b9_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x170910093218C8B9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x170910093218C8B9u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x170910093218c8b9_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x170910093218C8B9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x170910093218C8B9u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _net_gameserver_delete_character_slot_safe(
        
        
            slot: 
        , 
        
        
            transfer: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x51F1A8E48C3D2F6Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x51F1A8E48C3D2F6Du64;
        
        let result = invoke_raw!(
            hash,
                slot, 
                transfer
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _net_gameserver_delete_character_slot_raw(
        slot: , 
        transfer: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x51F1A8E48C3D2F6Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x51F1A8E48C3D2F6Du64;

        invoke_raw_typed!(
            hash,
                slot, 
                transfer
        )
    }
}

/// ## Parameters
*



pub fn _net_gameserver_catalog_item_exists_safe(
        
        
            name: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBD4D7EAF8A30F637u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBD4D7EAF8A30F637u64;
        
        let result = invoke_raw!(
            hash,
                name
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _net_gameserver_catalog_item_exists_raw(
        name: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBD4D7EAF8A30F637u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBD4D7EAF8A30F637u64;

        invoke_raw_typed!(
            hash,
                name
        )
    }
}

/// ## Return value



pub fn _0x613f125ba3bd2eb9_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x613F125BA3BD2EB9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x613F125BA3BD2EB9u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x613f125ba3bd2eb9_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x613F125BA3BD2EB9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x613F125BA3BD2EB9u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _net_gameserver_is_catalog_valid_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B949A1E6AEC8F6Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B949A1E6AEC8F6Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _net_gameserver_is_catalog_valid_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B949A1E6AEC8F6Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B949A1E6AEC8F6Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Used to be NETWORK_SHOP_CASH_TRANSFER_SET_TELEMETRY_NONCE_SEED
```



pub fn _net_gameserver_transfer_cash_set_telemetry_nonce_seed_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x498C1E05CE5F7877u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x498C1E05CE5F7877u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _net_gameserver_transfer_cash_set_telemetry_nonce_seed_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x498C1E05CE5F7877u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x498C1E05CE5F7877u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _net_gameserver_use_server_transactions_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7D2708796355B20Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7D2708796355B20Bu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _net_gameserver_use_server_transactions_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7D2708796355B20Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7D2708796355B20Bu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _0x357b152ef96c30b6_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x357B152EF96C30B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x357B152EF96C30B6u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x357b152ef96c30b6_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x357B152EF96C30B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x357B152EF96C30B6u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NET_GAMESERVER_*
```



pub fn _0x74a0fd0688f1ee45_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x74A0FD0688F1EE45u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x74A0FD0688F1EE45u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x74a0fd0688f1ee45_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x74A0FD0688F1EE45u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x74A0FD0688F1EE45u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Introduced: v372
```



pub fn _net_gameserver_basket_delete_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFA336E7F40C0A0D0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFA336E7F40C0A0D0u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _net_gameserver_basket_delete_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFA336E7F40C0A0D0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFA336E7F40C0A0D0u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _net_gameserver_basket_end_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA65568121DF2EA26u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA65568121DF2EA26u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _net_gameserver_basket_end_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA65568121DF2EA26u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA65568121DF2EA26u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _net_gameserver_delete_set_telemetry_nonce_seed_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x112CEF1615A1139Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x112CEF1615A1139Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _net_gameserver_delete_set_telemetry_nonce_seed_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x112CEF1615A1139Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x112CEF1615A1139Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NET_GAMESERVER_*

Checks if the transaction status is equal to 1.

NativeDB Introduced: v1365
```



pub fn _0xc830417d630a50f9_safe(
        
        
            transactionId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC830417D630A50F9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC830417D630A50F9u64;
        
        let result = invoke_raw!(
            hash,
                transactionId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xc830417d630a50f9_raw(
        transactionId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC830417D630A50F9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC830417D630A50F9u64;

        invoke_raw_typed!(
            hash,
                transactionId
        )
    }
}

/// ## Return value



pub fn _0xe3e5a7c64ca2c6ed_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE3E5A7C64CA2C6EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE3E5A7C64CA2C6EDu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xe3e5a7c64ca2c6ed_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE3E5A7C64CA2C6EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE3E5A7C64CA2C6EDu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _net_gameserver_is_session_valid_safe(
        
        
            charSlot: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB24F0944DA203D9Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB24F0944DA203D9Eu64;
        
        let result = invoke_raw!(
            hash,
                charSlot
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _net_gameserver_is_session_valid_raw(
        charSlot: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB24F0944DA203D9Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB24F0944DA203D9Eu64;

        invoke_raw_typed!(
            hash,
                charSlot
        )
    }
}

/// ## Parameters
*



pub fn _net_gameserver_basket_apply_server_data_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE1A0450ED46A7812u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE1A0450ED46A7812u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _net_gameserver_basket_apply_server_data_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE1A0450ED46A7812u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE1A0450ED46A7812u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

