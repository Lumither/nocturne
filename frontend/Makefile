
.PHONY:all
all: standalone

.PHONY: standalone
standalone:
	npm run build
	cp -r public .next/standalone/ && cp -r .next/static .next/standalone/.next/
