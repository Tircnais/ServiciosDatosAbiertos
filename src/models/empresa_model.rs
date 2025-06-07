use mongodb::bson::oid::ObjectId;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
// use serde::{self, Deserialize, Deserializer, Serialize, Serializer};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub fn serialize_datetime<S>(
    date: &Option<DateTime<Utc>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match date {
        Some(dt) => serializer.serialize_str(&dt.to_rfc3339()),
        None => serializer.serialize_none(),
    }
}

pub fn deserialize_datetime<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    if let Some(s) = opt {
        if s.trim().is_empty() {
            return Ok(None);
        }
        match NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S") {
            Ok(naive) => {
                match Utc.from_local_datetime(&naive).single() {
                    Some(datetime) => Ok(Some(datetime)),
                    None => Ok(None), // Si no se puede convertir, devolver None
                }
            },
            Err(_) => Ok(None), // Si el formato es inválido, devolver None
        }
    } else {
        Ok(None)
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Empresa {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    #[serde(rename = "NUMERO_RUC")]
    pub numero_ruc: String,

    #[serde(rename = "RAZON_SOCIAL")]
    pub razon_social: String,

    #[serde(rename = "CODIGO_JURISDICCION")]
    pub provincia_jurisdiccion: Option<String>,

    #[serde(rename = "ESTADO_CONTRIBUYENTE")]
    pub estado_contribuyente: Option<String>,
    #[serde(rename = "CLASE_CONTRIBUYENTE")]
    pub clase_contribuyente: Option<String>,
    #[serde(
        rename = "FECHA_INICIO_ACTIVIDADES",
        serialize_with = "serialize_datetime",
        deserialize_with = "deserialize_datetime",
        default
    )]
    pub fecha_inicio_actividades: Option<DateTime<Utc>>,

    #[serde(
        rename = "FECHA_ACTUALIZACION",
        serialize_with = "serialize_datetime",
        deserialize_with = "deserialize_datetime",
        default
    )]
    pub fecha_actualizacion: Option<chrono::DateTime<Utc>>,
    
    #[serde(
        rename = "FECHA_SUSPENSION_DEFINITIVA",
        serialize_with = "serialize_datetime",
        deserialize_with = "deserialize_datetime",
        default
    )]
    pub fecha_suspension_definitiva: Option<chrono::DateTime<Utc>>,
    
    #[serde(
        rename = "FECHA_REINICIO_ACTIVIDADES",
        serialize_with = "serialize_datetime",
        deserialize_with = "deserialize_datetime",
        default
    )]
    pub fecha_reinicio_actividades: Option<chrono::DateTime<Utc>>,
    #[serde(rename = "OBLIGADO")]
    pub obligado: Option<String>,
    #[serde(rename = "TIPO_CONTRIBUYENTE")]
    pub tipo_contribuyente: Option<String>,
    #[serde(rename = "NUMERO_ESTABLECIMIENTO")]
    pub numero_establecimiento: i64,
    #[serde(rename = "NOMBRE_FANTASIA_COMERCIAL", default)]
    pub nombre_fantasia_comercial: Option<String>,
    #[serde(rename = "ESTADO_ESTABLECIMIENTO")]
    pub estado_establecimiento: Option<String>,
    #[serde(rename = "DESCRIPCION_PROVINCIA_EST")]
    pub descripcion_provincia_est: String,
    #[serde(rename = "DESCRIPCION_CANTON_EST")]
    pub descripcion_canton_est: String,
    #[serde(rename = "DESCRIPCION_PARROQUIA_EST")]
    pub descripcion_parroquia_est: String,
    #[serde(rename = "CODIGO_CIIU")]
    pub codigo_ciiu: Option<String>,
    #[serde(rename = "ACTIVIDAD_ECONOMICA")]
    pub actividad_economica: Option<String>,
    #[serde(rename = "AGENTE_RETENCION")]
    pub agente_retencion: Option<String>,
    #[serde(rename = "ESPECIAL")]
    pub especial: Option<String>,
    /*
    #[serde(
        rename = "created_at",
        serialize_with = "serialize_datetime",
        deserialize_with = "deserialize_datetime",
        default
    )]
    pub created_at: Option<chrono::DateTime<Utc>>,
    */
}
