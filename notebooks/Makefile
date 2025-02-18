PIP_ROOT = .envrc/

.PHONY: help
help: ## This outputs help information.
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

requirements: ## generates requirements.txt file from requirements.in
	pip-compile --generate-hashes requirements.in
	# pip-compile --resolver=backtracking --generate-hashes requirements.in
	
preinstall:
	pip install pip-tools

install_local: requirements ## installs requirements into current virtualenv.
	pip install -r requirements.txt

install: preinstall requirements install_local ## install local dependencies to virtualenv.

notebook:  ## Start notebook
	jupyter lab --ip=127.0.0.1 --port=8090

up: notebook ## alias for notebook
