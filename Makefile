.PHONY: release site serve-site

# Run the automated release workflow
# See tools/release.sh for details
release:
	@./tools/release.sh

site:
	@./tools/build-site.sh

serve-site: site
	devd -ol ./_site/
