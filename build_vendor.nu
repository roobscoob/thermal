def main [--refetch] {
    if (not ([ ./vendor/cache ] | path exists | first) or $refetch) {
        rm -rf ./vendor/cache
        cargo run --package escpos_build -- fetch ./vendor/cache
    }

    cargo run --package escpos_build -- build ./vendor/cache --out ./vendor/spec/escpos-commands.json --labels ./vendor/labels.json --labels ./vendor/content.json
}