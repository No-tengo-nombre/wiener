# Declare crates to descend in the dependency tree
CRATES=(
    wiener_core
    wiener_utils
    wiener_gl
    wiener_vk
    wiener_internal
)

# Determine the arguments
dry_run=0

while getopts "d" arg
do
    case "$arg" in
        d)
            dry_run=1
            ;;
    esac
done

# Publish every crate
cd crates
for crate in "${CRATES[@]}"; do
    echo "Publishing crate '${crate}'"
    cd $crate
    if [ $dry_run -eq 0 ]; then
        cargo publish
    else
        cargo publish --dry-run
    fi
    cd ..
done

# Publish the root crate
cd ..
echo "Publishing crate 'wiener'"
if [ $dry_run -eq 0 ]; then
    cargo publish
else
    cargo publish --dry-run
fi
