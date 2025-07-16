use crate::*;

use std::fmt::Debug;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Ka10072ResItem {
    pub stk_nm: String,
    pub cntr_qty: String,
    pub buy_uv: String,
    pub cntr_pric: String,
    pub tdy_sel_pl: String,
    pub pl_rt: String,
    pub stk_cd: String,
    pub tdy_trde_cmsn: String,
    pub tdy_trde_tax: String,
    pub wthd_alowa: String,
    pub loan_dt: String,
    pub crd_tp: String,
    pub stk_cd_1: String,
    pub tdy_sel_pl_1: String,
}

define_api!(
    Ka10072 {
        host: "https://mockapi.kiwoom.com",
        endpoint: "/api/dostk/acnt",
        method: reqwest::Method::POST,
        ReqHeader {
            #[serde(rename = "Content_Type")]
            pub content_type: String,
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
            pub strt_dt: String, // YYYYMMDD
        },
        ResHeader {
            #[serde(rename = "Content-Type")]
            pub content_type: String,
            #[serde(rename = "cont-yn")]
            pub cont_yn: Option<String>,
            #[serde(rename = "next-key")]
            pub next_key: Option<String>,
            #[serde(rename = "api-id")]
            pub api_id: String,
        },
        Res {
            pub dt_stk_div_rlzt_pl: Vec<Ka10072ResItem>,
        }
    }
);

//alias_field!(Ka10072Res, dt_stk_div_rlzt_pl, 일자별종목별실현손익);

//alias_field!(Ka10072ResItem, stk_nm, 종목명);
//alias_field!(Ka10072ResItem, cntr_qty, 체결량);
//alias_field!(Ka10072ResItem, buy_uv, 매입단가);
//alias_field!(Ka10072ResItem, cntr_pric, 체결가);
//alias_field!(Ka10072ResItem, tdy_sel_pl, 당일매도손익);
//alias_field!(Ka10072ResItem, pl_rt, 손익률);
//alias_field!(Ka10072ResItem, stk_cd, 종목코드);
//alias_field!(Ka10072ResItem, tdy_trde_cmsn, 당일매매수수료);
//alias_field!(Ka10072ResItem, tdy_trde_tax, 당일매매세금);
//alias_field!(Ka10072ResItem, wthd_alowa, 인출가능금액);
//alias_field!(Ka10072ResItem, loan_dt, 대출일);
//alias_field!(Ka10072ResItem, crd_tp, 신용구분);
//alias_field!(Ka10072ResItem, stk_cd_1, 종목코드_1);
//alias_field!(Ka10072ResItem, tdy_sel_pl_1, 당일매도손익_1);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Ka10073ResItem {
    pub dt: String,
    pub tdy_htssel_cmsn: String,
    pub stk_nm: String,
    pub cntr_qty: String,
    pub buy_uv: String,
    pub cntr_pric: String,
    pub tdy_sel_pl: String,
    pub pl_rt: String,
    pub stk_cd: String,
    pub tdy_trde_cmsn: String,
    pub tdy_trde_tax: String,
    pub wthd_alowa: String,
    pub loan_dt: String,
    pub crd_tp: String,
}

define_api!(
    Ka10073 {
        host: "https://mockapi.kiwoom.com",
        endpoint: "/api/dostk/acnt",
        method: reqwest::Method::POST,
        ReqHeader {
            #[serde(rename = "Content_Type")]
            pub content_type: String,
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
            pub strt_dt: String, // YYYYMMDD
            pub end_dt: String, // YYYYMMDD
        },
        ResHeader {
            #[serde(rename = "Content-Type")]
            pub content_type: String,
            #[serde(rename = "cont-yn")]
            pub cont_yn: Option<String>,
            #[serde(rename = "next-key")]
            pub next_key: Option<String>,
            #[serde(rename = "api-id")]
            pub api_id: String,
        },
        Res {
            pub dt_stk_rlzt_pl: Vec<Ka10073ResItem>,
        }
    }
);

//alias_field!(Ka10073Res, dt_stk_rlzt_pl, 일자별종목별실현손익);

//alias_field!(Ka10073ResItem, dt, 일자);
//alias_field!(Ka10073ResItem, tdy_htssel_cmsn, 당일hts매도수수료);
//alias_field!(Ka10073ResItem, stk_nm, 종목명);
//alias_field!(Ka10073ResItem, cntr_qty, 체결량);
//alias_field!(Ka10073ResItem, buy_uv, 매입단가);
//alias_field!(Ka10073ResItem, cntr_pric, 체결가);
//alias_field!(Ka10073ResItem, tdy_sel_pl, 당일매도손익);
//alias_field!(Ka10073ResItem, pl_rt, 손익률);
//alias_field!(Ka10073ResItem, stk_cd, 종목코드);
//alias_field!(Ka10073ResItem, tdy_trde_cmsn, 당일매매수수료);
//alias_field!(Ka10073ResItem, tdy_trde_tax, 당일매매세금);
//alias_field!(Ka10073ResItem, wthd_alowa, 인출가능금액);
//alias_field!(Ka10073ResItem, loan_dt, 대출일);
//alias_field!(Ka10073ResItem, crd_tp, 신용구분);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Ka10074ResItem {
    pub dt: String,
    pub buy_amt: String,
    pub sell_amt: String,
    pub tdy_sel_pl: String,
    pub tdy_trde_cmsn: String,
    pub tdy_trde_tax: String,
}

define_api!(
    Ka10074 {
        host: "https://mockapi.kiwoom.com",
        endpoint: "/api/dostk/acnt",
        method: reqwest::Method::POST,
        ReqHeader {
            #[serde(rename = "Content_Type")]
            pub content_type: String,
            pub authorization: String,
            #[serde(rename = "cont-yn")]
            pub cont_yn: Option<String>,
            #[serde(rename = "next-key")]
            pub next_key: Option<String>,
            #[serde(rename = "api-id")]
            pub api_id: String,
        },
        Req {
            pub strt_dt: String, // YYYYMMDD
            pub end_dt: String, // YYYYMMDD
        },
        ResHeader {
            #[serde(rename = "Content-Type")]
            pub content_type: String,
            #[serde(rename = "cont-yn")]
            pub cont_yn: Option<String>,
            #[serde(rename = "next-key")]
            pub next_key: Option<String>,
            #[serde(rename = "api-id")]
            pub api_id: String,
        },
        Res {
            pub tot_buy_amt: Option<String>,
            pub tot_sell_amt: Option<String>,
            pub rlzt_pl: Option<String>,
            pub trde_cmsn: Option<String>,
            pub trde_tax: Option<String>,
            pub dt_rlzt_pl: Option<Vec<Ka10074ResItem>>,
        }
    }
);

//alias_field!(Ka10074Res, tot_buy_amt, 총매수금액);
//alias_field!(Ka10074Res, tot_sell_amt, 총매도금액);
//alias_field!(Ka10074Res, rlzt_pl, 실현손익);
//alias_field!(Ka10074Res, trde_cmsn, 매매수수료);
//alias_field!(Ka10074Res, trde_tax, 매매세금);
//alias_field!(Ka10074Res, dt_rlzt_pl, 일자별실현손익);

//alias_field!(Ka10074ResItem, dt, 일자);
//alias_field!(Ka10074ResItem, buy_amt, 매수금액);
//alias_field!(Ka10074ResItem, sell_amt, 매도금액);
//alias_field!(Ka10074ResItem, tdy_sel_pl, 당일매도손익);
//alias_field!(Ka10074ResItem, tdy_trde_cmsn, 당일매매수수료);
//alias_field!(Ka10074ResItem, tdy_trde_tax, 당일매매세금);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Ka10075ResItem {
    pub acnt_no: Option<String>,
    pub ord_no: Option<String>,
    pub mang_empno: Option<String>,
    pub stk_cd: Option<String>,
    pub tsk_tp: Option<String>,
    pub ord_stt: Option<String>,
    pub stk_nm: Option<String>,
    pub ord_qty: Option<String>,
    pub ord_pric: Option<String>,
    pub oso_qty: Option<String>,
    pub cntr_tot_amt: Option<String>,
    pub orig_ord_no: Option<String>,
    pub io_tp_nm: Option<String>,
    pub trde_tp: Option<String>,
    pub tm: Option<String>,
    pub cntr_no: Option<String>,
    pub cntr_pric: Option<String>,
    pub cntr_qty: Option<String>,
    pub cur_prc: Option<String>,
    pub sel_bid: Option<String>,
    pub buy_bid: Option<String>,
    pub unit_cntr_pric: Option<String>,
    pub unit_cntr_qty: Option<String>,
    pub tdy_trde_cmsn: Option<String>,
    pub tdy_trde_tax: Option<String>,
    pub ind_invsr: Option<String>,
    pub stex_tp: Option<String>, // 0 : 통합, 1 : KRX, 2 : NXT
    pub stex_tp_txt: Option<String>, // 통합,KRX,NXT
    pub sor_yn: Option<String>, // Y,N
    pub stop_pric: Option<String>, // 스톱지정가주문 스톱가
}

define_api!(
    Ka10075 {
        host: "https://mockapi.kiwoom.com",
        endpoint: "/api/dostk/acnt",
        method: reqwest::Method::POST,
        ReqHeader {
            #[serde(rename = "Content_Type")]
            pub content_type: String,
            pub authorization: String,
            #[serde(rename = "cont-yn")]
            pub cont_yn: Option<String>,
            #[serde(rename = "next-key")]
            pub next_key: Option<String>,
            #[serde(rename = "api-id")]
            pub api_id: String,
        },
        Req {
            pub all_stk_tp: String, // 0:전체, 1:종목
            pub trde_tp: String, // 0:전체,1:매도,2:매수
            pub stk_cd: Option<String>,
            pub stex_tp: String, // 0:통합,1:KRX,2:NXT
        },
        ResHeader {
            #[serde(rename = "Content-Type")]
            pub content_type: String,
            #[serde(rename = "cont-yn")]
            pub cont_yn: Option<String>,
            #[serde(rename = "next-key")]
            pub next_key: Option<String>,
            #[serde(rename = "api-id")]
            pub api_id: String,
        },
        Res {
            pub oso: Option<Vec<Ka10075ResItem>>,
        }
    }
);

//alias_field!(Ka10075Res, oso, 미체결);

//alias_field!(Ka10075ResItem, acnt_no, 계좌번호);
//alias_field!(Ka10075ResItem, ord_no, 주문번호);
//alias_field!(Ka10075ResItem, mang_empno, 관리사번);
//alias_field!(Ka10075ResItem, stk_cd, 종목코드);
//alias_field!(Ka10075ResItem, tsk_tp, 업무구분);
//alias_field!(Ka10075ResItem, ord_stt, 주문상태);
//alias_field!(Ka10075ResItem, stk_nm, 종목명);
//alias_field!(Ka10075ResItem, ord_qty, 주문수량);
//alias_field!(Ka10075ResItem, ord_pric, 주문가격);
//alias_field!(Ka10075ResItem, oso_qty, 미체결수량);
//alias_field!(Ka10075ResItem, cntr_tot_amt, 체결누계금액);
//alias_field!(Ka10075ResItem, orig_ord_no, 원주문번호);
//alias_field!(Ka10075ResItem, io_tp_nm, 주문구분);
//alias_field!(Ka10075ResItem, trde_tp, 매매구분);
//alias_field!(Ka10075ResItem, tm, 시간);
//alias_field!(Ka10075ResItem, cntr_no, 체결번호);
//alias_field!(Ka10075ResItem, cntr_pric, 체결가);
//alias_field!(Ka10075ResItem, cntr_qty, 체결량);
//alias_field!(Ka10075ResItem, cur_prc, 현재가);
//alias_field!(Ka10075ResItem, sel_bid, 매도호가);
//alias_field!(Ka10075ResItem, buy_bid, 매수호가);
//alias_field!(Ka10075ResItem, unit_cntr_pric, 단위체결가);
//alias_field!(Ka10075ResItem, unit_cntr_qty, 단위체결량);
//alias_field!(Ka10075ResItem, tdy_trde_cmsn, 당일매매수수료);
//alias_field!(Ka10075ResItem, tdy_trde_tax, 당일매매세금);
//alias_field!(Ka10075ResItem, ind_invsr, 개인투자자);
//alias_field!(Ka10075ResItem, stex_tp, 거래소구분);
//alias_field!(Ka10075ResItem, stex_tp_txt, 거래소구분텍스트);
//alias_field!(Ka10075ResItem, sor_yn, SOR여부값);
//alias_field!(Ka10075ResItem, stop_pric, 스톱가);

