import nextra from "nextra";

// Set up Nextra with its configuration
const withNextra = nextra({
	// ... Add Nextra-specific options here
});

// Export the final Next.js config with Nextra included
export default withNextra({
	// ... Add regular Next.js options here
	turbopack: {
		//  fix for https://nextra.site/docs/file-conventions/mdx-components-file#module-not-found-cant-resolve-next-mdx-import-source-file
		resolveAlias: {
			"next-mdx-import-source-file": "./mdx-components.tsx",
		},
	},
});
