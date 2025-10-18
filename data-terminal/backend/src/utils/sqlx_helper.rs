// Macro to implement SQLx traits for string-based enums
#[macro_export]
macro_rules! impl_sqlx_for_string_enum {
    ($enum_type:ty) => {
        impl<'a> sqlx::Type<sqlx::MySql> for $enum_type {
            fn type_info() -> sqlx::mysql::MySqlTypeInfo {
                // MySQL ENUM is treated as a string type
                <str as sqlx::Type<sqlx::MySql>>::type_info()
            }

            fn compatible(ty: &sqlx::mysql::MySqlTypeInfo) -> bool {
                // Check if the database type is compatible with the string type
                <str as sqlx::Type<sqlx::MySql>>::compatible(ty)
            }
        }

        impl<'r> sqlx::Decode<'r, sqlx::MySql> for $enum_type {
            fn decode(
                value: sqlx::mysql::MySqlValueRef<'r>,
            ) -> Result<Self, sqlx::error::BoxDynError> {
                let text = <&str as sqlx::Decode<sqlx::MySql>>::decode(value)?;
                use std::str::FromStr;
                Ok(<$enum_type>::from_str(text)?)
            }
        }

        impl<'q> sqlx::Encode<'q, sqlx::MySql> for $enum_type {
            fn encode_by_ref(
                &self,
                buf: &mut Vec<u8>,
            ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
                // Delegate to String encoding to ensure proper MySQL protocol formatting
                let s = self.to_string();
                <String as sqlx::Encode<sqlx::MySql>>::encode_by_ref(&s, buf)
            }
        }
    };
}
