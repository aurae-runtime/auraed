# ---------------------------------------------------------------------------- #
#             Apache 2.0 License Copyright © 2022 The Aurae Authors            #
#                                                                              #
#                ┏--------------------------------------------┓                #
#                ┃   █████╗ ██╗   ██╗██████╗  █████╗ ███████╗ ┃                #
#                ┃  ██╔══██╗██║   ██║██╔══██╗██╔══██╗██╔════╝ ┃                #
#                ┃  ███████║██║   ██║██████╔╝███████║█████╗   ┃                #
#                ┃  ██╔══██║██║   ██║██╔══██╗██╔══██║██╔══╝   ┃                #
#                ┃  ██║  ██║╚██████╔╝██║  ██║██║  ██║███████╗ ┃                #
#                ┃  ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝ ┃                #
#                ┗--------------------------------------------┛                #
#                                                                              #
#                         Distributed Systems Runtime                          #
#                                                                              #
# ---------------------------------------------------------------------------- #
#                                                                              #
#   Licensed under the Apache License, Version 2.0 (the "License");            #
#   you may not use this file except in compliance with the License.           #
#   You may obtain a copy of the License at                                    #
#                                                                              #
#       http://www.apache.org/licenses/LICENSE-2.0                             #
#                                                                              #
#   Unless required by applicable law or agreed to in writing, software        #
#   distributed under the License is distributed on an "AS IS" BASIS,          #
#   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.   #
#   See the License for the specific language governing permissions and        #
#   limitations under the License.                                             #                                                                             #
#                                                                              #
# ---------------------------------------------------------------------------- #

all: compile

executable   ?=  auraed

compile: ## Compile for the local architecture ⚙
	@cargo build --release

install: ## Install the program to /usr/bin 🎉
	@echo "Installing..."
	@cargo install --path .

#test: clean compile install ## 🤓 Run go tests
#	@echo "Testing..."
#	go test -v ./...

clean: ## Clean your artifacts 🧼
	@echo "Cleaning..."
	@cargo clean
	@rm -rvf target/*
	@rm -rvf $(executable)

.PHONY: help
help:  ## 🤔 Show help messages for make targets
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "[32m%-30s[0m %s", $$1, $$2}'
