# Kubevela Client Rust

## Requirements

You need to install Open API Generator:

```sh
brew install openapi-generator
```

```bash
openapi-generator generate -i ./spec/kubevela.yaml -g rust -o .

# Incase of errors
openapi-generator generate --skip-validate-spec -i ./spec/kubevela.yaml -g rust -o .

```
