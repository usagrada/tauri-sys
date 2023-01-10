use serde::de::DeserializeOwned;
use wasm_bindgen::JsValue;


#[derive(Clone, Eq, PartialEq, Debug, thiserror::Error)]
pub enum Error<T: DeserializeOwned = String> {
    #[error("TODO.")]
    Binding(String),
    #[error("TODO.")]
    Serde(String),
    #[cfg(any(feature = "event", feature = "window"))]
    #[error("TODO.")]
    OneshotCanceled(#[from] futures::channel::oneshot::Canceled),
    #[error("custom error TODO.")]
    CustomError(T),
}

impl<T: DeserializeOwned> From<serde_wasm_bindgen::Error> for Error<T> {
    fn from(e: serde_wasm_bindgen::Error) -> Self {
        Self::Serde(e.to_string())
    }
}

impl<T: DeserializeOwned> From<JsValue> for Error<T> {
    fn from(e: JsValue) -> Self {
        Self::Binding(format!("{:?}", e))
    }
}
