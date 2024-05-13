RELEASE_DIR:=out

build: specs/index.html
	mkdir -p ${RELEASE_DIR}
	cp -r $< specs/diagrams ${RELEASE_DIR}/
	cp -r TR ${RELEASE_DIR}/

specs/index.html: specs/index.bs
	bikeshed spec $< $@

serve:
	cd specs && bikeshed serve

.PHONY: serve release