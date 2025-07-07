#[macro_export]
macro_rules! define_api {
    (
        $prefix:ident {
            host: $host:expr,
            endpoint: $endpoint:expr,
            method: $method:expr,

            ReqHeader { $($rhk:tt)* },
            Req       { $($rk:tt)* },
            ResHeader { $($rshk:tt)* },
            Res       { $($rs:tt)* }
        }
    ) => {
        paste::paste! {
            #[derive(Debug, serde::Serialize)]
            pub struct [<$prefix ReqHeader>] {
                $($rhk)*
            }

            #[derive(Debug, serde::Serialize)]
            pub struct [<$prefix Req>] {
                $($rk)*
            }

            #[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
            pub struct [<$prefix ResHeader>] {
                $($rshk)*
            }

            #[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
            pub struct [<$prefix Res>] {
                $($rs)*
            }

            pub struct $prefix;

            impl $crate::api::ApiSpec for $prefix {
                type ReqHeader = [<$prefix ReqHeader>];
                type Req = [<$prefix Req>];
                type ResHeader = [<$prefix ResHeader>];
                type Res = [<$prefix Res>];

                const HOST: &'static str = $host;
                const ENDPOINT: &'static str = $endpoint;
                const METHOD: reqwest::Method = $method;
            }
        }
    };
}
