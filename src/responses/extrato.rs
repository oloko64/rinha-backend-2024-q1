use chrono::{DateTime, SecondsFormat, Utc};

use crate::models::ExtratoModel;

#[derive(serde::Serialize, Debug)]
pub struct ExtratoResponse {
    saldo: SaldoResponse,

    #[serde(rename = "ultimas_transacoes")]
    transacoes: Vec<TransactionResponse>,
}

impl From<ExtratoModel> for ExtratoResponse {
    fn from(extrato: ExtratoModel) -> Self {
        ExtratoResponse {
            saldo: SaldoResponse {
                total: extrato.saldo.balance,
                limite: extrato.saldo.balance_limit,
                data_extrato: DateTime::<Utc>::from_naive_utc_and_offset(
                    extrato.saldo.date,
                    chrono::Utc,
                )
                .to_rfc3339_opts(SecondsFormat::Micros, true),
            },
            transacoes: extrato
                .transactions
                .into_iter()
                .map(|t| TransactionResponse {
                    valor: t.amount,
                    tipo: t.r#type,
                    descricao: t.description,
                    realizada_em: DateTime::<Utc>::from_naive_utc_and_offset(
                        t.created_at,
                        chrono::Utc,
                    )
                    .to_rfc3339_opts(SecondsFormat::Micros, true),
                })
                .collect(),
        }
    }
}

#[derive(serde::Serialize, Debug)]
pub struct SaldoResponse {
    total: i32,
    limite: i32,
    data_extrato: String,
}

#[derive(serde::Serialize, Debug)]
pub struct TransactionResponse {
    valor: i32,
    tipo: String,
    descricao: String,
    realizada_em: String,
}
