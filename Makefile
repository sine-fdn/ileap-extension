all: specs/index.html

specs/index.html: specs/index.bs
	bikeshed spec $< $@

serve:
	cd specs && bikeshed serve

.PHONY: serve