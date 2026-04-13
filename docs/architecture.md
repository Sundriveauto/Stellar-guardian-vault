# Guardian Vault Architecture

## Recovery Flow
1. **Trigger:** A Guardian calls `start_recovery` with a proposed `new_owner`.
2. **Voting:** Other Guardians must call `vote_recovery` within 48 hours.
3. **Execution:** Once a 2/3 majority is reached, the `owner` key in storage is swapped.

## Security Features
- **Time-Lock:** All recovery attempts have a 48-hour delay to allow the original owner to cancel if they still have their keys.
- **Panic Mode:** The owner can temporarily freeze the vault if suspicious activity is detected.
