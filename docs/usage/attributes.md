---
nav_order: 2
parent: Usage
---

# Metadata attributes

Afterburn can consume cloud-specific metadata and serialize instance attributes into an environment file (e.g. `/run/metadata/afterburn`), which can then be consumed by systemd service units via `EnvironmentFile=`.

Usually, OS vendors which ship Afterburn do not enable it by default. Therefore, any service
which wants to make use of Afterburn metadata must explicitly pull it in using e.g.
`Requires=afterburn.service` and `After=afterburn.service`.

Cloud providers with supported metadata endpoints and their respective attributes are listed below.

* aliyun
  - BACKBURN_ALIYUN_EIPV4
  - BACKBURN_ALIYUN_HOSTNAME
  - BACKBURN_ALIYUN_IMAGE_ID
  - BACKBURN_ALIYUN_INSTANCE_ID
  - BACKBURN_ALIYUN_INSTANCE_TYPE
  - BACKBURN_ALIYUN_IPV4_PRIVATE
  - BACKBURN_ALIYUN_IPV4_PUBLIC
  - BACKBURN_ALIYUN_REGION_ID
  - BACKBURN_ALIYUN_VPC_ID
  - BACKBURN_ALIYUN_ZONE_ID
* aws
  - BACKBURN_AWS_HOSTNAME
  - BACKBURN_AWS_PUBLIC_HOSTNAME
  - BACKBURN_AWS_IPV4_LOCAL
  - BACKBURN_AWS_IPV4_PUBLIC
  - BACKBURN_AWS_AVAILABILITY_ZONE
  - BACKBURN_AWS_INSTANCE_ID
  - BACKBURN_AWS_INSTANCE_TYPE
  - BACKBURN_AWS_REGION
* azure
  - BACKBURN_AZURE_IPV4_DYNAMIC
  - BACKBURN_AZURE_IPV4_VIRTUAL
  - BACKBURN_AZURE_VMSIZE

* openstack
  - BACKBURN_OPENSTACK_HOSTNAME
  - BACKBURN_OPENSTACK_IPV4_LOCAL
  - BACKBURN_OPENSTACK_IPV4_PUBLIC
  - BACKBURN_OPENSTACK_INSTANCE_ID
  - BACKBURN_OPENSTACK_INSTANCE_TYPE
* openstack-metadata
  - BACKBURN_OPENSTACK_HOSTNAME
  - BACKBURN_OPENSTACK_IPV4_LOCAL
  - BACKBURN_OPENSTACK_IPV4_PUBLIC
  - BACKBURN_OPENSTACK_INSTANCE_ID
  - BACKBURN_OPENSTACK_INSTANCE_TYPE


Additionally, some attribute names are reserved for custom metadata providers.
These can be safely used by external providers on platforms not supported by Afterburn:

* custom
  - BACKBURN_CUSTOM_HOSTNAME
  - BACKBURN_CUSTOM_PUBLIC_IPV4
  - BACKBURN_CUSTOM_PRIVATE_IPV4
  - BACKBURN_CUSTOM_PUBLIC_IPV6
  - BACKBURN_CUSTOM_PRIVATE_IPV6
