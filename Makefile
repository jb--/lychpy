# initial setup for a makefile
# important bit is only to use the build container for publishing

publish:
	docker run --rm -v $(pwd):/io maturinbuild publish --username __token__ --password $(PYPI_TOKEN)

docker:
	docker build -t maturinbuild -f Dockerfile.build .