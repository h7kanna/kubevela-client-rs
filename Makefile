yq:
	@yq -P '.' spec/kubevela.json > spec/kubevela.yaml

generate:
	@openapi-generator generate -i spec/kubevela.yaml -g rust -o kubevela
