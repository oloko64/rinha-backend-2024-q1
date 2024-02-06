use chrono::{DateTime, SecondsFormat, Utc};

use crate::models::ExtratoModel;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ExtratoResponse {
    saldo: SaldoResponse,
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
                .to_rfc3339_opts(SecondsFormat::Millis, true),
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
                    .to_rfc3339_opts(SecondsFormat::Millis, true),
                })
                .collect(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct SaldoResponse {
    total: i64,
    limite: i64,
    data_extrato: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct TransactionResponse {
    valor: i64,
    tipo: String,
    descricao: String,
    realizada_em: String,
}
