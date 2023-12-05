# Kubevela Client Rust

## Requirements

You need to install Open API Generator:

```sh
brew install openapi-generator
```

```bash
openapi-generator generate --package-name kubevela --input-spec ./spec/kubevela.json --generator-name rust --output .

# Incase of errors
openapi-generator generate \
--skip-validate-spec \
--package-name kubevela \
--input-spec ./spec/kubevela.json \
--generator-name rust \
--output .

# Differences 
openapi-generator generate \
--minimal-update \
--skip-validate-spec \
--package-name kubevela \
--input-spec ./spec/kubevela.json \
--generator-name rust \
--output .

```
