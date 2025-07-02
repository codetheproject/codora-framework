//! Configuration parsing utilities
//!
use std::borrow::Cow;

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub enum Error {
    #[error("Parse Error: {0}")]
    ParseError(Cow<'static, str>),
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[cfg(test)]
mod tests {
    #![cfg_attr(debug_assertions, allow(warnings))]
    use super::*;
    use anyhow::Result;
    use codora_framework_proc_macro::Conf;

    #[derive(Debug, Conf)]
    #[conf(prefix = "APP_", json)]
    struct Conf {
        port: u16,
    }

    #[derive(Debug, Conf)]
    #[conf(file = ".env")]
    struct Env {
        database_url: String,
    }

    // assumption
    #[derive(new)]
    struct ParserContext {}

    impl ParserContext {
        fn parse<T>(&self) -> Result<T> {
            todo!()
        }
    }
    #[test]
    fn test_parsing() -> Result<()> {
        // given
        let json_string = r#"
{
    "port": "3000",        
}
        "#;
        let env_string = r#"
database_url=postgres://john@doe
        "#;

        // Parser context is how you pass in option to the source
        let parser_context = ParserContext::new();
        let conf = parser_context.parse::<Conf>()?;
        let env = parser_context.parse::<Env>()?;

        assert_eq!(conf.port, 3000);
        assert_eq!(env.database_url, "postgres://john@doe");
        Ok(())
    }

    #[derive(Debug, Conf)]
    #[conf(prefix = "APP_", json, yaml, toml)]
    struct ConfEnv {
        port: u16,
        // it can contains multiple files from .env to yaml to json
        env: Env,
    }
    fn test_conf_with_multiple_sources() -> Result<()> {
        let parser_context = ParserContext::new();
        let conf_env = parser_context.parse::<ConfEnv>()?;

        assert_eq!(conf_env.port, 3000);
        assert_eq!(conf_env.env.database_url, "postgres://john@doe");
        Ok(())
    }

    // We wanna see how we could benefit from serde features like rename, validator and other's
}
