// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

pub mod authenticated {
    use parsec_schema::parsec_schema;

    /*
     * PingReq
     */

    #[parsec_schema]
    pub struct PingReq {
        pub ping: String,
    }

    /*
     * PingRep
     */

    #[parsec_schema]
    #[serde(tag = "status", rename_all = "snake_case")]
    pub enum PingRep {
        Ok { pong: String },
        UnknownError { error: String },
    }
}

pub mod invited {
    use parsec_schema::parsec_schema;

    /*
     * PingReq
     */

    #[parsec_schema]
    pub struct PingReq {
        pub ping: String,
    }

    /*
     * PingRep
     */

    #[parsec_schema]
    #[serde(tag = "status", rename_all = "snake_case")]
    pub enum PingRep {
        Ok { pong: String },
        UnknownError { error: String },
    }
}
