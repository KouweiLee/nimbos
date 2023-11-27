cd ./kernel && make build ARCH=aarch64 LOG=warn && \
cd .. && cp ./kernel/target/aarch64/release/nimbos.bin ../arceos-hv/apps/hv/guest/nimbos/nimbos-aarch64.bin