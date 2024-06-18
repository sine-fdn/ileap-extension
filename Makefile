RELEASE_DIR:=out
MMDC := ./node_modules/.bin/mmdc
DIAGRAMS := $(patsubst %.mmd,%.svg,$(wildcard specs/diagrams/*.mmd))

build: specs/index.html
	mkdir -p ${RELEASE_DIR}
	cp -r $< specs/diagrams ${RELEASE_DIR}/
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

.PHONY: serve build
