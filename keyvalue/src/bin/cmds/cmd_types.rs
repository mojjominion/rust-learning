use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum CMD {
    /*
     * Starts a transaction.
     * These transactions allow you to modify the state of the system and commit or rollback your changes.
     */
    BEGIN,

    /*
     * Sets the given key to the specified value. A key can also be updated.
     */
    SET,

    /*
     * Prints out the current value of the specified key.
     */
    GET,

    /*
     * Returns the number of keys that have been set to the specified value.
     * If no keys have been set to that value, prints 0.
     */
    COUNT,

    /*
     * Deletes the given key. If the key has not been set, ignore
     */
    DELETE,

    /*
     * Commits the changes made within the context of the active transaction and ends the active transaction.
     */
    COMMIT,

    /*
     * Throws away changes made within the context of the active transaction.
     * If no transaction is active, prints "No Active Transaction".
     */
    ROLLBACK,

    /*
     * Ends a transaction. Everything done within the "active" transaction is lost.
     */
    END,
}

impl FromStr for CMD {
    type Err = String;

    fn from_str(input: &str) -> Result<CMD, Self::Err> {
        let command = input.to_uppercase();

        match command.as_str() {
            "BEGIN" => Ok(CMD::BEGIN),
            "SET" => Ok(CMD::SET),
            "GET" => Ok(CMD::GET),
            "DELETE" => Ok(CMD::DELETE),
            "COUNT" => Ok(CMD::COUNT),
            "COMMIT" => Ok(CMD::COMMIT),
            "ROLLBACK" => Ok(CMD::ROLLBACK),
            "END" => Ok(CMD::END),
            _ => Err(format!("Command {:?} not found", command)),
        }
    }
}
