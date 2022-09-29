/**
 * [Retrieving Query Results Row-by-Row](https://www.postgresql.org/docs/current/libpq-single-row-mode.html)
 */
impl Connection {
    /**
     * Select single-row mode for the currently-executing query.
     *
     * See
     * [PQsetSingleRowMode](https://www.postgresql.org/docs/current/libpq-single-row-mode.html#LIBPQ-PQSETSINGLEROWMODE).
     */
    pub fn set_single_row_mode(&self) -> crate::errors::Result {
        let success = unsafe { pq_sys::PQsetSingleRowMode(self.into()) };

        if success == 1 {
            Ok(())
        } else {
            Err(crate::errors::Error::Unknow)
        }
    }
}
