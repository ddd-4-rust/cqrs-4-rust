#!/usr/bin/env bash
set -euo pipefail

project_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
java_root="${1:-${project_root}/../cqrs-4-java}"
mapping_file="${project_root}/docs/migration/file_mapping.csv"

if [[ ! -d "${java_root}" ]]; then
    echo "Java baseline not found: ${java_root}" >&2
    exit 2
fi

if [[ ! -f "${mapping_file}" ]]; then
    echo "Migration mapping not found: ${mapping_file}" >&2
    exit 2
fi

java_files="$({
    find "${java_root}" -type f -name '*.java' \
        ! -path '*/.mvn/wrapper/MavenWrapperDownloader.java' \
        ! -path '*/.codegraph/*'
} | wc -l | tr -d ' ')"

rust_files="$(find "${project_root}" -type f -name '*.rs' \
    ! -path '*/target/*' | wc -l | tr -d ' ')"
mapping_entries="$(awk -F ',' 'NR > 1 && NF { count++ } END { print count + 0 }' "${mapping_file}")"
unique_java_paths="$(tail -n +2 "${mapping_file}" | cut -d ',' -f 1 | sort -u | wc -l | tr -d ' ')"
unique_rust_paths="$(tail -n +2 "${mapping_file}" | cut -d ',' -f 2 | sort -u | wc -l | tr -d ' ')"
declared_present="$(awk -F ',' 'NR > 1 && $3 == "present" { count++ } END { print count + 0 }' "${mapping_file}")"
actual_present="$(awk -F ',' 'NR > 1 { print $2 }' "${mapping_file}" | while IFS= read -r rust_path; do
    if [[ -f "${project_root}/${rust_path}" ]]; then
        echo "${rust_path}"
    fi
done | wc -l | tr -d ' ')"
missing_java="$(awk -F ',' 'NR > 1 { print $1 }' "${mapping_file}" | while IFS= read -r java_path; do
    if [[ ! -f "${java_root}/${java_path}" ]]; then
        echo "${java_path}"
    fi
done | wc -l | tr -d ' ')"
rust_infrastructure_files="$((rust_files - actual_present))"

echo "Java migration files: ${java_files}"
echo "Migration mapping entries: ${mapping_entries}"
echo "Rust source files (including infrastructure): ${rust_files}"
echo "Rust mapped files present: ${actual_present}"
echo "Rust infrastructure files: ${rust_infrastructure_files}"

if [[ "${java_files}" -ne 140 ]]; then
    echo "Unexpected Java baseline count; expected 140" >&2
    exit 3
fi

if [[ "${mapping_entries}" -ne "${java_files}" ]] \
    || [[ "${unique_java_paths}" -ne "${java_files}" ]] \
    || [[ "${unique_rust_paths}" -ne "${java_files}" ]] \
    || [[ "${missing_java}" -ne 0 ]]; then
    echo "Migration mapping is not a unique ${java_files}-entry bijection" >&2
    exit 4
fi

if [[ "${declared_present}" -ne "${actual_present}" ]]; then
    echo "Mapping status is stale: declared ${declared_present}, actual ${actual_present}" >&2
    exit 5
fi

if [[ "${actual_present}" -ne "${java_files}" ]]; then
    echo "Migration parity incomplete: ${actual_present}/${java_files} mapped Rust files exist" >&2
    exit 1
fi

echo "Migration file-count parity satisfied."
