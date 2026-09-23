#!/bin/bash

# Builds the engine in several configurations, each in its own directory, so a
# game can pick the one that fits what it is doing:
#
#   tracy -> build-{arch}-linux-tracy/install
#            Optimized (RELWITHDEBINFO) with on-demand Tracy. For playing and
#            profiling; without a connected Tracy client it runs like a normal
#            build.
#   asan  -> build-{arch}-linux-asan/install
#            AddressSanitizer, no Tracy, no mimalloc (so ASan sees every C++
#            allocation). For debugging memory errors; do not profile it.
#
# The variants share the ccache, and each is an ordinary build.sh build, so a
# single variant can also be rebuilt later with
#   docker-build-v2/build.sh --build-dir build-amd64-linux-asan --compile linux

set -e -u -o pipefail

USAGE="Usage: $0 [-h|--help] [--parallel] [-j|--jobs {number_of_jobs}] [--arch {arm64|amd64}] [--compile] [variant...] [-- cmake_flag...]"

VARIANTS_HELP="Engine builds (point your game/launcher at the install directory):

  default  build-{arch}-linux/install        (docker-build-v2/build.sh linux)
           Optimized, no Tracy. Normal play, tests, CI-like runs.
  tracy    build-{arch}-linux-tracy/install  (this script)
           Same optimization plus on-demand Tracy. Use when profiling; without a
           connected Tracy client it behaves like the default build.
  asan     build-{arch}-linux-asan/install   (this script)
           AddressSanitizer, no Tracy, no mimalloc. Use when debugging crashes or
           memory corruption; several times slower, never use it for timing.
           Leak reports at exit are on by default; ASAN_OPTIONS=detect_leaks=0
           disables them, LSAN_OPTIONS=suppressions=<file> filters driver noise.

Variants built by this script: tracy, asan (default: both)."

declare -A VARIANT_FLAGS=(
	[tracy]="-DTRACY_ENABLE=ON -DTRACY_ON_DEMAND=ON"
	[asan]="-DUSE_ASAN=ON -DUSE_MIMALLOC=OFF -DTRACY_ENABLE=OFF"
)

case $(uname -m) in
	x86_64) ARCH=amd64 ;;
	aarch64) ARCH=arm64 ;;
	*) ARCH=unknown ;;
esac

PARALLEL=false
JOBS=
PHASE=()
VARIANTS=()
EXTRA_FLAGS=()

while (( $# > 0 )); do
	case $1 in
		-h|--help)
			echo "$USAGE"
			echo ""
			echo "$VARIANTS_HELP"
			echo ""
			echo "Options:"
			echo "  --parallel  build the variants at the same time, splitting the jobs"
			echo "  -j, --jobs  total number of concurrent compile jobs"
			echo "  --arch      arm64 or amd64, defaults to host"
			echo "  --compile   only compile, reuse each variant's existing configuration"
			echo "  -- flags    extra cmake flags passed to every variant's configure"
			exit 0
			;;
		--parallel) PARALLEL=true; shift ;;
		--compile) PHASE=(--compile); shift ;;
		--arch) ARCH="$2"; shift 2 ;;
		-j|--jobs)
			if ! [[ "${2-}" =~ ^[1-9][0-9]*$ ]]; then
				echo "--jobs requires a number"
				exit 1
			fi
			JOBS="$2"
			shift 2
			;;
		--) shift; EXTRA_FLAGS=("$@"); break ;;
		*)
			if [[ -z "${VARIANT_FLAGS[$1]+set}" ]]; then
				echo "Unknown variant or option: $1"
				echo ""
				echo "$USAGE"
				exit 1
			fi
			VARIANTS+=("$1")
			shift
			;;
	esac
done

if (( ${#VARIANTS[@]} == 0 )); then
	VARIANTS=(tracy asan)
fi

cd "$(dirname "$(readlink -f "$0")")/.."

TOTAL_JOBS="${JOBS:-$(nproc)}"
if $PARALLEL; then
	PER_VARIANT_JOBS=$(( TOTAL_JOBS / ${#VARIANTS[@]} ))
	PER_VARIANT_JOBS=$(( PER_VARIANT_JOBS > 0 ? PER_VARIANT_JOBS : 1 ))
else
	PER_VARIANT_JOBS="$TOTAL_JOBS"
fi

build_variant() {
	local variant="$1"
	local dir="build-$ARCH-linux-$variant"
	local flags=()
	# --compile forwards trailing args to `cmake --build`, so configure flags
	# only go along when configuring
	if (( ${#PHASE[@]} == 0 )); then
		read -r -a flags <<< "${VARIANT_FLAGS[$variant]}"
		flags+=("${EXTRA_FLAGS[@]}")
	fi

	echo "=== [$variant] building in $dir (jobs: $PER_VARIANT_JOBS)"
	# Rust crate checks do not depend on the variant; run them once, up front
	SKIP_RUST_CHECK=1 docker-build-v2/build.sh "${PHASE[@]}" -j "$PER_VARIANT_JOBS" --arch "$ARCH" \
		--build-dir "$dir" linux "${flags[@]}"
	echo "=== [$variant] done: $dir/install"
}

if [[ -z "${SKIP_RUST_CHECK:-}" ]] && command -v cargo >/dev/null; then
	./docker-build-v2/check-rust.sh
fi

if $PARALLEL; then
	mkdir -p .cache/build-variants
	declare -A PIDS=()
	for variant in "${VARIANTS[@]}"; do
		log=".cache/build-variants/$variant.log"
		build_variant "$variant" > "$log" 2>&1 &
		PIDS[$variant]=$!
		echo "=== [$variant] started, log: $log"
	done

	failed=()
	for variant in "${VARIANTS[@]}"; do
		if wait "${PIDS[$variant]}"; then
			echo "=== [$variant] done: build-$ARCH-linux-$variant/install"
		else
			failed+=("$variant")
			echo "=== [$variant] FAILED, see .cache/build-variants/$variant.log"
		fi
	done
	(( ${#failed[@]} == 0 )) || exit 1
else
	for variant in "${VARIANTS[@]}"; do
		build_variant "$variant"
	done
fi
