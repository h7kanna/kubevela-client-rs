NAME := "controller"
ORG := "kube-rs"
VERSION := `git rev-parse HEAD`
SEMVER_VERSION := `grep version Cargo.toml | awk -F"\"" '{print $2}' | head -n 1`

default:
  @just --list --unsorted --color=always | rg -v "    default"

yq:
  @yq -P '.' spec/kubevela.json > spec/kubevela.yaml

generate:
  @openapi-generator generate -i spec/kubevela.yaml -g rust -o .