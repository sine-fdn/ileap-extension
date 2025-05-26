RELEASE_DIR:=out
MMDC := ./node_modules/.bin/mmdc
DIAGRAMS := $(patsubst %.mmd,%.svg,$(wildcard specs/diagrams/*.mmd))

AZURE_STORAGE_ACCOUNT := ghpreview
AZURE_STORAGE_CONTAINER := preview

DATE := $(shell date -u "+%Y%m%d")
HOUR := $(shell date -u "+%H%M%S")

build: specs/index.html specs/faq.html
	mkdir -p ${RELEASE_DIR}
	cp -r $^ specs/diagrams ${RELEASE_DIR}/
	cp -r TR ${RELEASE_DIR}/

specs/index.html: specs/index.bs ${DIAGRAMS}
	bikeshed spec $< $@

specs/faq.html: specs/faq.bs
	bikeshed spec $< $@

serve: ${DIAGRAMS}
	cd specs && bikeshed serve

clean:
	rm -f ${DIAGRAMS}

%.svg: %.mmd ${MMDC}
	${MMDC} -i $< -o $@

${MMDC}:
	npm install @mermaid-js/mermaid-cli

azure-upload-preview: build
	az storage blob upload-batch \
		-d ${AZURE_STORAGE_CONTAINER} \
		--account-name ${AZURE_STORAGE_ACCOUNT} \
		-s ${RELEASE_DIR} --destination-path ileap/${DATE}${HOUR} \

release: clean build
	$(eval YEAR := $(shell date -u "+%Y"))
	$(eval DATE := $(shell date -u "+%Y%m%d"))
	mkdir -p TR/$(YEAR)/ileap-extension-$(DATE)/diagrams
	sed -E -i '' 's|<h2 id="subtitle" class="no-num no-toc no-ref">.*</h2>|<h2 id="subtitle" class="no-num no-toc no-ref">Stable Release</h2>|' specs/header.include
	cp specs/index.html specs/faq.html TR/$(YEAR)/ileap-extension-$(DATE)/
	cp specs/diagrams/*.svg TR/$(YEAR)/ileap-extension-$(DATE)/diagrams/

.PHONY: serve build azure-upload-preview clean
