#!/usr/bin/env bash
# tools/check_migration_parity.sh
#
# Enforce the cqrs-4-rust migration baseline: every Java migration
# source must have exactly one corresponding Rust source, and vice versa.
#
# Usage:
#   ./tools/check_migration_parity.sh                 # auto-detect everything
#   JAVA_BASELINE=/path/to/cqrs-4-java ./tools/...     # explicit Java path
#   SKIP_JAVA=1 ./tools/...                            # skip Java-side checks (CI mode)
#
# Exit codes:
#   0 — parity satisfied
#   1 — parity incomplete (Rust files missing)
#   2 — mapping file missing
#   3 — Java baseline count unexpected
#   4 — mapping is not a 140-entry bijection
#   5 — mapping status stale (declared `present` count != actual on-disk)
set -euo pipefail

project_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Find the mapping CSV regardless of where it lives post-migration.
mapping_file="${MAPPING_FILE:-}"
if [[ -z "${mapping_file}" ]]; then
    for candidate in \
        "${project_root}/docs/superpowers/plans/2026-07-23-cqrs-4-rust-migration-accounting.csv" \
        "${project_root}/docs/migration/file_mapping.csv"; do
        if [[ -f "${candidate}" ]]; then
            mapping_file="${candidate}"
            break
        fi
    done
fi
if [[ -z "${mapping_file}" || ! -f "${mapping_file}" ]]; then
    echo "Migration mapping not found (looked under docs/superpowers/plans/ and docs/migration/)" >&2
    exit 2
fi

# Resolve Java baseline.
skip_java="${SKIP_JAVA:-0}"
java_root="${JAVA_BASELINE:-${project_root}/../cqrs-4-java}"
if [[ ! -d "${java_root}" && "${skip_java}" != "1" ]]; then
    echo "Java baseline not found: ${java_root}" >&2
    echo "Set JAVA_BASELINE=/path/to/cqrs-4-java or SKIP_JAVA=1 for CI mode." >&2
    exit 2
fi

# Count Java migration sources.
java_files=0
if [[ "${skip_java}" == "1" ]]; then
    echo "Java baseline: skipped (SKIP_JAVA=1)"
else
    java_files="$({
        find "${java_root}" -type f -name '*.java' \
            ! -path '*/.mvn/wrapper/MavenWrapperDownloader.java' \
            ! -path '*/.codegraph/*'
    } | wc -l | tr -d ' ')"
    echo "Java migration files: ${java_files}"
fi

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

missing_java=0
if [[ "${skip_java}" != "1" ]]; then
    missing_java="$(awk -F ',' 'NR > 1 { print $1 }' "${mapping_file}" | while IFS= read -r java_path; do
        if [[ ! -f "${java_root}/${java_path}" ]]; then
            echo "${java_path}"
        fi
    done | wc -l | tr -d ' ')"
fi

rust_infrastructure_files=$((rust_files - actual_present))

echo "Migration mapping entries: ${mapping_entries}"
echo "Rust source files (including infrastructure): ${rust_files}"
echo "Rust mapped files present: ${actual_present}"
echo "Rust infrastructure files: ${rust_infrastructure_files}"
if [[ "${skip_java}" != "1" ]]; then
    echo "Missing Java sources (in mapping, not on disk): ${missing_java}"
fi

if [[ "${skip_java}" != "1" ]]; then
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
    expected="${java_files}"
else
    # In CI mode we still require the mapping to be self-consistent even
    # without a Java baseline: entries must be unique in both columns.
    if [[ "${unique_java_paths}" -ne "${mapping_entries}" ]] \
        || [[ "${unique_rust_paths}" -ne "${mapping_entries}" ]]; then
        echo "Migration mapping is not a unique ${mapping_entries}-entry bijection" >&2
        exit 4
    fi
    expected="${mapping_entries}"
fi

if [[ "${declared_present}" -ne "${actual_present}" ]]; then
    echo "Mapping status is stale: declared ${declared_present}, actual ${actual_present}" >&2
    exit 5
fi

if [[ "${actual_present}" -ne "${expected}" ]]; then
    echo "Migration parity incomplete: ${actual_present}/${expected} mapped Rust files exist" >&2
    exit 1
fi

echo "Migration file-count parity satisfied."