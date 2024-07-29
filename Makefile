RELEASE_DIR:=out
MMDC := ./node_modules/.bin/mmdc
DIAGRAMS := $(patsubst specs/%.mmd,${RELEASE_DIR}/%.svg,$(wildcard specs/diagrams/*.mmd))
HTMLS := $(patsubst specs/%.bs,${RELEASE_DIR}/%.html,$(wildcard specs/*.bs))

build: ${HTMLS} ${DIAGRAMS}
	cp -r TR ${RELEASE_DIR}/

${RELEASE_DIR}/%.html: specs/%.bs ${DIAGRAMS} ${RELEASE_DIR}
	bikeshed spec $< $@

serve: ${DIAGRAMS}
	cd specs && bikeshed serve

clean:
	rm -f ${DIAGRAMS}

%.svg: %.mmd ${MMDC} ${RELEASE_DIR}
	${MMDC} -i $< -o $@

${RELEASE_DIR}:
	mkdir -p $@

${MMDC}:
	npm install @mermaid-js/mermaid-cli

.PHONY: serve build
