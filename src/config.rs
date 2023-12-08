use aws_config::{Region, SdkConfig};
use aws_sdk_ebs::Client as EbsClient;
use aws_sdk_ec2::Client as Ec2Client;
pub struct Config {
    pub(crate) config: SdkConfig,
}

/// Create a config to build an AWS SDK client
pub async fn build_client_config<S1, S2, S3>(
    region: Option<S1>,
    profile: Option<S2>,
    endpoint: Option<S3>,
) -> Config
where
    S1: Into<String>,
    S2: AsRef<str>,
    S3: Into<String>,
{
    let config: aws_config::ConfigLoader = match (region, profile.as_ref()) {
        (Some(region), _) => {
            // Region option passed in
            aws_config::from_env().region(Region::new(region.into()))
        }
        (None, Some(profile)) => {
            // Take region from profile
            aws_config::from_env().region(
                aws_config::profile::ProfileFileRegionProvider::builder()
                    .profile_name(profile.as_ref())
                    .build(),
            )
        }
        (None, None) => {
            // No region or profile passed in, use defaults
            aws_config::from_env()
        }
    };

    let config = if let Some(profile) = profile {
        // Add profile credential provider
        config.credentials_provider(
            aws_config::profile::ProfileFileCredentialsProvider::builder()
                .profile_name(profile.as_ref())
                .build(),
        )
    } else {
        // Keep config unchanged
        config
    };

    let config: aws_config::ConfigLoader = match endpoint {
        Some(endpoint) => config.endpoint_url(endpoint),
        None => {
            // Keep config the same
            config
        }
    };

    Config {
        config: config.load().await,
    }
}

pub struct Client {
    pub(crate) ebs_client: EbsClient,
    pub(crate) ec2_client: Ec2Client,
}

impl Client {
    pub fn new(config: &Config) -> Self {
        Self {
            ebs_client: EbsClient::new(&config.config),
            ec2_client: Ec2Client::new(&config.config),
        }
    }
}
