use crate::source::Source;

#[derive(Debug)]
pub struct Enviroment {}

impl Enviroment {
    pub fn new() -> Self {
        Self {}
    }
}

impl Source for Enviroment {}

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub enum EnviromentError {}

// Change the error but this is what we wanna have
pub fn from_env<T>() -> Result<T, EnviromentError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize)]
    struct CodoraConf {
        port: u16,
    }

    #[derive(Debug, Deserialize)]
    struct ConfEnv {
        database_url: String,
        env: CodoraConf,
    }

    // Solutions
    //  Follow Deserializer and Each source are Deserializer to map multiple source
    // we will have an adapter that consume all source into a map then T could just visit map and work with it

    #[test]
    fn test_from_env() -> Result<(), EnviromentError> {
        let codora_conf = from_env::<CodoraConf>()?;

        assert_eq!(codora_conf.port, 3000);
        Ok(())
    }

    #[test]
    fn test_from_env_with_nested_fields() -> Result<(), EnviromentError> {
        let conf_env = from_env::<ConfEnv>()?;

        assert_eq!(conf_env.database_url, String::from("postgres://john@doe"));
        assert_eq!(conf_env.env.port, 3000);
        Ok(())
    }
}
