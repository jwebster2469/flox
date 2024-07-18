use anyhow::Result;
use bpaf::Bpaf;
use flox_rust_sdk::flox::Flox;
use flox_rust_sdk::providers::services::ServiceError;
use tracing::instrument;

mod stop;

/// Services Commands.
#[derive(Debug, Clone, Bpaf)]
pub enum ServicesCommands {
    /// Stop a service or services
    #[bpaf(command)]
    Stop(#[bpaf(external(stop::stop))] stop::Stop),
}

impl ServicesCommands {
    #[instrument(name = "services", skip_all)]
    pub async fn handle(self, flox: Flox) -> Result<()> {
        if !flox.features.services {
            return Err(ServiceError::FeatureFlagDisabled.into());
        }

        match self {
            ServicesCommands::Stop(args) => args.handle(flox).await?,
        }

        Ok(())
    }
}