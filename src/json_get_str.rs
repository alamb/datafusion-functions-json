use std::any::Any;
use std::sync::Arc;

use datafusion::arrow::array::{ArrayRef, StringArray};
use datafusion::arrow::datatypes::DataType;
use datafusion::common::{Result as DataFusionResult, ScalarValue};
use datafusion::logical_expr::{ColumnarValue, ScalarUDFImpl, Signature, Volatility};
use jiter::Peek;

use crate::common::{get_err, invoke, jiter_json_find, scalar_udf_return_type, GetError, JsonPath};
use crate::common_macros::make_udf_function;

make_udf_function!(
    JsonGetStr,
    json_get_str,
    json_data path,
    r#"Get a string value from a JSON string by its "path""#
);

#[derive(Debug)]
pub(super) struct JsonGetStr {
    signature: Signature,
    aliases: [String; 1],
}

impl Default for JsonGetStr {
    fn default() -> Self {
        Self {
            signature: Signature::variadic_any(Volatility::Immutable),
            aliases: ["json_get_str".to_string()],
        }
    }
}

impl ScalarUDFImpl for JsonGetStr {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn name(&self) -> &str {
        self.aliases[0].as_str()
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn return_type(&self, arg_types: &[DataType]) -> DataFusionResult<DataType> {
        scalar_udf_return_type(arg_types, self.name(), DataType::Utf8)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> DataFusionResult<ColumnarValue> {
        invoke::<StringArray, String>(
            args,
            jiter_json_get_str,
            |c| Ok(Arc::new(c) as ArrayRef),
            ScalarValue::Utf8,
        )
    }

    fn aliases(&self) -> &[String] {
        &self.aliases
    }
}

fn jiter_json_get_str(json_data: Option<&str>, path: &[JsonPath]) -> Result<String, GetError> {
    if let Some((mut jiter, peek)) = jiter_json_find(json_data, path) {
        match peek {
            Peek::String => Ok(jiter.known_str()?.to_owned()),
            _ => get_err!(),
        }
    } else {
        get_err!()
    }
}
