RELEASE_DIR:=out
MMDC := ./node_modules/.bin/mmdc
DIAGRAMS := $(patsubst %.mmd,%.svg,$(wildcard specs/diagrams/*.mmd))

AZURE_STORAGE_ACCOUNT := ghpreview
AZURE_STORAGE_CONTAINER := preview


build: specs/index.html
	mkdir -p ${RELEASE_DIR}
	cp -r $^ specs/diagrams ${RELEASE_DIR}/
	cp -r TR ${RELEASE_DIR}/

specs/index.html: specs/index.bs ${DIAGRAMS}
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
		-s ${RELEASE_DIR} --destination-path ileap/$(shell date -u "+%Y%m%dT%H%M%SZ")


.PHONY: serve build azure-upload-preview clean
