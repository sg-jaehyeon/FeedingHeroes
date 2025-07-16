use crate::define_api;

use std::fmt::Debug;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Ka40001ResItem {
    pub etfprft_rt: Option<String>,
    pub cntr_prft_rt: Option<String>,
    pub for_netprps_qty: Option<String>,
    pub orgn_netprps_qty: Option<String>,
}

define_api!(
    Ka40001 {
        host: "https://mockapi.kiwoom.com",
        endpoint: "/api/dostk/etf",
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
            pub stk_cd: String,
            pub etfobjt_idex_cd: String,
            pub dt: String,
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
            pub etfprft_rt_lst: Option<Vec<Ka40001ResItem>>
        }
    }
);

define_api!(
    Ka40002 {
        host: "https://mockapi.kiwoom.com",
        endpoint: "/api/dostk/etf",
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
            pub stk_cd: String,
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
            pub stk_nm: Option<String>,
            pub etfobjt_idex_nm: Option<String>,
            pub wonju_pric: Option<String>,
            pub etftxon_type: Option<String>,
            pub etntxon_type: Option<String>,
        }
    }
);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Ka40003ResItem {
    pub cntr_dt: Option<String>,
    pub cur_prc: Option<String>,
    pub pre_sig: Option<String>,
    pub pred_pre: Option<String>,
    pub pre_rt: Option<String>,
    pub trde_qty: Option<String>,
    pub nav: Option<String>,
    pub acc_trde_prica: Option<String>,
    pub navidex_dispty_rt: Option<String>,
    pub navetfdispty_rt: Option<String>,
    pub trace_eor_rt: Option<String>,
    pub trace_cur_prc: Option<String>,
    pub trace_pred_pre: Option<String>,
    pub trace_pre_sig: Option<String>,
}

define_api!(
    Ka40003 {
        host: "https://mockapi.kiwoom.com",
        endpoint: "/api/dostk/etf",
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
            pub stk_cd: String,
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
            pub etfdaly_tmsn: Option<Vec<Ka40003ResItem>>
        }
    }
);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Ka40004ResItem {
    pub stk_cd: Option<String>,
    pub stk_cls: Option<String>,
    pub stk_nm: Option<String>,
    pub close_pric: Option<String>,
    pub pre_sig: Option<String>,
    pub pred_pre: Option<String>,
    pub pre_rt: Option<String>,
    pub trde_qty: Option<String>,
    pub nav: Option<String>,
    pub trace_eor_rt: Option<String>,
    pub txbs: Option<String>,
    pub dvid_bf_base: Option<String>,
    pub pred_dvida: Option<String>,
    pub trace_idex_nm: Option<String>,
    pub drng: Option<String>,
    pub trace_idex_cd: Option<String>,
    pub trace_idex: Option<String>,
    pub trace_flu_rt: Option<String>,
}

define_api!(
    Ka40004 {
        host: "https://mockapi.kiwoom.com",
        endpoint: "/api/dostk/etf",
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
            /**
             * 0: 전체, 1: 비과세, 2: 보유기간과세, 3: 회사형,
             * 4: 외국, 5: 비과세해외(보유기간관세)
             */
            pub txon_type: String,
            pub navpre: String,
            /**
             * 0000: 전체,
             * 3020: KODEX (삼성),
             * 3027: KOSEF (키움),
             * 3191: TIGER (미래에셋),
             * 3228: KINDEX (한국투자),
             * 3023: KStar (KB),
             * 3022: 아리랑 (한화),
             * 9999: 기타운용사
             */
            pub mngmcomp: String,
            /**
             * 0: 전체, 1: 과세, 2: 비과세
             */
            pub txon_yn: String,
            pub trace_idex: String,
            pub stex_tp: String
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
            pub etfall_mrpr: Option<Vec<Ka40004ResItem>>
        }
    }
);
