RELEASE_DIR:=out
DIAGRAMS_DIR:=specs/diagrams
DIAGRAMS:=$(patsubst $(DIAGRAMS_DIR)/%.mmd,$(DIAGRAMS_DIR)/%.svg,$(wildcard $(DIAGRAMS_DIR)/*.mmd))

build: specs/index.html $(DIAGRAMS)
	mkdir -p ${RELEASE_DIR}
	cp -r $< specs/diagrams ${RELEASE_DIR}/
	cp -r TR ${RELEASE_DIR}/

specs/index.html: specs/index.bs
	bikeshed spec $< $@

$(DIAGRAMS_DIR)/%.svg: $(DIAGRAMS_DIR)/%.mmd | mermaid
	mmdc -i $< -o $@

serve:
	cd specs && bikeshed serve

mermaid:
	npm install -g @mermaid-js/mermaid-cli

.PHONY: serve build
