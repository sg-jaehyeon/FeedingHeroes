use crate::define_api;

use std::fmt::Debug;

define_api!(
    Kt10000 {
        host: "https://mockapi.kiwoom.com",
        endpoint: "/api/dostk/ordr",
        method: reqwest::Method::POST,

        ReqHeader {
            pub authorization: String,
            #[serde(rename = "cont-yn")]
            pub cont_yn: Option<String>,
            #[serde(rename = "next-key")]
            pub next_key: Option<String>,
            #[serde(rename = "api-id")]
            pub api_id: String,
        },
        Req {
            pub dmst_stex_tp: String, // "KRX", "NXT", "SOR"
            pub stk_cd: String,
            pub ord_qty: String,
            pub ord_uv: Option<String>,
            /**
             * 0: 보통, 3: 시장가, 5: 조건부지정가, 81: 장마감후시간외, 61: 장시작전시간외,
             * 62: 시간외단일가, 6: 최유리지정가, 7: 최우선지정가, 10: 보통(IOC),
             * 13: 시장가(IOC), 16: 최유리(IOC), 20: 보통(FOK), 23: 시장가(FOK),
             * 26: 최유리(FOK), 28: 스톱지정가, 29: 중간가, 30: 중간가(IOC), 31: 중간가(FOK)
             */
            pub trde_tp: String,
            pub cond_uv: Option<String>
        },
        ResHeader {
            #[serde(rename = "cont-yn")]
            pub cont_yn: Option<String>,
            #[serde(rename = "next-key")]
            pub next_key: Option<String>,
            #[serde(rename = "api-id")]
            pub api_id: String,
        },
        Res {
            pub ord_no: Option<String>,
            pub dmst_stex_tp: Option<String>
        }
    }
);

define_api!(
    Kt10001 {
        host: "https://mockapi.kiwoom.com",
        endpoint: "/api/dostk/ordr",
        method: reqwest::Method::POST,

        ReqHeader {
            pub authorization: String,
            #[serde(rename = "cont-yn")]
            pub cont_yn: Option<String>,
            #[serde(rename = "next-key")]
            pub next_key: Option<String>,
            #[serde(rename = "api-id")]
            pub api_id: String,
        },
        Req {
            pub dmst_stex_tp: String, // "KRX", "NXT", "SOR"
            pub stk_cd: String,
            pub ord_qty: String,
            pub ord_uv: Option<String>,
            /**
             * 0: 보통, 3: 시장가, 5: 조건부지정가, 81: 장마감후시간외, 61: 장시작전시간외,
             * 62: 시간외단일가, 6: 최유리지정가, 7: 최우선지정가, 10: 보통(IOC),
             * 13: 시장가(IOC), 16: 최유리(IOC), 20: 보통(FOK), 23: 시장가(FOK),
             * 26: 최유리(FOK), 28: 스톱지정가, 29: 중간가, 30: 중간가(IOC), 31: 중간가(FOK)
             */
            pub trde_tp: String,
            pub cond_uv: Option<String>
        },
        ResHeader {
            #[serde(rename = "cont-yn")]
            pub cont_yn: Option<String>,
            #[serde(rename = "next-key")]
            pub next_key: Option<String>,
            #[serde(rename = "api-id")]
            pub api_id: String,
        },
        Res {
            pub ord_no: Option<String>,
            pub dmst_stex_tp: Option<String>
        }
    }
);

define_api!(
    Kt10002 {
        host: "https://mockapi.kiwoom.com",
        endpoint: "/api/dostk/ordr",
        method: reqwest::Method::POST,

        ReqHeader {
            pub authorization: String,
            #[serde(rename = "cont-yn")]
            pub cont_yn: Option<String>,
            #[serde(rename = "next-key")]
            pub next_key: Option<String>,
            #[serde(rename = "api-id")]
            pub api_id: String,
        },
        Req {
            pub dmst_stex_tp: String, // "KRX", "NXT", "SOR"
            pub orig_ord_no: String,
            pub stk_cd: String,
            pub mdfy_qty: String,
            pub mdfy_uv: String,
            pub mdfy_cond_uv: Option<String>
        },
        ResHeader {
            #[serde(rename = "cont-yn")]
            pub cont_yn: Option<String>,
            #[serde(rename = "next-key")]
            pub next_key: Option<String>,
            #[serde(rename = "api-id")]
            pub api_id: String,
        },
        Res {
            pub ord_no: Option<String>,
            pub base_orig_ord_no: Option<String>,
            pub mdfy_qty: Option<String>,
            pub dmst_stex_tp: Option<String>
        }
    }
);

define_api!(
    Kt10003 {
        host: "https://mockapi.kiwoom.com",
        endpoint: "/api/dostk/ordr",
        method: reqwest::Method::POST,

        ReqHeader {
            pub authorization: String,
            #[serde(rename = "cont-yn")]
            pub cont_yn: Option<String>,
            #[serde(rename = "next-key")]
            pub next_key: Option<String>,
            #[serde(rename = "api-id")]
            pub api_id: String,
        },
        Req {
            pub dmst_stex_tp: String, // "KRX", "NXT", "SOR"
            pub orig_ord_no: String,
            pub stk_cd: String,
            pub cncl_qty: String, // 0 입력시 잔량 전부 취소
        },
        ResHeader {
            #[serde(rename = "cont-yn")]
            pub cont_yn: Option<String>,
            #[serde(rename = "next-key")]
            pub next_key: Option<String>,
            #[serde(rename = "api-id")]
            pub api_id: String,
        },
        Res {
            pub ord_no: Option<String>,
            pub base_orig_ord_no: Option<String>,
            pub cncl_qty: Option<String>
        }
    }
);
