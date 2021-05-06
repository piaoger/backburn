use crate::providers::aws;
use crate::providers::MetadataProvider;
use anyhow::Context;
use mockito;


#[test]
fn test_aws_attributes() {
    let instance_id = "test-instance-id";
    let instance_type = "test-instance-type";
    let ipv4_local = "test-ipv4-local";
    let ipv4_public = "test-ipv4-public";
    let availability_zone = "test-availability-zone";
    let hostname = "test-hostname";
    let public_hostname = "test-public-hostname";
    let instance_id_doc = r#"{"region": "test-region"}"#;
    let region = "test-region";

    let endpoints = maplit::btreemap! {
        "/meta-data/instance-id" => instance_id,
        "/meta-data/instance-type" => instance_type,
        "/meta-data/local-ipv4" => ipv4_local,
        "/meta-data/public-ipv4" => ipv4_public,
        "/meta-data/placement/availability-zone" => availability_zone,
        "/meta-data/hostname" => hostname,
        "/meta-data/public-hostname" => public_hostname,
        "/dynamic/instance-identity/document" => instance_id_doc,
    };

    let mut mocks = Vec::with_capacity(endpoints.len());
    for (endpoint, body) in endpoints {
        let m = mockito::mock("GET", endpoint)
            .with_status(200)
            .with_body(body)
            .create();
        mocks.push(m);
    }

    let attributes = maplit::hashmap! {
        "AWS_INSTANCE_ID".to_string() => instance_id.to_string(),
        "AWS_INSTANCE_TYPE".to_string() => instance_type.to_string(),
        "AWS_IPV4_LOCAL".to_string() => ipv4_local.to_string(),
        "AWS_IPV4_PUBLIC".to_string() => ipv4_public.to_string(),
        "AWS_AVAILABILITY_ZONE".to_string() => availability_zone.to_string(),
        "AWS_HOSTNAME".to_string() => hostname.to_string(),
        "AWS_PUBLIC_HOSTNAME".to_string() => public_hostname.to_string(),
        "AWS_REGION".to_string() => region.to_string(),
    };

    let client = crate::retry::Client::try_new()
        .context("failed to create http client")
        .unwrap()
        .max_retries(0)
        .return_on_404(true);
    let provider = aws::AwsProvider { client };

    let v = provider.attributes().unwrap();
    assert_eq!(v, attributes);

    mockito::reset();
    provider.attributes().unwrap_err();
}

#[test]
fn test_aws_imds_versions() {
    let instance_id = "test-instance-id";
    let instance_type = "test-instance-type";
    let ipv4_local = "test-ipv4-local";
    let ipv4_public = "test-ipv4-public";
    let availability_zone = "test-availability-zone";
    let hostname = "test-hostname";
    let public_hostname = "test-public-hostname";
    let instance_id_doc = r#"{"region": "test-region"}"#;
    let region = "test-region";

    let attributes = maplit::hashmap! {
        "AWS_INSTANCE_ID".to_string() => instance_id.to_string(),
        "AWS_INSTANCE_TYPE".to_string() => instance_type.to_string(),
        "AWS_IPV4_LOCAL".to_string() => ipv4_local.to_string(),
        "AWS_IPV4_PUBLIC".to_string() => ipv4_public.to_string(),
        "AWS_AVAILABILITY_ZONE".to_string() => availability_zone.to_string(),
        "AWS_HOSTNAME".to_string() => hostname.to_string(),
        "AWS_PUBLIC_HOSTNAME".to_string() => public_hostname.to_string(),
        "AWS_REGION".to_string() => region.to_string(),
    };

    let endpoints = maplit::btreemap! {
        "/meta-data/instance-id" => instance_id,
        "/meta-data/instance-type" => instance_type,
        "/meta-data/local-ipv4" => ipv4_local,
        "/meta-data/public-ipv4" => ipv4_public,
        "/meta-data/placement/availability-zone" => availability_zone,
        "/meta-data/hostname" => hostname,
        "/meta-data/public-hostname" => public_hostname,
        "/dynamic/instance-identity/document" => instance_id_doc,
    };

    let client = crate::retry::Client::try_new()
        .context("failed to create http client")
        .unwrap()
        .max_retries(0)
        .return_on_404(true);

    // first test imdsv1
    {
        let mut mocks = Vec::with_capacity(endpoints.len());
        for (endpoint, body) in endpoints.clone() {
            let m = mockito::mock("GET", endpoint)
                .with_status(200)
                .with_body(body)
                .create();
            mocks.push(m);
        }

        let _m = mockito::mock("PUT", "/api/token")
            .match_header("X-aws-ec2-metadata-token-ttl-seconds", "21600")
            .with_status(403)
            .with_body("Forbidden")
            .create();

        let provider = aws::AwsProvider::with_client(client.clone()).unwrap();

        let v = provider.attributes().unwrap();
        assert_eq!(v, attributes);

        mockito::reset();
        provider.attributes().unwrap_err();
    }

    {
        // then test imdsv2
        let token = "test-api-token";
        let mut mocks = Vec::with_capacity(endpoints.len());
        for (endpoint, body) in endpoints.clone() {
            let m = mockito::mock("GET", endpoint)
                .match_header("X-aws-ec2-metadata-token", token)
                .with_status(200)
                .with_body(body)
                .create();
            mocks.push(m);
        }

        let _m = mockito::mock("PUT", "/api/token")
            .match_header("X-aws-ec2-metadata-token-ttl-seconds", "21600")
            .with_status(200)
            .with_body(token)
            .create();

        let provider = aws::AwsProvider::with_client(client.clone()).unwrap();

        let v = provider.attributes().unwrap();
        assert_eq!(v, attributes);

        mockito::reset();
        provider.attributes().unwrap_err();
    }
}
