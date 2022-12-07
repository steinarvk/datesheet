#!/bin/bash
set -euxo pipefail

font_family=Roboto
desired_text="0123456789-%20MonTueWedThuFriSatSun"

script_dir=$( cd -- "$(dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
output_dir=${script_dir}/../fonts
output_filename=${output_dir}/minimal.${font_family}.ttf

temp_filename_prefix=tmp.tiny-font.${font_family}

curl --silent "https://fonts.googleapis.com/css2?family=${font_family}&text=${desired_text}" --output "${temp_filename_prefix}.css"
woff2_url=$(grep "src: url" "${temp_filename_prefix}.css" | grep "format('truetype')" | grep -o -E 'https://[^)]*')

curl --silent "${woff2_url}" --output "${temp_filename_prefix}.ttf"

rm ${temp_filename_prefix}.css

mkdir -p "${output_dir}"

mv ${temp_filename_prefix}.ttf "${output_filename}"
