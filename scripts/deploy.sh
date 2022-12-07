#!/bin/bash
set -euo pipefail

scriptpath="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"

project_name=${CLOUD_RUN_PROJECT}
service_region=${CLOUD_RUN_REGION}
service_name=${CLOUD_RUN_SERVICE}

image_version=$(${scriptpath}/build-and-push.sh | tail -1)

echo "Deploying ${image_version} to ${project_name} in ${service_name} (region ${service_region})"

gcloud run deploy "${service_name}" --project "${project_name}" --image "${image_version}" --region=${service_region}
